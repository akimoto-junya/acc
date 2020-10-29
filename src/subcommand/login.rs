use crate::acc_client::AccClient;
use crate::util;
use clap::{App, ArgMatches, SubCommand};
use rpassword::read_password;
use std::io::Write;
use std::process;

pub const NAME: &str = "login";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME).about("Login to Atcoder")
}

pub fn run(_matches: &ArgMatches) {
    /* ユーザネームとパスワードを入力 */
    print!("username:");
    std::io::stdout().flush().unwrap();
    let mut username = String::new();
    std::io::stdin().read_line(&mut username).unwrap();
    let mut password = String::new();
    let mut password2 = String::new();
    loop {
        print!("password:");
        std::io::stdout().flush().unwrap();
        password = read_password().unwrap();
        print!("password again:");
        std::io::stdout().flush().unwrap();
        password2 = read_password().unwrap();
        if password == password2 {
            break;
        }
        println!("password is wrong!!\n");
    }
    /* ログインしてクッキーとcsrfトークンを保存する */
    let client = AccClient::new(false);
    let (_, token, cookies) = client
        .login_atcoder(&username, &password)
        .unwrap_or_else(|| {
            util::print_error("login failire");
            process::exit(1);
        });
    util::save_state(&token, cookies);
    println!("OK");
}
