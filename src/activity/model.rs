use crate::time::DTimeSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActivityType {
    Class,
    Conference,
    Meeting,
    Lab,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Activity {
    kind: ActivityType,
    duration: DTimeSpan,
    interruptible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActivityError {
    EmptyDuration,
}

impl Activity {
    pub fn new(kind: ActivityType, duration: DTimeSpan) -> Result<Self, ActivityError> {
        if duration.is_zero() {
            return Err(ActivityError::EmptyDuration);
        }

        Ok(Self {
            kind,
            duration,
            interruptible: false,
        })
    }

    pub fn interruptible(mut self, interruptible: bool) -> Self {
        self.interruptible = interruptible;
        self
    }

    pub fn kind(&self) -> ActivityType {
        self.kind
    }

    pub fn duration(&self) -> DTimeSpan {
        self.duration
    }

    pub fn is_interruptible(&self) -> bool {
        self.interruptible
    }
}
