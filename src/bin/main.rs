mod cli;

use std::thread::sleep;
use std::time::Duration;

use anyhow::Result;
use clap::Parser;

use start_timer::clock::Clock;
use start_timer::sound::Sound;

use self::cli::StartTimerCli;

fn main() -> Result<()> {
    let args = StartTimerCli::parse();

    match args.command {
        cli::command::Command::New(new_opt) => {
            let clock = Clock::new(new_opt.duration.into());
            let sound = Sound::new()?;

            loop {
                sleep(Duration::from_secs(1));

                if clock.has_ended() {
                    println!("Timer has ended");
                    sound.play()?;

                    return Ok(());
                }

                println!("{}", clock);
                print!("{esc}c", esc = 27 as char);
            }
        }
    }
}
