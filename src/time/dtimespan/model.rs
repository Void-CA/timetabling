use std::fmt;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DTimeSpan {
    seconds: u32,
}

impl DTimeSpan {
    pub const SECONDS_PER_MINUTE: u32 = 60;
    pub const SECONDS_PER_HOUR: u32 = 3_600;
    pub const SECONDS_PER_DAY: u32 = 86_400;

    pub const fn from_seconds(seconds: u32) -> Self {
        Self { seconds }
    }

    pub const fn from_minutes(minutes: u32) -> Self {
        Self {
            seconds: minutes * Self::SECONDS_PER_MINUTE,
        }
    }

    pub const fn from_hours(hours: u32) -> Self {
        Self {
            seconds: hours * Self::SECONDS_PER_HOUR,
        }
    }

    pub const fn from_days(days: u32) -> Self {
        Self {
            seconds: days * Self::SECONDS_PER_DAY,
        }
    }

    pub const fn zero() -> Self {
        Self { seconds: 0 }
    }

    pub const fn as_seconds(&self) -> u32 {
        self.seconds
    }

    pub const fn as_minutes(&self) -> u32 {
        self.seconds / Self::SECONDS_PER_MINUTE
    }

    pub const fn as_hours(&self) -> u32 {
        self.seconds / Self::SECONDS_PER_HOUR
    }

    pub const fn is_zero(&self) -> bool {
        self.seconds == 0
    }

    pub const fn hours(&self) -> u32 {
        self.seconds / Self::SECONDS_PER_HOUR
    }

    pub const fn minutes(&self) -> u32 {
        (self.seconds % Self::SECONDS_PER_HOUR) / Self::SECONDS_PER_MINUTE
    }

    pub const fn seconds(&self) -> u32 {
        self.seconds % Self::SECONDS_PER_MINUTE
    }

    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.seconds.checked_sub(rhs.seconds) {
            Some(seconds) => Some(Self { seconds }),
            None => None,
        }
    }

    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self {
            seconds: self.seconds.saturating_sub(rhs.seconds),
        }
    }
}

impl fmt::Display for DTimeSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}",
            self.hours(),
            self.minutes(),
            self.seconds()
        )
    }
}
