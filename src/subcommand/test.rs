use std::process::Command;
use clap::{App, ArgMatches, SubCommand, Arg};

pub const NAME: &str = "test";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Create a new atcoder project")
        .arg(Arg::with_name("DIR_NAME")
            .required(true)
            .index(1))
}

pub fn run(matches: &ArgMatches) {
    let dir_name = matches.value_of("DIR_NAME").unwrap();
    let status = Command::new("mkdir")
        .arg(dir_name)
        .status()
        .expect("failed to excute process");
    println!("{}", status.success());
}
