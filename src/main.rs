extern crate acc;
use clap::App;
use acc::subcommand;

fn main() {
    let matches = App::new("acc")
        .version("v1.0.0")
        .subcommand(subcommand::init::get_command())
        .subcommand(subcommand::submit::get_command())
        .subcommand(subcommand::test::get_command())
        .subcommand(subcommand::config::get_command())
        .get_matches();
    match matches.subcommand() {
        (subcommand::init::NAME, Some(matches)) => {
            subcommand::init::run(matches);
        },
        (subcommand::submit::NAME, Some(matches)) => {
            subcommand::submit::run(matches);
        },
        (subcommand::test::NAME, Some(matches)) => {
            subcommand::test::run(matches);
        },
        (subcommand::config::NAME, Some(matches)) => {
            subcommand::config::run(matches);
        }
        _ => {},
    }
}
