use crate::time::{DTime, DTimeSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SegmentKind {
    Academic,
    Break,
    EatingBreak,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeSegment {
    start_time: DTime,
    end_time: DTime,
    kind: SegmentKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SegmentError {
    EmptySegment,
}

impl TimeSegment {
    pub fn new(
        start_time: DTime,
        end_time: DTime,
        kind: SegmentKind,
    ) -> Result<Self, SegmentError> {
        let segment = Self {
            start_time,
            end_time,
            kind,
        };

        if segment.duration().is_zero() {
            return Err(SegmentError::EmptySegment);
        }

        Ok(segment)
    }

    pub fn start_time(&self) -> DTime {
        self.start_time
    }

    pub fn end_time(&self) -> DTime {
        self.end_time
    }

    pub fn kind(&self) -> SegmentKind {
        self.kind
    }

    pub fn duration(&self) -> DTimeSpan {
        self.end_time - self.start_time
    }

    pub fn is_academic(&self) -> bool {
        self.kind == SegmentKind::Academic
    }
}
