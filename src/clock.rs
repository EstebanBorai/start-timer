use std::fmt::Display;
use std::time::{Duration, Instant};

pub struct Clock {
    pub durantion: Duration,
    pub started_at: Instant,
}

impl Clock {
    pub fn new(duration: Duration) -> Self {
        Self {
            durantion: duration,
            started_at: Instant::now(),
        }
    }

    pub fn has_ended(&self) -> bool {
        self.started_at.elapsed() >= self.durantion
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let elapsed = self.started_at.elapsed();
        let remaining = self.durantion.checked_sub(elapsed).unwrap_or_default();

        write!(f, "{}", remaining.as_secs())
    }
}
