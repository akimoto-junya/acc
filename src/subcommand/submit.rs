use crate::acc_client::{self, AccClient};
use crate::util;
use crate::colortext;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::{env, process};

pub const NAME: &str = "submit";

pub const USAGE: &str ="acc submit [<CONTEST_NAME>] [<CONTEST_TASK_NAME] <FILE_NAME>

    --- arg -------------------------------------------
      <CONTEST_NAME> <CONTEST_TASK_NAME> <FILE_NAME>
        Specify all.
        ex.) $ acc submit practice practice_1 p1(.cpp)

      <CONTEST_NAME> <FILE_NAME>
        CONTEST_TASK_NAME and FILE_NAME are the same.
        ex.) $ acc submit practice practice_1(.cpp)

      <FILE_NAME>
        Use settings in config.toml and specify the FILE_NAME as task name.
        ex.) $ acc submit 1(.cpp)
    ---------------------------------------------------

";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Submits source code to AtCoder")
        .usage(USAGE)
        .arg(
            Arg::with_name("CONTEST_INFO")
            .required(true)
            .max_values(3)
        )
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
    let contest_info: Vec<&str> = matches.values_of("CONTEST_INFO").unwrap().collect();
    let config = util::load_config(true);

    let (contest_name, contest_task_name, file_name) = match contest_info.len() {
        1 => {
            let file_name = contest_info[0];
            let contest_task_name = config.contest_task_name.unwrap_or(config.contest.clone()) + "_" + &util::remove_extension(file_name).to_lowercase();
            (config.contest, contest_task_name, file_name)
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

    let language = if util::has_extension(file_name) {
        let extension = file_name.clone().split_terminator(".").last().unwrap();
        util::select_language(config.languages, &extension).unwrap_or_else(|| {
            util::print_error(format!("language setting for \".{}\" is not found", extension));
            process::exit(1);
        })
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
    let file_name = if util::has_extension(file_name) {
        file_name.to_string()
    } else {
        [file_name, &extension].join(".")
    };

    let mut path = env::current_dir().unwrap();
    path.push(file_name.clone());
    if !path.exists() {
        util::print_error(format!("{} is not found", path.to_str().unwrap()));
        process::exit(1);
    }

    let language_id = language.language_id;

    let client = AccClient::new(true);
    let url = acc_client::SUBMIT_URL.to_string();
    let url = url.replace("<CONTEST>", &contest_name);
    let source = get_source(&file_name).unwrap_or_else(|| {
        util::print_error(format!("{} is not found", &file_name));
        process::exit(1);
    });
    let token = client.get_csrf_token().unwrap_or_else(|| {
        util::print_error("Require to run \"acc login\"");
        process::exit(1);
    });
    println!("{}: submit to \"{}\"", colortext::info(), &url);
    let form_data = vec![
        ("csrf_token", token.clone()),
        ("sourceCode", source),
        ("data.LanguageId", language_id.to_string()),
        ("data.TaskScreenName", contest_task_name),
    ];
    let result = client.post_form_data(&url, form_data);
    match result {
        Some(result) => {
            let (url, _content, cookies) = result;
            let correct_url = acc_client::SUBMISSIONS_URL.replace("<CONTEST>", &contest_name);
            util::save_state(&token, cookies);
            if &correct_url == &url { // submit後は自分の提出のページに遷移することを利用
                println!("OK");
            } else {
                util::print_error("failed to submit source code");
            }
        },
        None => util::print_error("failed to get submit page")
    }
}
