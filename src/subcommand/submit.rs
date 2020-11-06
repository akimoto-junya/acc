use crate::acc_client::{self, AccClient};
use crate::util;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::{env, process};

pub const NAME: &str = "submit";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Submits source code to AtCoder")
        .arg(Arg::with_name("TASK_NAME").required(true).index(1))
}

fn get_source(file_name: &str) -> Option<String> {
    let mut path = env::current_dir().unwrap();
    path.push(file_name);
    if !path.exists() {
        return None;
    }
    let path = path.to_str().unwrap();
    Some(util::read_file(path))
}

pub fn run(matches: &ArgMatches) {
    let task_name = matches.value_of("TASK_NAME").unwrap();
    let config = util::load_config(true);
    let contest_name = config.contest;
    let contest_task_name = config.contest_task_name;
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
    let extension = language.extension;
    let file_name = [task_name, &extension].join(".");
    let language_id = language.language_id;

    let client = AccClient::new(true);
    let url = acc_client::SUBMIT_URL.to_string();
    let url = url.replace("<CONTEST>", &contest_name);
    let task = if let Some(contest_task_name) = contest_task_name {
        contest_task_name
    } else {
        contest_name.clone()
    };
    let source = get_source(&file_name).unwrap_or_else(|| {
        util::print_error(format!("{} is not found", &file_name));
        process::exit(1);
    });
    let screen_name = format!("{}_{}", &task, task_name.to_lowercase());
    let token = client.get_csrf_token().unwrap_or_else(|| {
        util::print_error("Require to run \"acc login\"");
        process::exit(1);
    });
    let form_data = vec![
        ("csrf_token", token.clone()),
        ("sourceCode", source),
        ("data.LanguageId", language_id.to_string()),
        ("data.TaskScreenName", screen_name),
    ];
    let result = client.post_form_data(&url, form_data);
    if result.is_some() {
        let (url, _, cookies) = result.unwrap();
        let correct_url = acc_client::SUBMISSIONS_URL.replace("<CONTEST>", &contest_name);
        util::save_state(&token, cookies);
        if &correct_url == &url { // submit後は自分の提出のページに遷移することを利用
            println!("OK");
        } else {
            util::print_error("failed to submit source code");
        }
    } else {
        util::print_error("failed to get submit page");
    }
}
