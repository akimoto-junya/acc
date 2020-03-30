use std::{fs, process, env};
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;
use crate::colortext;
use crate::config::{Config, Overridable};

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
    fs::read_to_string(path).unwrap_or_else(
        |_| {
            print_error(format!("{} is not found", path));
            process::exit(1);
        }
    )
}

const DEFAULT_CONFIG: &str = r#"[user]

[init]

[test]
compiler = 'g++'
compile_arg = '<TASK>.cpp -o <TASK>'
command = './<TASK>'"#;

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
