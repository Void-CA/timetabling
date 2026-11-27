use crate::structure::interruption::Interruption;
use crate::time::{DTime, DTimeSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StartPolicy {
    PeriodBoundaries,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placement {
    start: DTime,
    end: DTime,
    pieces: Vec<DTimeSpan>,
    interruptions: Vec<Interruption>,
}

impl Placement {
    pub(crate) fn new(
        start: DTime,
        end: DTime,
        pieces: Vec<DTimeSpan>,
        interruptions: Vec<Interruption>,
    ) -> Self {
        Self {
            start,
            end,
            pieces,
            interruptions,
        }
    }

    pub fn start(&self) -> DTime {
        self.start
    }

    pub fn end(&self) -> DTime {
        self.end
    }

    pub fn pieces(&self) -> &[DTimeSpan] {
        &self.pieces
    }

    pub fn interruptions(&self) -> &[Interruption] {
        &self.interruptions
    }
}
