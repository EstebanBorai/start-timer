mod cli;

use std::{thread::sleep, time::Duration};

use clap::Parser;

use start_timer::clock::Clock;

use self::cli::StartTimerCli;

fn main() {
    let args = StartTimerCli::parse();

    match args.command {
        cli::command::Command::New(new_opt) => {
            let clock = Clock::new(Duration::from_secs(new_opt.duration));

            loop {
                sleep(Duration::from_secs(1));

                if clock.has_ended() {
                    println!("Timer has ended");
                    break;
                }

                println!("{}", clock);
                print!("{esc}c", esc = 27 as char);
            }
        }
    }
}
