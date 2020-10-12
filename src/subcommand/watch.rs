use std::{env, process};
use std::time::Duration;
use std::sync::mpsc::channel;
use clap::{App, ArgMatches, SubCommand, Arg};
use notify::{Watcher, watcher,RecursiveMode, DebouncedEvent};
use chrono::Local;
use crate::util;
use crate::subcommand::test;

pub const NAME: &str = "watch";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Watch <TASK_NAME> modification")
        .arg(Arg::with_name("TASK_NAME")
            .required(true)
            .index(1))
}

pub fn run(matches: &ArgMatches) {
    let task_name = matches.value_of("TASK_NAME").unwrap();
    let config = util::load_config(true);
    let userdata = util::load_userdata();
    let username = userdata.username;
    let password = userdata.password;
    let contest_task_name = config.contest_task_name;
    if username.is_none() || password.is_none() {
        util::print_error("username (or/and) password in config.toml is not defined");
        process::exit(1);
    }
    let username = username.unwrap();
    let password = password.unwrap();
    let contest_name = config.contest.unwrap_or_else( ||{
        util::print_error("contest_name in local config.toml is not defined");
        process::exit(1);
    });
    let extension = config.extension;
    if extension.is_none() {
        util::print_error("extension in local config.toml is not defined");
        process::exit(1);
    }
    let extension = extension.unwrap();
    let config = config.test;
    let (inputs, outputs) = test::get_testcases(&contest_name, contest_task_name, &task_name, &username, &password);

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
    let mut path = env::current_dir().unwrap();
    let file_name = [task_name, &extension].join(".");
    path.push(file_name);
    let path = path.to_str().unwrap();
    watcher.watch(path, RecursiveMode::NonRecursive).unwrap();
    println!("watching task {} ...", task_name);
    let mut count = 1;
    loop {
        match rx.recv() {
            Ok(event) => {
                if let DebouncedEvent::Write(_) = event {
                    println!("[{}]: {}", count, Local::now().format("%Y/%m/%d %H:%M:%S"));
                    test::test(&task_name, &inputs, &outputs, &config);
                    println!("\n");
                    count += 1;
                }
            },
            Err(_e) => {
                util::print_error("can not watch file modification");
                process::exit(1);
            }
        }
    }
}
