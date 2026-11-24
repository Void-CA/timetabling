use std::fmt;

use super::model::{SegmentError, SegmentKind, TimeSegment};

impl fmt::Display for SegmentKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            SegmentKind::Academic => "Academic",
            SegmentKind::Break => "Break",
            SegmentKind::EatingBreak => "EatingBreak",
            SegmentKind::Other => "Other",
        };

        f.write_str(name)
    }
}

impl fmt::Display for TimeSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} -> {} ({})",
            self.start_time(),
            self.end_time(),
            self.kind()
        )
    }
}

impl fmt::Display for SegmentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SegmentError::EmptySegment => f.write_str("un tramo no puede tener duración cero"),
        }
    }
}

impl std::error::Error for SegmentError {}
