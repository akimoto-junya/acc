use crate::colortext;
use crate::config::state::{Cookie, State};
use crate::config::{Config, Overridable};
use std::io::prelude::*;
use std::{env, fs, process};

const DEFAULT_CONFIG: &str = r#"total_task = 6
extension = "cpp"
language_id = 3003

[test]
compiler = "g++"
compile_arg = "<TASK>.cpp -o <TASK>"
command = "./<TASK>"
print_wrong_answer = true
"#;

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

pub fn remove_last_indent<S: Into<String>>(content: S) -> String {
    let mut result = content.into();
    if result.ends_with("\n") {
        result.pop();
    }
    result
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap_or(String::from(""))
}

pub fn load_config(is_local: bool) -> Config {
    let config_dir = if is_local {
        env::current_dir().unwrap_or_else(|_| {
            print_error("config directory is not defined");
            process::exit(1);
        })
    } else {
        let mut dir = dirs::config_dir().unwrap_or_else(|| {
            print_error("config directory is not defined");
            process::exit(1);
        });
        dir.push("acc");
        dir
    };
    let mut config_path = config_dir.clone();
    config_path.push("config.toml");
    let config_path = config_path.to_str().unwrap();
    if !(is_local || config_dir.as_path().exists()) {
        let result = make_dir(config_dir.to_str().unwrap());
        if !result {
            print_error("Can not create config directory");
            process::exit(1);
        }
        let mut file = fs::File::create(config_path).unwrap();
        file.write_all(DEFAULT_CONFIG.as_bytes()).unwrap();
    }

    let content = read_file(config_path);
    let mut config: Config = toml::from_str(&content).unwrap_or_else(|_| {
        print_error("config content is wrong");
        process::exit(1);
    });
    config.test.override_by_default();
    config
}

pub fn save_state(token: &str, cookies: Vec<Cookie>) {
    let mut path = dirs::config_dir().unwrap_or_else(|| {
        print_error("config directory is not defined");
        process::exit(1);
    });
    path.push("acc");
    let config_path = path.clone();
    path.push("state.toml");
    if !config_path.exists() {
        let result = make_dir(config_path.to_str().unwrap());
        if !result {
            print_error("Can not create config directory");
            process::exit(1);
        }
        let _ = fs::File::create(&path).unwrap();
    }
    let content = toml::to_string(&State {
        csrf_token: token.to_string(),
        cookies: cookies,
    })
    .unwrap();
    let mut file = fs::File::create(&path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}

pub fn load_state() -> (String, Vec<Cookie>) {
    let mut path = dirs::config_dir().unwrap_or_else(|| {
        print_error("config directory is not defined");
        process::exit(1);
    });
    path.push("acc");
    let config_path = path.clone();
    path.push("state.toml");
    if !config_path.exists() {
        let result = make_dir(config_path.to_str().unwrap());
        if !result {
            print_error("Can not create config directory");
            process::exit(1);
        }
        let _ = fs::File::create(&path).unwrap();
    }
    let state_path = path.to_str().unwrap();
    let content = read_file(state_path);
    if content.is_empty() {
        print_error("require to run \"acc login\" command");
        process::exit(1);
    }
    let state: State = toml::from_str(&content).unwrap_or_else(|_| {
        print_error("state.toml is wrong");
        process::exit(1);
    });
    (state.csrf_token, state.cookies)
}
