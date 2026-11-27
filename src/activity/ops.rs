use std::fmt;

use crate::time::DTimeSpan;

use super::error::ActivityError;
use super::model::{Activity, ActivityType};
use super::policy::InterruptionPolicy;

impl fmt::Display for ActivityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ActivityType::Class => "Class",
            ActivityType::Conference => "Conference",
            ActivityType::Meeting => "Meeting",
            ActivityType::Lab => "Lab",
        };

        f.write_str(name)
    }
}

impl fmt::Display for Activity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind(), self.duration())?;

        if self.is_interruptible() {
            f.write_str(" (interruptible)")?;
        }

        Ok(())
    }
}

impl fmt::Display for ActivityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ActivityError::EmptyDuration => {
                f.write_str("la duración de una actividad no puede ser cero")
            }
        }
    }
}

impl fmt::Display for InterruptionPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterruptionPolicy::None => f.write_str("None"),
            InterruptionPolicy::Breaks {
                max_interruptions,
                breaks,
                eating_breaks,
                min_chunk,
            } => {
                let max = if *max_interruptions == u32::MAX {
                    "unbounded".to_string()
                } else {
                    max_interruptions.to_string()
                };

                write!(
                    f,
                    "Breaks max_interruptions={max} breaks={breaks} eating_breaks={eating_breaks} min_chunk={min_chunk}"
                )
            }
        }
    }
}

impl std::error::Error for ActivityError {}

impl TryFrom<(ActivityType, DTimeSpan)> for Activity {
    type Error = ActivityError;

    fn try_from((kind, duration): (ActivityType, DTimeSpan)) -> Result<Self, Self::Error> {
        Activity::new(kind, duration)
    }
}
