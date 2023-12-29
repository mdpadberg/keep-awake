use crate::{keyboard::Keyboard, mouse::Mouse, programname::ProgramName, settings::Settings};
use clap::{ArgMatches, Command};

pub trait MySubCommand {
    const NAME: &'static str;
    fn command() -> Command;
    fn action(args: &ArgMatches);
}

pub fn parse() {
    let settings = Settings::load().unwrap_or(Settings {
        application_name: String::from("ka"),
    });
    let application_name: &str = settings.application_name.leak();

    let matches = Command::new(application_name)
        .bin_name(application_name)
        .version(env!("CARGO_PKG_VERSION"))
        .about("Keep you machine awake")
        .arg_required_else_help(true)
        .subcommands(vec![
            Mouse::command(),
            ProgramName::command(),
            Keyboard::command(),
        ])
        .get_matches();

    match matches.subcommand() {
        Some((Mouse::NAME, args)) => Mouse::action(args),
        Some((ProgramName::NAME, args)) => ProgramName::action(args),
        Some((Keyboard::NAME, args)) => Keyboard::action(args),
        _ => unreachable!(),
    }
}
