use clap::Parser;

#[derive(Debug, Parser)]
pub struct NewOpt {
    /// Label used to identify the timer
    #[clap(short = 'l', long = "label")]
    pub label: String,
    /// Duration of the timer in seconds
    #[clap(short = 'd', long = "duration")]
    pub duration: u64,
}
