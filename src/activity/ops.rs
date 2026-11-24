use std::fmt;

use crate::time::DTimeSpan;

use super::model::{Activity, ActivityError, ActivityType};

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

impl std::error::Error for ActivityError {}

impl TryFrom<(ActivityType, DTimeSpan)> for Activity {
    type Error = ActivityError;

    fn try_from((kind, duration): (ActivityType, DTimeSpan)) -> Result<Self, Self::Error> {
        Activity::new(kind, duration)
    }
}
