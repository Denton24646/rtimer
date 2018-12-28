use std::error::Error;
use std::env;
use std::time::{Duration};
use std::thread::sleep;
use std::string::String;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(">>Waiting {} {} {}", config.time, config.interval, config.countdown_description(config.countdown));

    let mut duration = Duration::from_secs(config.time);
    let increment = Duration::new(1, 0);

    loop {
    match duration.checked_sub(increment) {
        Some(new_duration) => {
            println!("Time passed: {:?} ", duration);
            duration = new_duration;
            sleep(increment); 
        }
        None => {
            println!(">>Done! Waited {} {}...", config.time, config.interval); 
            break
            }
        }
    }
    
    Ok(())
}

pub struct Config {
    pub time: u64,
    pub interval: String,
    pub countdown: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let time = match args.next() {
            Some(arg) => arg.parse().unwrap(), 
            None => return Err("Didn't get a timer amount"),
        };

        let interval = match args.next() {
            Some(arg) => {
                let mut i: String = arg.parse().unwrap();
                match i.as_ref() {
                    "s" | "sec" | "seconds" => { i = String::from("seconds");},
                    "m" | "min" | "minutes" => { i = String::from("minutes");},
                    _ => return Err("Unsupported interval amount"),
                }
                i
            },
            None => return Err("Didn't get an interval amount"),
        };

        let countdown = match args.next() {
            Some(arg) => {
                let mut c: bool = arg.parse().unwrap();
                match c {
                    true => {c = true;},
                    false => {c = false;}
                }
                c
            }
            None => return Err("Didn't get a countdown boolean"),
        };

        Ok(Config { time, interval, countdown })
    }

    pub fn countdown_description(&self, c: bool) -> String {
        match c {
            true => {let r = String::from("counting down..."); return r},
            false => {let r = String::from("counting up..."); return r},
        };
    }
}


