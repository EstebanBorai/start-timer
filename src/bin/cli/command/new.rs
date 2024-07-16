use clap::Parser;
use humantime::Duration;

#[derive(Debug, Parser)]
pub struct NewOpt {
    /// Duration of the timer using time expressions (e.g. 1h30m, 1h, 30m, 1h30m30s, 1h30s, 30m30s, 1h30m30s)
    #[clap(value_parser=clap::value_parser!(humantime::Duration))]
    pub duration: Duration,
}
