use std::{fs, process, env};
use std::io::prelude::*;
use std::process::Command;
use crate::colortext;
use crate::config::{Config, Overridable};

pub fn print_error<S: Into<String>>(error_message: S) {
    let error_message = error_message.into();
    println!("{}: {}", colortext::ERROR, error_message);
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
        env::current_dir().unwrap_or_else(
            |_| {
                print_error(format!("config directory is not defined"));
                process::exit(1);
            }
        ).to_string_lossy().to_string()
    } else {
        dirs::config_dir().unwrap_or_else(
            || {
                print_error(format!("config directory is not defined"));
                process::exit(1);
            }
        ).to_string_lossy().to_string() + "/acc"
    };
    let config_path: &str = &(config_dir.clone() + "/config.toml");

    if !is_local {
        let output = Command::new("test")
            .args(&["-d", &config_dir])
            .output()
            .expect("failed to execute process");
        if !output.status.success() {
            let _ = Command::new("mkdir")
                .args(&["-p", &config_dir])
                .output()
                .expect("failed to execute process");
            let mut file = fs::File::create(config_path).unwrap();
            file.write_all(DEFAULT_CONFIG.as_bytes()).unwrap();
        }
    }

    let content = fs::read_to_string(config_path).unwrap();
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
