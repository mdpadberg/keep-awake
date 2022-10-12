use crate::{keyboard::Keyboard, mouse::Mouse, programname::ProgramName, settings::Settings};
use clap::{ArgMatches, Command};

pub trait MySubCommand {
    const NAME: &'static str;
    fn command() -> Command<'static>;
    fn action(args: &ArgMatches);
}

pub fn parse() {
    let settings = Settings::load();
    let application_name = if settings.is_some() {
        settings.unwrap().application_name
    } else {
        String::from("ka")
    };

    let matches = Command::new(&application_name)
        .bin_name(&application_name)
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
