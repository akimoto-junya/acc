extern crate acc;
use acc::subcommand;
use clap::App;

fn main() {
    let matches = App::new("acc")
        .version("v1.1.3")
        .subcommand(subcommand::init::get_command())
        .subcommand(subcommand::submit::get_command())
        .subcommand(subcommand::test::get_command())
        .subcommand(subcommand::watch::get_command())
        .subcommand(subcommand::login::get_command())
        .get_matches();
    match matches.subcommand() {
        (subcommand::init::NAME, Some(matches)) => {
            subcommand::init::run(matches);
        }
        (subcommand::submit::NAME, Some(matches)) => {
            subcommand::submit::run(matches);
        }
        (subcommand::test::NAME, Some(matches)) => {
            subcommand::test::run(matches);
        }
        (subcommand::watch::NAME, Some(matches)) => {
            subcommand::watch::run(matches);
        }
        (subcommand::login::NAME, Some(matches)) => {
            subcommand::login::run(matches);
        }
        _ => {}
    }
}
