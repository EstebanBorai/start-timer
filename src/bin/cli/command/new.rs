use clap::Parser;

#[derive(Debug, Parser)]
pub struct NewOpt {
    /// Duration of the timer in seconds
    #[clap(short = 'd', long = "duration")]
    pub duration: u64,
}
