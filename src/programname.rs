use crate::{cli::MySubCommand, settings::Settings};
use clap::{Arg, ArgMatches, Command};

pub struct ProgramName;

impl MySubCommand for ProgramName {
    const NAME: &'static str = "programname";

    fn command() -> clap::Command {
        Command::new(ProgramName::NAME)
            .about("Set name for this program")
            .arg(
                Arg::new("name")
                    .help("Set name for this program")
                    .required(true),
            )
    }

    fn action(args: &ArgMatches) {
        let current_settings = Settings::load();
        let name = args.get_one::<String>("name");
        if name.is_some() {
            let new_application_name = name.unwrap();
            let settings = if current_settings.is_some() {
                let mut new_settings = current_settings.unwrap();
                new_settings.application_name = new_application_name.clone();
                new_settings
            } else {
                Settings {
                    application_name: new_application_name.clone(),
                }
            };
            let success = settings.save();
            if success.is_err() {
                eprintln!("Could not set program name: {}", new_application_name);
            }
        } else {
            eprintln!("Could set program name because no name was specified");
        }
    }
}
