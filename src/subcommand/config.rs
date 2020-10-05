use std::io::Write;
use std::fs::File;
use std::process;
use clap::{App, ArgMatches, SubCommand};
use rpassword::read_password;
use crate::util;

pub const NAME: &str = "config";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Sets username and password to config_dir/config.toml")
}

pub fn run(_matches: &ArgMatches) {
    print!("username:");
    std::io::stdout().flush().unwrap();
    let username = read_password().unwrap();
    println!("{}", username);
    let mut password = String::new();
    let mut password2 = String::new();
    loop {
        print!("password:");
        std::io::stdout().flush().unwrap();
        password = read_password().unwrap();
        println!("{}", std::iter::repeat("*").take(password.len()).collect::<String>());
        print!("password again:");
        std::io::stdout().flush().unwrap();
        password2 = read_password().unwrap();
        if password == password2 {
            break;
        }
        println!("password is wrong!!\n");
    }

    let _ = util::load_config(false); // 初回時config.toml作成
    let mut userdata = util::load_userdata();
    let userdata_path = dirs::config_dir().unwrap_or_else(|| {
            util::print_error("config directory is not defined");
            process::exit(1)
        }).to_string_lossy().to_string() + "/acc/userdata.toml";
    userdata.username = Some(username);
    userdata.password = Some(password);
    let mut userdata_file = File::create(userdata_path).unwrap();
    let content = toml::to_string(&userdata).unwrap();
    userdata_file.write_all(content.as_bytes()).unwrap_or_else(|_| {
        util::print_error("failed to write to config");
        process::exit(1);
    });
}
