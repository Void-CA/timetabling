use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DTime {
    hour: u8,
    minutes: u8,
    seconds: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimeError {
    InvalidHour(u8),
    InvalidMinutes(u8),
    InvalidSeconds(u8),
}

impl DTime {
    pub(super) const SECONDS_PER_DAY: u32 = 86_400;

    pub fn new(hour: u8, minutes: u8, seconds: u8) -> Result<DTime, TimeError> {
        if hour > 23 {
            return Err(TimeError::InvalidHour(hour));
        }

        if minutes > 59 {
            return Err(TimeError::InvalidMinutes(minutes));
        }

        if seconds > 59 {
            return Err(TimeError::InvalidSeconds(seconds));
        }

        Ok(DTime {
            hour,
            minutes,
            seconds,
        })
    }

    pub fn as_seconds(&self) -> u32 {
        (self.hour as u32 * 3600) + (self.minutes as u32 * 60) + self.seconds as u32
    }

    pub fn from_seconds(seconds: u32) -> Self {
        let seconds = seconds % Self::SECONDS_PER_DAY;

        Self {
            hour: (seconds / 3600) as u8,
            minutes: ((seconds % 3600) / 60) as u8,
            seconds: (seconds % 60) as u8,
        }
    }

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minutes(&self) -> u8 {
        self.minutes
    }

    pub fn seconds(&self) -> u8 {
        self.seconds
    }
}

impl fmt::Display for DTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hour, self.minutes, self.seconds
        )
    }
}
