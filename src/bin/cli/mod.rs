pub mod command;

use clap::Parser;

use self::command::Command;

#[derive(Debug, Parser)]
#[command(
    name = "start-timer",
    about = "CLI Utility to Start a Timer",
    author = "Esteban Borai <estebanborai@gmail.com> (https://github.com/EstebanBorai/start-timer)",
    next_line_help = true
)]
pub struct StartTimerCli {
    #[clap(subcommand)]
    pub command: Command,
}
