use std::{process, env};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use clap::{App, ArgMatches, SubCommand, Arg};
use crate::util;
use crate::colortext;

pub const NAME: &str = "init";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Create a new atcoder project")
        .arg(Arg::with_name("DIR_NAME")
            .required(true)
            .index(1))
        .arg(Arg::with_name("extension")
            .short("e")
            .long("extension")
            .help("Sets a init file extension")
            .takes_value(true)
            .value_name("EXT"))
        .arg(Arg::with_name("language_id")
            .short("l")
            .long("lang")
            .help("Sets a language id")
            .takes_value(true)
            .value_name("LANGUAGE_ID"))
        .arg(Arg::with_name("total_task")
            .short("t")
            .long("total")
            .help("Sets a number of tasks")
            .takes_value(true)
            .value_name("TOTAL"))
}


pub fn run(matches: &ArgMatches) {
    let dir_name = matches.value_of("DIR_NAME").unwrap();
    let extension = matches.value_of("extension");
    let language_id = matches.value_of("language_id");
    let total_task = matches.value_of("total_task");
    let path = env::current_dir().unwrap();
    let path = path.to_str().unwrap();
    let config = util::load_config(false);

    let is_successful = util::make_dir(dir_name);
    if !is_successful {
        util::print_error(format!("{} directory already exists", dir_name));
        process::exit(1);
    }
    let _ = util::make_dir(&[dir_name , "testcase"].join("/"));

    // コンテスト名, 拡張子, 言語IDを追記してローカルに保存
    let local_config_path = [path, dir_name, "config.toml"].join("/");
    let mut overriding_config = config.clone();
    overriding_config.contest = Some(dir_name.to_string());
    if let Some(extension) = extension {
        overriding_config.extension = Some(extension.to_string());
    }
    if let Some(language_id) = language_id {
        let language_id: u16 = language_id.parse().unwrap_or_else(|_| {
            util::print_error("language_id is wrong");
            process::exit(1);
        });
        overriding_config.language_id = Some(language_id);
    }
    let content = toml::to_string(&overriding_config).unwrap();
    let mut local_config_file = File::create(local_config_path).unwrap();
    local_config_file.write_all(content.as_bytes()).unwrap_or_else(|_| {
        util::print_error("failed to create local config");
        process::exit(1);
    });

    // 拡張子が設定されているならその分のファイルを作成
    let extension = if extension.is_some() {
        match extension {
            Some(ext) => Some(ext.to_string()),
            None => None,
        }
    } else {
        config.extension
    };
    if let Some(extension) = extension {
        let total_file = if total_task.is_some() {
            total_task.unwrap().parse().unwrap_or_default()
        } else {
            config.total_task.unwrap()
        };
        if total_file <= 0 {
            println!("{}: TOTAL_TASK can not set", colortext::WARNING);
        }
        let files = (b'A'..=b'Z').take(total_file as usize)
            .map(|c| c as char)
            .map(|file| [file.to_string(), extension.clone()].join("."))
            .collect::<Vec<String>>();
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
