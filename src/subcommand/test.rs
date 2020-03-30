use std::{env, time, process, thread, fs};
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::cmp::Ordering;
use clap::{App, ArgMatches, SubCommand, Arg};
use easy_scraper::Pattern;
use serde::{Deserialize, Serialize};
use reqwest::blocking;
use crate::{util, colortext};
use crate::config::Test;

pub const NAME: &str = "test";
const URL_TEMPLATE: &str = "https://atcoder.jp/contests/<CONTEST>/tasks/<CONTEST>_<TASK>";
const TESTCASE_PATTERN: &str = r#"<span class="lang-ja"><h3></h3><pre>{{io}}</pre></span>"#;


#[derive(Copy, Clone, Eq)]
enum Status {
    AC  = 0,
    TLE = 1,
    RE  = 2,
    WA  = 3,
    CE  = 4,
}

impl Ord for Status {
    fn cmp(&self, other: &Status) -> Ordering {
        (*self as i32).cmp(&(*other as i32))
    }
}

impl PartialOrd for Status {
    fn partial_cmp(&self, other: &Status) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Status) -> bool {
        *self as i32 == *other as i32
    }
}

impl Status {
    fn to_string(&self) -> String {
        match self {
            Status::AC  => colortext::AC.to_string(),
            Status::TLE => colortext::TLE.to_string(),
            Status::RE  => colortext::RE.to_string(),
            Status::WA  => colortext::WA.to_string(),
            Status::CE  => colortext::CE.to_string(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct TestcaseFile {
    testcases: Vec<Testcase>,
}

impl TestcaseFile {
    fn new(testcases: Vec<Testcase>) -> TestcaseFile{
        TestcaseFile {
            testcases: testcases
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Testcase {
    input: String,
    output: String
}

impl Testcase {
    fn new<S: Into<String>, T: Into<String>>(input: S, output: T) -> Testcase{
        let input = input.into();
        let output = output.into();
        Testcase {
            input: input,
            output: output
        }
    }
}

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Run tests for <TASK_NAME>")
        .arg(Arg::with_name("TASK_NAME")
            .required(true)
            .index(1))
}

fn compile(config: &Test, task_name: &str) {
    let compiler = config.compiler.as_ref().unwrap();
    if config.compile_arg.is_none() {
        util::print_error("\"compile_arg\" in config.toml is not defined");
        process::exit(1);
    }
    println!("{}: starting compile", colortext::INFO);
    let arg = config.compile_arg.as_ref().unwrap();
    let arg = arg.replace("<TASK>", task_name);
    let args = arg.split(" ");
    let output = Command::new(compiler)
        .args(args)
        .output()
        .expect("failed to execute process");
    let status = output.status;
    if !status.success() {
        let output = String::from_utf8_lossy(&output.stderr);
        util::print_error("failed to compile");
        println!("{}\n\nresult: {}", output, colortext::CE);
        process::exit(1);
    }
    println!("{}: compiled successfully\n", colortext::INFO);
}

fn execute(config: &Test, task_name: &str, testcase_input: &str, tle_time: u16) -> (bool, Option<String>) {
    let input = Command::new("echo")
        .args(&["-e", "-n"])
        .arg(testcase_input)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    let input = input.stdout.unwrap();
    let command_name = config.command.replace("<TASK>", task_name);
    let mut command = Command::new(command_name);
    if let Some(arg) = config.command_arg.as_ref() {
        let arg = arg.replace("<TASK>", task_name);
        let args = arg.split(" ");
        command.args(args);
    }
    let mut command_child = command.stdin(input).stdout(Stdio::piped()).spawn().unwrap();
    let start = time::Instant::now();
    loop {
        match command_child.try_wait() {
            Ok(Some(status)) => {
                if !status.success() {
                    return (true, None)
                }
                let output = command_child.stdout.unwrap();
                let output = Command::new("cat").stdin(output).output().unwrap();
                return (false, Some(String::from_utf8_lossy(&output.stdout).to_string()))
            },
            Ok(None) => {
                let duration = start.elapsed().as_millis();
                if duration > tle_time.into() {
                    let _ = command_child.kill().expect("command wasn't running");
                    return (false, None)
                }
            },
            Err(_e) => {
                util::print_error("command is not available");
                process::exit(1);
            }
        }
        thread::yield_now();
    }
}

pub fn get_testcases<S: Into<String>, T: Into<String>>(contest_name: S, task_name: T) -> (Vec<String>, Vec<String>) {
    let contest_name = contest_name.into();
    let task_name = task_name.into();
    let mut path = env::current_dir().unwrap();
    path.push("testcase");
    path.push([&task_name, "toml"].join("."));
    let testcase_path = path.as_path();

    // すでにテストケースがあるならそれを返す
    if testcase_path.exists() {
        let testcase_path = testcase_path.to_str().unwrap();
        let content = util::read_file(testcase_path);
        let file: TestcaseFile = toml::from_str(&content).unwrap_or_else(|_|{
            util::print_error("testcase file is wrong");
            process::exit(1);
        });
        let inputs = file.testcases.iter().map(|x| x.input.clone()).collect();
        let outputs = file.testcases.iter().map(|x| x.output.clone()).collect();
        return (inputs, outputs)
    }

    // テストケースをAtCoderから取得
    let url = URL_TEMPLATE.to_string();
    let url = url.replace("<CONTEST>", &contest_name);
    let url = url.replace("<TASK>", &task_name.to_lowercase());
    let document = blocking::get(&url).unwrap_or_else(|_| {
        util::print_error(format!("{} is wrong", url));
        process::exit(1);
    });
    let document = document.text().unwrap();
    let pattern = Pattern::new(TESTCASE_PATTERN).unwrap();
    let io_cases = pattern.matches(&document);
    if io_cases.len() % 2 != 0 {
        util::print_error("The correct test case could not be get");
        process::exit(1);
    }
    let inputs: Vec<String> = io_cases.iter().step_by(2).map(|x| x["io"].clone()).collect();
    let outputs: Vec<String> = io_cases.iter().skip(1).step_by(2).map(|x| x["io"].clone()).collect();

    // テストケースファイルの作成
    let mut testcases = Vec::<Testcase>::new();
    for (input, output) in inputs.iter().zip(outputs.iter()) {
        testcases.push(Testcase::new(input, output));
    }
    let content = toml::to_string(&TestcaseFile::new(testcases)).unwrap();
    let testcase_path = testcase_path.to_str().unwrap();
    let mut file = File::create(testcase_path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    (inputs, outputs)
}

pub fn run(matches: &ArgMatches) {
    let task_name = matches.value_of("TASK_NAME").unwrap();
    let config = util::load_config(true);
    let contest_name = config.contest.unwrap_or_else( ||{
        util::print_error("contest_name in local config.toml is not defined");
        process::exit(1);
    });
    let config = config.test;

    if config.compiler.is_some() {
        compile(&config, task_name);
    }

    let mut all_result = Status::AC;
    let mut count = 0;
    let (inputs, outputs) = get_testcases(contest_name, task_name);
    println!("{}: starting test ...", colortext::INFO);
    for (input, output) in inputs.iter().zip(outputs.iter()) {
        count += 1;
        print!("- testcase {} ... ", count);

        let tle_time = config.tle_time.unwrap_or(3000);
        let (caused_runtime_error, result) = execute(&config, task_name, input, tle_time);
        if caused_runtime_error {
            all_result = all_result.max(Status::RE);
            println!("{}\n", colortext::RE);
            continue;
        }
        if result.is_none() {
            all_result = all_result.max(Status::TLE);
            println!("{}", colortext::TLE);
            continue;
        }
        let result = util::remove_last_indent(result.unwrap());
        let output = util::remove_last_indent(output);
        let is_correct = result == output;
        let status = if is_correct {
            colortext::AC
        } else {
            all_result = all_result.max(Status::WA);
            colortext::WA
        };
        println!("{}", status);
    }
    println!("result: {}", all_result.to_string());
}
