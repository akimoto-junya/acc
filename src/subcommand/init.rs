use std::{process, env};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
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
    let config = util::load_config(false).init;
    if let Some(extension) = config.extension {
        println!("{}", extension);
        let total_file = config.total_task.unwrap();
        let files = (b'A'..=b'Z').take(total_file as usize)
            .map(|c| c as char)
            .map(|file| [file.to_string(), extension.clone()].join("."))
            .collect::<Vec<String>>();
        let path = env::current_dir().unwrap();
        let path = path.to_str().unwrap();
        let config_dir = dirs::config_dir().unwrap_or_else(|| {
                    util::print_error("config directory is not defined");
                    process::exit(1);
                }).to_string_lossy().to_string() + "/acc";
        let template_path = [config_dir + "/template", extension].join(".");
        let exists_template = Path::new(&template_path).exists();
        let template = fs::read_to_string(template_path).unwrap_or(String::new());
        for file in files {
            let file_path = [path, dir_name, &file].join("/");
            let mut file = File::create(file_path).unwrap();
            if exists_template {
                file.write_all(template.as_bytes()).unwrap_or_else(|_| {
                    util::print_error("failed to copy template");
                    process::exit(1);
                });
            }
        }
    }
}
