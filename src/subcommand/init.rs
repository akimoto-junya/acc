use std::{process, env};
use std::process::Command;
use clap::{App, ArgMatches, SubCommand, Arg};
use crate::colortext;
use crate::util;

pub const NAME: &str = "init";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Create a new atcoder project")
        .arg(Arg::with_name("DIR_NAME")
            .required(true)
            .index(1))
}

fn make_dir<S: Into<String>>(dir_name: S) -> bool {
    let dir_name = dir_name.into();
    let output = Command::new("mkdir")
        .arg(dir_name)
        .output()
        .expect("failed to execute process");
    output.status.success()
}

pub fn run(matches: &ArgMatches) {
    let dir_name = matches.value_of("DIR_NAME").unwrap();
    let is_successful = make_dir(dir_name);
    if !is_successful {
        util::print_error(format!("{} directory already exists", dir_name));
        process::exit(1);
    }
    let _ = make_dir(dir_name.to_string() + "/testcase");

    /* 拡張子が設定されているならその分のファイルを作成 */
    let config = util::load_config(true).init;
    if let Some(extension) = config.extension {
        let total_file = config.total_task.unwrap();
        let files = (b'A'..=b'Z').take(total_file as usize)
            .map(|c| c as char)
            .map(|file| [file.to_string(), extension.clone()].join("."))
            .collect::<Vec<String>>();
        for file in files {
            let path = env::current_dir().unwrap();
            let path = path.to_str().unwrap();
            let path = path.to_string();
            let file_path = [path, dir_name.to_string(), file].join("/");
            let _ = Command::new("touch")
                .arg(file_path)
                .output()
                .expect("failed to execute process");
        }
    }
}
