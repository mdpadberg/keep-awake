use crate::mouse::Mouse;
use clap::{ArgMatches, Command};

pub trait MySubCommand {
    const NAME: &'static str;
    fn command() -> Command<'static>;
    fn action(args: &ArgMatches);
}

pub fn parse() {
    let matches = Command::new("ka")
        .bin_name("ka")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Keep you machine awake")
        .arg_required_else_help(true)
        .subcommands(vec![Mouse::command()])
        .get_matches();

    match matches.subcommand() {
        Some((Mouse::NAME, args)) => Mouse::action(args),
        _ => unreachable!(),
    }
}
