use std::{
    sync::{Arc, Mutex},
    time::SystemTime,
};

use appbiotic_error::StatusResult;
use chrono::{DateTime, Utc};

use crate::timestamp::Timestamp;

pub trait Clock {
    fn now(&self) -> StatusResult<Timestamp>;
}

pub struct StdClock;

impl Clock for StdClock {
    fn now(&self) -> StatusResult<Timestamp> {
        Ok(Timestamp(Utc::now()))
    }
}

pub struct ClockFake {
    timestamp: Arc<Mutex<Timestamp>>,
}

impl ClockFake {
    pub fn new(timestamp: Arc<Mutex<Timestamp>>) -> Self {
        Self { timestamp }
    }

    pub fn update(&self, timestamp: Timestamp) {
        let mut current = self.timestamp.lock().unwrap();
        *current = timestamp;
    }
}

impl Default for ClockFake {
    fn default() -> Self {
        Self::new(Arc::new(Mutex::new(
            DateTime::from(SystemTime::UNIX_EPOCH).into(),
        )))
    }
}

impl Clock for ClockFake {
    fn now(&self) -> StatusResult<Timestamp> {
        Ok(self.timestamp.lock().unwrap().clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn clock_fake_default() {
        let clock = ClockFake::default();
        let now = clock.now().unwrap();
        assert_eq!(now.to_string(), "1970-01-01T00:00:00.000000000Z");
    }

    #[test]
    fn clock_fake_updates() {
        let clock = ClockFake::default();
        clock.update("1999-12-31T23:59:59.999999999Z".parse().unwrap());
        assert_eq!(
            clock.now().unwrap().to_string(),
            "1999-12-31T23:59:59.999999999Z"
        );
    }
}
