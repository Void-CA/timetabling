use std::fmt;

use super::model::Run;

impl fmt::Display for Run {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {} ({})",
            self.start(),
            self.end(),
            self.duration()
        )
    }
}
