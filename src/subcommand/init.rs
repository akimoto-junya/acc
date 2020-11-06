use crate::colortext;
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use std::{env, process};

pub const NAME: &str = "init";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Create a new atcoder project")
        .arg(Arg::with_name("DIR_NAME").required(true).index(1))
        .arg(
            Arg::with_name("language_name")
                .short("l")
                .long("language")
                .help("Selects a programming language")
                .takes_value(true)
                .value_name("LANGUAGE_NAME"),
        )
        .arg(
            Arg::with_name("total_task")
                .short("t")
                .long("total")
                .help("Sets a number of tasks")
                .takes_value(true)
                .value_name("TOTAL"),
        )
}

pub fn run(matches: &ArgMatches) {
    let dir_name = matches.value_of("DIR_NAME").unwrap();
    let language_name = matches.value_of("language_name");
    let total_task = matches.value_of("total_task");
    let path = env::current_dir().unwrap();
    let path = path.to_str().unwrap();
    let config = util::load_config(false);

    let is_successful = util::make_dir(dir_name);
    if !is_successful {
        util::print_error(format!("{} directory already exists", dir_name));
        process::exit(1);
    }
    let _ = util::make_dir(&[dir_name, "testcase"].join("/"));

    // コンテスト名, 拡張子, 言語IDを追記してローカルに保存
    let local_config_path = [path, dir_name, "config.toml"].join("/");
    let mut config = config.clone();
    config.contest = dir_name.to_string();
    if let Some(language_name) = language_name {
        config.selected_language = Some(language_name.to_string());
    }
    let content = toml::to_string(&config).unwrap();
    let mut local_config_file = File::create(local_config_path).unwrap();
    local_config_file
        .write_all(content.as_bytes())
        .unwrap_or_else(|_| {
            util::print_error("failed to create local config");
            process::exit(1);
        });

    // 言語が設定されているならその分のファイルを作成
    let language_name = config.selected_language.clone();
    if let Some(language_name) = language_name {
        if !config.languages.contains_key(&language_name) {
            util::print_error(format!("\"{}\" is not found in languages", language_name));
            process::exit(1);
        }
        let extension = config.languages[&language_name].extension.clone();
        let total_file = if total_task.is_some() {
            total_task.unwrap().parse().unwrap_or_default()
        } else {
            6 // 指定されていないならA〜F作成
        };
        if total_file <= 0 {
            println!("{}: TOTAL_TASK can not set", colortext::WARNING);
        }
        let files = (b'A'..=b'Z')
            .take(total_file as usize)
            .map(|c| c as char)
            .map(|file| [file.to_string(), extension.clone()].join("."))
            .collect::<Vec<String>>();
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| {
                util::print_error("config directory is not defined");
                process::exit(1);
            })
            .to_string_lossy()
            .to_string()
            + "/acc";
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
