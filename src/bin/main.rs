mod cli;
mod osx;

use anyhow::Result;
use clap::Parser;

use self::cli::StartTimerCli;
use self::osx::MenuItem;

fn main() -> Result<()> {
    let args = StartTimerCli::parse();

    match args.command {
        cli::command::Command::New(new_opt) => {
            MenuItem::run();
        }
    }

    Ok(())
}
