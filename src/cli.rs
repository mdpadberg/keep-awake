use std::{thread::sleep, time::Duration};

use chrono::Local;
use clap::{Parser, ValueEnum};
use enigo::{Enigo, Settings};
use rand::{thread_rng, Rng};

use crate::{keyboard::keyboard, mouse::mouse};

#[derive(Parser)]
#[command(name = "ka")]
#[command(bin_name = "ka")]
enum Ka {
    /// Use keyboard and mouse to keep your machine awake [default random mouse movements & tab & windows/super/command]
    KeyboardAndMouse(Args),
    /// Use keyboard to keep your machine awake [default tab & windows/super/command]
    Keyboard(Args),
    /// Use mouse to keep your machine awake [default random mouse movements]
    Mouse(Args),
}

#[derive(clap::Args, Debug)]
#[command(version, about)]
struct Args {
    ///How long you want this command to run
    time: u32,

    ///Timeunit
    #[arg(short, long, value_enum, default_value_t=TimeUnit::Minutes)]
    timeunit: TimeUnit,
}

#[derive(Debug, Clone, ValueEnum)]
enum TimeUnit {
    Hours,
    Minutes,
}

pub fn parse() -> anyhow::Result<()> {
    let mut enigo = Enigo::new(&Settings::default())?;
    match Ka::parse() {
        Ka::Keyboard(args) => {
            run_in_loop(args, &mut enigo, vec![keyboard])?;
        }
        Ka::Mouse(args) => {
            run_in_loop(args, &mut enigo, vec![mouse])?;
        }
        Ka::KeyboardAndMouse(args) => {
            run_in_loop(args, &mut enigo, vec![keyboard, mouse, mouse, mouse, mouse])?;
        }
    };
    Ok(())
}

fn run_in_loop(
    args: Args,
    enigo: &mut Enigo,
    functions: Vec<impl Fn(&mut Enigo) -> Result<(), anyhow::Error>>,
) -> anyhow::Result<()> {
    let start_time = Local::now();
    let time_to_run_in_seconds = match args.timeunit {
        TimeUnit::Hours => args.time * 3600,
        TimeUnit::Minutes => args.time * 60,
    };
    let mut rng = thread_rng();
    'outer: loop {
        let wait_amount_of_seconds_between_functions: u64 = rng.gen_range(2..30);
        for f in &functions {
            if ((Local::now().timestamp() - start_time.timestamp()) as u32) > time_to_run_in_seconds {
                break 'outer;
            }
            sleep(Duration::from_secs(wait_amount_of_seconds_between_functions));
            f(enigo)?;
        }
    }
    Ok(())
}
