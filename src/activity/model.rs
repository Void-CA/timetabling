use crate::time::DTimeSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ActivityType {
    Class,
    Conference,
    Meeting,
    Lab,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptionPolicy {
    None,
    Breaks {
        max: u32,
        breaks: bool,
        eating_breaks: bool,
        min_chunk: DTimeSpan,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Activity {
    kind: ActivityType,
    duration: DTimeSpan,
    policy: InterruptionPolicy,
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
            policy: InterruptionPolicy::None,
        })
    }

    pub fn interruptible(mut self, interruptible: bool) -> Self {
        self.policy = if interruptible {
            InterruptionPolicy::Breaks {
                max: u32::MAX,
                breaks: true,
                eating_breaks: true,
                min_chunk: DTimeSpan::zero(),
            }
        } else {
            InterruptionPolicy::None
        };

        self
    }

    pub fn policy(mut self, policy: InterruptionPolicy) -> Self {
        self.policy = policy;
        self
    }

    pub fn kind(&self) -> ActivityType {
        self.kind
    }

    pub fn duration(&self) -> DTimeSpan {
        self.duration
    }

    pub fn interruption_policy(&self) -> InterruptionPolicy {
        self.policy
    }

    pub fn is_interruptible(&self) -> bool {
        self.policy != InterruptionPolicy::None
    }
}
