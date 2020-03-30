use std::process::Command;
use clap::{App, ArgMatches, SubCommand, Arg};

pub const NAME: &str = "submit";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Submit atcoder ")
        .arg(Arg::with_name("TASK_NAME")
            .required(true)
            .index(1))
}

pub fn run(matches: &ArgMatches) {
    // if let Some(time) = file.modified() {

    // } else {
    //     // The program may have been updated
    // }
    // time -> latest ok
    // The task hasn't passed the test, but is it safe to submit it?


}
