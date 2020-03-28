use std::process;
use std::process::{Command, Stdio};
use clap::{App, ArgMatches, SubCommand, Arg};
use crate::{util, colortext};

pub const NAME: &str = "test";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Run tests for <TASK_NAME>")
        .arg(Arg::with_name("TASK_NAME")
            .required(true)
            .index(1))
}

pub fn run(matches: &ArgMatches) {
    let task_name = matches.value_of("TASK_NAME").unwrap();
    let config = util::load_config().test;
    if let Some(compiler) = config.compiler {
        if config.compile_arg.is_none() {
            util::print_error("\"compile_arg\" in config.toml is not defined");
            process::exit(1);
        }
        let arg = config.compile_arg.unwrap();
        let arg = arg.replace("<TASK>", task_name);
        let args = arg.split(" ");
        let output = Command::new(compiler)
            .args(args)
            .output()
            .expect("failed to execute process");
        let status = output.status;
        if !status.success() {
            let output = String::from_utf8(output.stderr).unwrap();
            util::print_error("failed to compile");
            println!("{}\n\nresult: {}", output, colortext::CE);
            process::exit(1);
        }
        println!("compile is finished!\n");
    }

    let input = Command::new("echo")
        .args(&["-e", "-n"])
        .arg("testcase input")
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    let input = input.stdout.unwrap();
    let mut command = Command::new(config.command);
    if let Some(arg) = config.command_arg {
        let arg = arg.replace("<TASK>", task_name);
        let args = arg.split(" ");
        command.args(args);
    }
    let output = command.stdin(input)
        .output()
        .expect("failed to execute process");

}
