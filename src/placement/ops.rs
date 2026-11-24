use std::fmt;

use super::model::Placement;

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
                    f.write_str("; ")?;
                }

                write!(f, "{interruption}")?;
            }
        }

        f.write_str(")")
    }
}
