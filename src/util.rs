use crate::colortext;
use crate::config::state::{Cookie, State};
use crate::config::{Config, Language};
use std::io::prelude::*;
use std::collections::HashMap;
use std::{env, fs, process};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{cursor, style, clear};
use termion::cursor::DetectCursorPos;
use std::io::{Write, stdout, stdin, Stdout};

const DEFAULT_CONFIG: &str = r#"contest = "config"
selected_language = "cpp_gcc"

[languages.cpp_gcc]
extension = "cpp"
language_id = "4003"

[languages.cpp_gcc.test]
compiler = "g++"
compile_arg = "-std=gnu++17 -o <TASK> <TASK>.cpp"
command = "./<TASK>"
tle_time = 3000
print_wrong_answer = true

[languages.python]
extension = "py"
language_id = "4006"

[languages.python.test]
command = "python3"
command_arg = "<TASK>.py"
tle_time = 3000
print_wrong_answer = true
"#;

pub fn print_error<S: Into<String>>(error_message: S) {
    let error_message = error_message.into();
    println!("{}: {}", colortext::ERROR, error_message);
}

pub fn print_warning<S: Into<String>>(warning_message: S) {
    let warning_message = warning_message.into();
    println!("{}: {}", colortext::WARNING, warning_message);
}

pub fn make_dir(dir_name: &str) -> bool {
    match fs::create_dir(dir_name) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn has_extension<S: Into<String>>(task: S) -> bool {
    let task = task.into();
    task.contains(".")
}

pub fn remove_last_indent<S: Into<String>>(content: S) -> String {
    let mut result = content.into();
    if result.ends_with("\n") {
        result.pop();
    }
    result
}

pub fn select_language(languages: HashMap<String, Language>, extension: &str) -> Option<Language> {
    let languages = languages.into_iter()
        .filter(|(_n, l)| &l.extension == extension)
        .collect::<Vec<(String, Language)>>();
    if languages.is_empty() {
        return None;
    } else if languages.len() == 1 {
        return Some(languages[0].1.clone());
    }

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout,"{}", cursor::Hide).unwrap();
    stdout.flush().unwrap();

    /* 表示するコンテンツ分のスペースを改行で確保 */
    let count = languages.len() as u16;
    let preface = 1;
    for _ in 0..(count + preface) {
        print!("\n");
    }
    stdout.flush().unwrap();

    /* 表示開始位置に戻り，その位置を保存しておく */
    write!(stdout, "{}", cursor::Up(count + preface)).unwrap();
    write!(stdout, "{}", cursor::Save).unwrap();

    let mut down: u16 = 0;
    let (_, min_top) = stdout.cursor_pos().unwrap();

    /* 表示用のクロージャ作成 */
    let print_content = |stdout: &mut Stdout, down: &mut u16| {
        write!(stdout, "{}", cursor::Goto(1, min_top)).unwrap();
        write!(stdout, "{}", clear::AfterCursor).unwrap();
        print!("*** Select programming language ***");
        for (i, x) in (0..count).zip(languages.iter()) {
            let name = if i == *down {
                format!("- {}{}{}", style::Underline, x.0 , style::Reset)
            } else {
                String::from("- ") + &x.0
            };
            write!(stdout, "{}", termion::cursor::Goto(1, min_top + i + preface)).unwrap();
            print!("{}", name);
        }
        stdout.flush().unwrap();
    };

    print_content(&mut stdout, &mut down);
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('q') => break,
            Key::Char('j') | Key::Down  => down  = (down + 1) % count,
            Key::Char('k') | Key::Up    => down  = (count + down - 1) % count,
            Key::Char('\n') => {
                break;
            },
            _ => {}
        }
        print_content(&mut stdout, &mut down);
    }
    /* 後処理をして終了 */
    write!(stdout, "{}{}{}", cursor::Restore, cursor::Show, clear::AfterCursor).unwrap();
    stdout.flush().unwrap();
    Some(languages[down as usize].1.clone())
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
    let config_path_exists = config_path.as_path().exists();
    let config_path = config_path.to_str().unwrap();
    if !(is_local || config_path_exists) {
        let _ = make_dir(config_dir.to_str().unwrap());
        let mut file = fs::File::create(config_path).unwrap();
        file.write_all(DEFAULT_CONFIG.as_bytes()).unwrap();
        println!("{}: {} is created", colortext::INFO, config_path);
    }

    let content = read_file(config_path);
    let config: Config = toml::from_str(&content).unwrap_or_else(|_| {
        print_error("config content is wrong");
        process::exit(1);
    });
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
