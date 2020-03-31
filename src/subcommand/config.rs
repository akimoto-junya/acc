use std::io::Write;
use std::fs::File;
use std::process;
use clap::{App, ArgMatches, SubCommand, Arg};
use crate::util;

pub const NAME: &str = "config";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Sets username and password to config_dir/config.toml")
        .arg(Arg::with_name("USERNAME")
            .required(true)
            .index(1))
        .arg(Arg::with_name("PASSWORD")
             .required(true)
             .index(2))
}

pub fn run(matches: &ArgMatches) {
    let username = matches.value_of("USERNAME").unwrap();
    let password = matches.value_of("PASSWORD").unwrap();
    let mut config = util::load_config(false);
    let config_path = dirs::config_dir().unwrap_or_else(|| {
            util::print_error("config directory is not defined");
            process::exit(1)
        }).to_string_lossy().to_string() + "/acc/config.toml";
    config.user.username = Some(username.to_string());
    config.user.password = Some(password.to_string());
    let mut config_file = File::create(config_path).unwrap();
    let content = toml::to_string(&config).unwrap();
    config_file.write_all(content.as_bytes()).unwrap_or_else(|_| {
        util::print_error("failed to write to config");
        process::exit(1);
    });
}
