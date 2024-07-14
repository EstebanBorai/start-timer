use clap::Subcommand;

pub mod new;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Creates a new timer and starts it
    New(new::NewOpt),
}
