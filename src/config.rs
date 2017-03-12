use std::time::Duration;

/// Represents configuration given through command line arguments
pub struct Config {
    // timeout in second, will be translated to std::time::Duration
    timeout: u64,
    //
    buffer_size: usize,
}

impl Config {
    /// Create a new configuration object with default values
    pub fn new() -> Self {
        Config {
            timeout: 0,
            buffer_size: 65535,
        }
    }

    /// Set timeout as u64 integer
    pub fn set_timeout(&mut self, seconds: u64) {
        self.timeout = seconds;
    }

    /// Get timeout as time::Duration type
    pub fn get_timeout(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }
}