use crate::time::DTimeSpan;

use super::error::ActivityError;
use super::policy::InterruptionPolicy;

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
    policy: InterruptionPolicy,
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
                max_interruptions: u32::MAX,
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
