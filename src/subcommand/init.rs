use std::process;
use std::process::Command;
use clap::{App, ArgMatches, SubCommand, Arg};
use crate::colortext;
use crate::util;

pub const NAME: &str = "init";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Create a new atcoder project")
        .arg(Arg::with_name("DIR_NAME")
            .required(true)
            .index(1))
}

fn make_dir(dir_name: &str) -> bool {
    let output = Command::new("mkdir")
        .arg(dir_name)
        .output()
        .expect("failed to execute process");
    output.status.success()
}

fn copy_config_file() {

}

pub fn run(matches: &ArgMatches) {
    let dir_name = matches.value_of("DIR_NAME").unwrap();
    let is_successful = make_dir(dir_name);
    if !is_successful {
        println!("{}: {} directory already exists", colortext::ERROR, dir_name);
        process::exit(1);
    }
    let config = util::load_config();
}
