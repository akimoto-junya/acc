use std::{fs, process, env};
use std::io::prelude::*;
use std::path::Path;
use reqwest::Client;
use easy_scraper::Pattern;
use crate::colortext;
use crate::config::{Config, Overridable};

const DEFAULT_CONFIG: &str = r#"[user]

[init]

[test]
compiler = 'g++'
compile_arg = '<TASK>.cpp -o <TASK>'
command = './<TASK>'"#;

pub const LOGIN_URL: &str = "https://atcoder.jp/login";
pub const TASK_URL: &str = "https://atcoder.jp/contests/<CONTEST>/tasks/<CONTEST>_<TASK>";
pub const TESTCASE_PATTERN: &str = r#"<span class="lang-ja"><h3></h3><pre>{{io}}</pre></span>"#;
pub const CSRF_TOKEN_PATTERN: &str = r#"<input type="hidden" name="csrf_token" value={{token}} />"#;

pub fn print_error<S: Into<String>>(error_message: S) {
    let error_message = error_message.into();
    println!("{}: {}", colortext::ERROR, error_message);
}

pub fn make_dir(dir_name: &str) -> bool {
    match fs::create_dir(dir_name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn get_page(url: &str, client: &Client) -> Option<String> {
    let task = async {
        client.get(url).send().await
    };
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let document = rt.block_on(task);
    if let Ok(doc) = document {
        let task = async {
            doc.text().await
        };
        let doc = rt.block_on(task);
        match doc {
            Ok(doc) => Some(doc),
            Err(_) => None,
        }
    } else {
        None
    }
}

pub fn post_form_data(url: &str, client: &Client, form_data: Vec<(&str, String)>) -> Option<String>{
    let mut form = reqwest::multipart::Form::new();
    for data in form_data {
        form = form.text(data.0.to_string(), data.1);
    }
    let task = async {
        client.post(url).multipart(form).send().await
    };
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let document = rt.block_on(task);
    if let Ok(doc) = document {
        let task = async {
            doc.text().await
        };
        let doc = rt.block_on(task);
        match doc {
            Ok(doc) => Some(doc),
            Err(_) => None,
        }
    } else {
        None
    }
}

pub fn login_atcoder(url: &str, client: &Client, username: &str, password: &str) {
    let document = get_page(url, client).unwrap_or_else(|| {
        print_error("The URL may be wrong");
        process::exit(1);
    });
    let pattern = Pattern::new(CSRF_TOKEN_PATTERN).unwrap();
    let token = pattern.matches(&document).iter().map(|x| x["token"].to_string()).next().unwrap_or_else(|| {
        print_error("csrf_token is not found");
        process::exit(1);
    });
    let form_data = vec![
        ("csrf_token", token.to_string()),
        ("username", username.to_string()),
        ("password", password.to_string()),
    ];
    let _ = post_form_data(url, client, form_data).unwrap_or_else(|| {
        print_error("The URL may be wrong");
        process::exit(1);
    });
}

pub fn remove_last_indent<S: Into<String>>(content: S) -> String {
    let mut result = content.into();
    if result.ends_with("\n") {
        result.pop();
    }
    result
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(
        |_| {
            print_error(format!("{} is not found", path));
            process::exit(1);
        }
    )
}

pub fn load_config(is_local: bool) -> Config {
    let config_dir = if is_local {
        env::current_dir().unwrap_or_else( |_| {
                print_error(format!("config directory is not defined"));
                process::exit(1);
            }).to_str().unwrap().to_string()
    } else {
        [dirs::config_dir().unwrap_or_else( || {
            print_error(format!("config directory is not defined"));
            process::exit(1);
        }).to_str().unwrap(), "acc"].join("/")
    };

    let config_path: &str = &[&config_dir,  "config.toml"].join("/");

    if !(is_local || Path::new(&config_dir).exists()) {
        let _ = make_dir(&config_dir);
        let mut file = fs::File::create(config_path).unwrap();
        file.write_all(DEFAULT_CONFIG.as_bytes()).unwrap();
    }

    let content = read_file(config_path);
    let mut config: Config = toml::from_str(&content).unwrap_or_else(
        |_| {
            print_error("config content is wrong");
            process::exit(1);
        }
    );
    config.init.override_by_default();
    config.test.override_by_default();
    config
}
