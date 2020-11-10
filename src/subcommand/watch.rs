use crate::subcommand::test;
use crate::util;
use crate::colortext;
use chrono::Local;
use clap::{App, Arg, ArgMatches, SubCommand};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use termion::raw::IntoRawMode;
use termion::event::{Event, Key, parse_event};
use termion::async_stdin;
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::Duration;
use std::{env, process};
use std::io::{Write, Read, stdout};

pub const NAME: &str = "watch";

pub const USAGE: &str ="acc watch [<CONTEST_NAME>] [<CONTEST_TASK_NAME] <FILE_NAME>

    q: exit

    --- arg -------------------------------------------
      <CONTEST_NAME> <CONTEST_TASK_NAME> <FILE_NAME>
          Specify all
          ex.) $ acc watch practice practice_1 p1(.cpp)

      <CONTEST_NAME> <FILE_NAME>
          CONTEST_TASK_NAME and FILE_NAME are the same.
          ex.) $ acc watch practice practice_1(.cpp)

      <FILE_NAME>
          Use settings in config.toml
          ex.) $ acc watch 1(.cpp)
    ---------------------------------------------------

";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Watch <FILE_NAME> modification")
        .usage(USAGE)
        .arg(
            Arg::with_name("CONTEST_INFO")
            .required(true)
            .max_values(3)
        )
}

pub fn run(matches: &ArgMatches) {
    let contest_info: Vec<&str> = matches.values_of("CONTEST_INFO").unwrap().collect();
    let config = util::load_config(true);

    let (contest_name, contest_task_name, task_name) = match contest_info.len() {
        1 => {
            let task_name = contest_info[0];
            let contest_task_name = config.contest_task_name.unwrap_or(config.contest.clone()) + "_" + &util::remove_extension(task_name).to_lowercase();
            (config.contest, contest_task_name, task_name)
        },
        2 => {
            let contest_task_name = util::remove_extension(contest_info[1]);
            (contest_info[0].to_string(), contest_task_name, contest_info[1])
        },
        _ => {
            let contest_task_name = util::remove_extension(contest_info[1]);
            (contest_info[0].to_string(), contest_task_name, contest_info[2])
        },
    };

    let language = if util::has_extension(task_name) {
        let extension = task_name.clone().split_terminator(".").last().unwrap();
        util::select_language(config.languages, &extension).unwrap()
    } else {
        let language_name = config.selected_language.unwrap_or_else(|| {
            util::print_error("selected_language setting or file extension is needed");
            process::exit(1);
        });
        config.languages.get(&language_name).unwrap_or_else(|| {
            util::print_error(format!("\"{}\" is not found in languages", language_name));
            process::exit(1);
        }).clone()
    };
    let config = language.test;
    let extension = language.extension;
    let task_name = if util::has_extension(task_name) {
        util::remove_extension(task_name)
    } else {
        task_name.to_string()
    };
    let (inputs, outputs) = test::get_testcases(&contest_name, contest_task_name);

    let mut path = env::current_dir().unwrap();
    let file_name = [task_name.clone(), extension].join(".");
    path.push(file_name);
    let path = path.to_str().unwrap();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
    println!("watching task {} ...", task_name);
    let mut count = 1;
    let mut stdin = async_stdin().bytes();
    let mut stdout = stdout().into_raw_mode().unwrap();
    stdout.flush().unwrap();
    loop {
        match rx.recv_timeout(Duration::from_millis(10)) {
            Ok(event) => {
                if let DebouncedEvent::Write(_) = event {
                    stdout.suspend_raw_mode().unwrap();
                    println!("[{}]: {}", count, Local::now().format("%Y/%m/%d %H:%M:%S"));
                    test::test(&task_name, &inputs, &outputs, &config);
                    println!("\n");
                    stdout.activate_raw_mode().unwrap();
                    count += 1;
                }
            },
            Err(e) => {
                if e == RecvTimeoutError::Timeout {
                    let b = stdin.next();
                    if let Some(k) = b {
                        let k = parse_event(k.unwrap(), &mut stdin).unwrap_or(Event::Key(Key::Char('q')));
                        match k {
                            Event::Key(Key::Char('q')) | Event::Key(Key::Ctrl('c')) => {
                                stdout.suspend_raw_mode().unwrap();
                                println!("{}: finished", colortext::INFO);
                                break;
                            },
                            _ => {}
                        }
                    }
                } else {
                    util::print_error("can not watch file modification");
                    process::exit(1);
                }
            }
        }
    }
}
