use std::{thread, time::Duration};

use crate::cli::MySubCommand;
use clap::{Arg, ArgMatches, Command};
use enigo::{Enigo, MouseControllable};
use rand::{thread_rng, Rng};

pub struct Mouse;

impl MySubCommand for Mouse {
    const NAME: &'static str = "mouse";

    fn command() -> clap::Command<'static> {
        Command::new(Mouse::NAME)
            .about("Use mouse to keep your machine awake")
            .arg(
                Arg::new("width")
                    .help("set your monitors width in pixels")
                    .long("width")
                    .takes_value(true)
                    .default_value("1920"),
            )
            .arg(
                Arg::new("height")
                    .help("set your monitors height in pixels")
                    .long("height")
                    .takes_value(true)
                    .default_value("1080"),
            )
            .arg(
                Arg::new("interval")
                    .help("set a time interval in seconds on how often you want to run this")
                    .long("interval")
                    .takes_value(true)
                    .default_value("60"),
            )
    }

    fn action(args: &ArgMatches) {
        let width = args.get_one::<String>("width").unwrap();
        let height = args.get_one::<String>("height").unwrap();
        let interval = args.get_one::<String>("interval").unwrap();

        if let (Ok(width), Ok(height), Ok(interval)) = (
            width.parse::<i32>(),
            height.parse::<i32>(),
            interval.parse::<u64>(),
        ) {
            let mut rng = thread_rng();
            let mut enigo = Enigo::new();
            loop {
                let a = rng.gen_range(0..width);
                let b = rng.gen_range(0..height);
                println!("{},{} ", a,b);
                thread::sleep(Duration::from_secs(interval));
                enigo.mouse_move_to(
                    a,
                    b,
                );
            }
        } else {
            eprintln!("Could not parse input width: {}, height: {}, interval:{}. Did you configure a valid number?", width, height, interval);
        }
    }
}
