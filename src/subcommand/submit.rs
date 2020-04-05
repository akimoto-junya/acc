use std::{env, process};
use clap::{App, ArgMatches, SubCommand, Arg};
use reqwest::Client;
use crate::util;

pub const NAME: &str = "submit";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Submits source code to AtCoder")
        .arg(Arg::with_name("TASK_NAME")
            .required(true)
            .index(1))
}

fn get_source(file_name: &str) -> String {
    let mut path = env::current_dir().unwrap();
    path.push(file_name);
    let path = path.to_str().unwrap();
    util::read_file(path)
}

pub fn run(matches: &ArgMatches) {
    let task_name = matches.value_of("TASK_NAME").unwrap();
    let config = util::load_config(true);
    let contest_task_name = config.contest_task_name;
    let extension = config.extension.unwrap_or_else(|| {
        util::print_error("extension in config.toml is not defined");
        process::exit(1);
    });
    let language_id = config.language_id.unwrap_or_else(|| {
        util::print_error("language_id in config.toml is not defined");
        process::exit(1);
    });
    let userdata = util::load_userdata();
    let username = userdata.username;
    let password = userdata.password;
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
    let file_name = [task_name, &extension].join(".");

    let client = Client::builder().cookie_store(true).user_agent("acc/1.0.0").build().unwrap();
    let url = util::LOGIN_URL;
    let token = util::login_atcoder(url, &client, &username, &password);
    let url = util::SUBMIT_URL.to_string();
    let url = url.replace("<CONTEST>", &contest_name);
    let task = if let Some(contest_task_name) = contest_task_name {
        contest_task_name
    } else {
        contest_name
    };
    let screen_name = format!("{}_{}", &task, task_name.to_lowercase());
    let source = get_source(&file_name);
    let form_data = vec![
        ("csrf_token", token),
        ("sourceCode", source),
        ("data.LanguageId", language_id.to_string()),
        ("data.TaskScreenName", screen_name),
    ];
    let result = util::post_form_data(&url, &client, form_data);
    if result.is_some() {
        println!("OK");
    } else {
        util::print_error("failed to submit source code");
    }
}
