use crate::subcommand::test;
use crate::util;
use chrono::Local;
use clap::{App, Arg, ArgMatches, SubCommand};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{env, process};

pub const NAME: &str = "watch";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Watch <TASK_NAME> modification")
        .arg(Arg::with_name("TASK_NAME").required(true).index(1))
}

pub fn run(matches: &ArgMatches) {
    let task_name = matches.value_of("TASK_NAME").unwrap();
    let config = util::load_config(true);
    let contest_task_name = config.contest_task_name;
    let contest_name = config.contest;
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

    let (inputs, outputs) = test::get_testcases(&contest_name, contest_task_name, &task_name);

    let mut path = env::current_dir().unwrap();
    let file_name = [task_name, &extension].join(".");
    path.push(file_name);
    let path = path.to_str().unwrap();
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();
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
            }
            Err(_e) => {
                util::print_error("can not watch file modification");
                process::exit(1);
            }
        }
    }
}
