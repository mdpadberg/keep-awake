use std::{thread, time::Duration};

use crate::cli::MySubCommand;
use clap::{Arg, Command};
use enigo::{Enigo, Key, KeyboardControllable};

pub struct Keyboard;

impl MySubCommand for Keyboard {
    const NAME: &'static str = "keyboard";

    fn command() -> clap::Command {
        Command::new(Keyboard::NAME)
            .about("Use keyboard to keep your machine awake [default tab & windows/super/command]")
            .arg(
                Arg::new("interval")
                    .help("set a time interval in seconds on how often you want to run this")
                    .long("interval")
                    .default_value("60"),
            )
    }

    fn action(args: &clap::ArgMatches) {
        let interval = args.get_one::<String>("interval").unwrap();
        let mut enigo = Enigo::new();
        if let Ok(interval) = interval.parse::<u64>() {
            loop {
                thread::sleep(Duration::from_secs(interval));
                enigo.key_down(Key::Meta);
                thread::sleep(Duration::from_millis(200));
                enigo.key_down(Key::Tab);
                thread::sleep(Duration::from_millis(200));
                enigo.key_up(Key::Tab);
                thread::sleep(Duration::from_millis(200));
                enigo.key_up(Key::Meta);
            }
        } else {
            eprintln!(
                "Could not parse interval:{}. Did you configure a valid number?",
                interval
            );
        }
    }
}
