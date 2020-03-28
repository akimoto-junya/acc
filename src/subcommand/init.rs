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

fn make_dir(dir_name: &str) -> bool {
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
        println!("{}: {} directory already exists", colortext::ERROR, dir_name);
        process::exit(1);
    }
    let config = util::load_config().init;
    /* 拡張子が設定されているならその分のファイルを作成 */
    if let Some(extension) = config.extension {
        let total_file = config.total_task.unwrap();
        let files = (b'A'..=b'Z').take(total_file as usize)
            .map(|c| c as char)
            .map(|file| [file.to_string(), extension.clone()].connect("."))
            .collect::<Vec<String>>();
        for file in files {
            let path = env::current_dir().unwrap();
            let path = path.to_str().unwrap();
            let path = path.to_string();
            let file_path = [path, dir_name.to_string(), file].connect("/");
            let _ = Command::new("touch")
                .arg(file_path)
                .output()
                .expect("failed to execute process");
        }
    }
}
