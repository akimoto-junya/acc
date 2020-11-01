use crate::acc_client::{self, AccClient};
use crate::util;
use clap::{App, ArgMatches, SubCommand};
use termion::input::TermRead;
use std::io::{Write, stdin, stdout};
use std::process;

pub const NAME: &str = "login";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME).about("Login to Atcoder")
}

pub fn run(_matches: &ArgMatches) {
    /* ユーザネームとパスワードを入力 */
    let stdin = stdin();
    let stdout = stdout();
    let mut stdout = stdout.lock();
    let mut stdin = stdin.lock();
    stdout.write_all(b"username:").unwrap();
    stdout.flush().unwrap();
    let username = stdin.read_line()
        .unwrap_or(Some(String::new()))
        .unwrap_or(String::new());
    let password = (|| -> String {
        loop {
            stdout.write_all(b"password:").unwrap();
            stdout.flush().unwrap();
            let password = stdin.read_passwd(&mut stdout)
                .unwrap_or(Some(String::new()))
                .unwrap_or(String::new());
            stdout.write_all(b"\npassword again:").unwrap();
            stdout.flush().unwrap();
            let password2 = stdin.read_passwd(&mut stdout)
                .unwrap_or(Some(String::new()))
                .unwrap_or(String::new());
            stdout.write_all(b"\n").unwrap();
            if &password != &password2 {
                stdout.write_all(b"password is wrong!!\n").unwrap();
                continue;
            }
            return password;
        }
    })();
    /* ログインしてクッキーとcsrfトークンを保存する */
    let client = AccClient::new(false);
    let (url, token, cookies) = client
        .login_atcoder(&username, &password)
        .unwrap_or_else(|| {
            util::print_error("login failire");
            process::exit(1);
        });
    if &url != acc_client::PRACTICE_URL {
        util::print_error("login failire");
        process::exit(1);
    }
    util::save_state(&token, cookies);
    println!("OK");
}
