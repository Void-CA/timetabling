use crate::time::{DTime, DTimeSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
    start: DTime,
    end: DTime,
    duration: DTimeSpan,
}

impl Run {
    pub(crate) fn new(start: DTime, end: DTime) -> Self {
        Self {
            start,
            end,
            duration: end - start,
        }
    }

    pub fn start(&self) -> DTime {
        self.start
    }

    pub fn end(&self) -> DTime {
        self.end
    }

    pub fn duration(&self) -> DTimeSpan {
        self.duration
    }

    pub(crate) fn contains(&self, time: DTime) -> bool {
        (time - self.start) < self.duration
    }
}
