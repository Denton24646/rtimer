use std::error::Error;
use std::env;
use std::time::{Duration, Instant};
use std::thread::sleep;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Waiting {} {}...", config.time, config.interval);

    let duration = Duration::from_secs(config.time);
    
    sleep(duration);

    println!("I finished waiting. The time is now {:?} ", Instant::now());

    Ok(())
}

pub struct Config {
    pub time: u64,
    pub countdown: bool,
    pub interval: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let time = match args.next() {
            Some(arg) => arg.parse().unwrap(),
            None => return Err("Didn't get a timer amount"),
        };

        let interval = match args.next() {
            Some(arg) => arg.parse().unwrap(), //TODO parse into string and check if "m" or "s"
            None => return Err("Didn't get an interval amount"),
        };

        let countdown = match args.next() {
            Some(arg) => !arg.is_empty(), //TODO fix hack
            None => return Err("Didn't get a countdown boolean"),
        };

        Ok(Config { time, interval, countdown })
    }
}


