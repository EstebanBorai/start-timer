use clap::Subcommand;

pub mod new;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Creates a new timer and dispatches it
    New(new::NewOpt),
}
