use std::fmt;

use super::model::{Interruption, Placement, Run, Separator, SeparatorKind};

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

impl fmt::Display for SeparatorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            SeparatorKind::Break => "Break",
            SeparatorKind::EatingBreak => "EatingBreak",
            SeparatorKind::Unassigned => "Unassigned",
            SeparatorKind::Hole => "Hole",
        };

        f.write_str(name)
    }
}

impl fmt::Display for Interruption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind(), self.duration())
    }
}

impl fmt::Display for Separator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (index, interruption) in self.interruptions().iter().enumerate() {
            if index > 0 {
                f.write_str(" + ")?;
            }

            write!(f, "{interruption}")?;
        }

        Ok(())
    }
}

impl fmt::Display for Placement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {} (", self.start(), self.end())?;

        for (index, piece) in self.pieces().iter().enumerate() {
            if index > 0 {
                f.write_str(" + ")?;
            }

            write!(f, "{piece}")?;
        }

        if !self.interruptions().is_empty() {
            f.write_str(" | ")?;

            for (index, interruption) in self.interruptions().iter().enumerate() {
                if index > 0 {
                    f.write_str(", ")?;
                }

                write!(f, "{interruption}")?;
            }
        }

        f.write_str(")")
    }
}
