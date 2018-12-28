use std::env;
use std::thread;
use std::io;
use std::error::Error;
use std::time::Duration;
use std::thread::sleep;
use std::sync::mpsc;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!(">>Waiting {} {}...", config.time, config.interval);

    let mut duration = Duration::from_secs(config.time);
    let increment = Duration::new(1, 0);
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
        let received = rx.try_recv();
        match received {
            Ok(_) => {
                sleep(Duration::from_secs(400000000000))
            }
            Err(_) => {
                continue
            }
        }
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
    });

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            tx.send(input).unwrap()
        }
        Err(error) => println!("error: {}", error),
    }

    
    
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


