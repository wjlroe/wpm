use std::env;
use std::time::Duration;

const DEFAULT_DURATION_SECS: u64 = 60;

pub struct Config {
    pub default_test_duration: Duration,
}

impl Config {
    pub fn new() -> Self {
        let mut duration_secs = DEFAULT_DURATION_SECS;
        if let Ok(duration_str) = env::var("WPM_TEST_DURATION") {
            let duration_from_env: u64 = duration_str.parse().unwrap_or(0);
            if duration_from_env > 0 {
                duration_secs = duration_from_env;
            }
        }
        Self {
            default_test_duration: Duration::from_secs(duration_secs),
        }
    }
}
