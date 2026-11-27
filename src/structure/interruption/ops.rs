use std::fmt;

use super::model::{Interruption, InterruptionSpan, InterruptionSpanKind};

impl fmt::Display for InterruptionSpanKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            InterruptionSpanKind::Break => "Break",
            InterruptionSpanKind::EatingBreak => "EatingBreak",
            InterruptionSpanKind::Unassigned => "Unassigned",
            InterruptionSpanKind::Hole => "Hole",
        };

        f.write_str(name)
    }
}

impl fmt::Display for InterruptionSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind(), self.duration())
    }
}

impl fmt::Display for Interruption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, span) in self.spans().iter().enumerate() {
            if index > 0 {
                f.write_str(" + ")?;
            }

            write!(f, "{span}")?;
        }

        Ok(())
    }
}
