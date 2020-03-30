use clap::{App, ArgMatches, SubCommand, Arg};

pub const NAME: &str = "config";

pub fn get_command<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name(&NAME)
        .about("Set AtCoder login info")
        .arg(Arg::with_name("DIR_NAME")
            .required(true)
            .index(1))
}

pub fn run(matches: &ArgMatches) {

}
