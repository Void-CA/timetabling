use super::timesegment::TimeSegment;
use crate::time::{DTime, DTimeSpan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructureError {
    EmptyStructure,
    Overlapping { previous: usize, next: usize },
    ExceedsDay { duration: DTimeSpan },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayStructure {
    segments: Vec<TimeSegment>,
}

impl DayStructure {
    pub fn new(segments: Vec<TimeSegment>) -> Result<Self, StructureError> {
        if segments.is_empty() {
            return Err(StructureError::EmptyStructure);
        }

        let mut elapsed = DTimeSpan::zero();

        for (index, pair) in segments.windows(2).enumerate() {
            let step = pair[1].start_time() - pair[0].start_time();

            if step < pair[0].duration() {
                return Err(StructureError::Overlapping {
                    previous: index,
                    next: index + 1,
                });
            }

            elapsed = elapsed + step;

            if elapsed.as_seconds() >= DTimeSpan::SECONDS_PER_DAY {
                return Err(StructureError::ExceedsDay {
                    duration: elapsed + pair[1].duration(),
                });
            }
        }

        let last = segments[segments.len() - 1];
        let total = elapsed + last.duration();

        if total.as_seconds() > DTimeSpan::SECONDS_PER_DAY {
            return Err(StructureError::ExceedsDay { duration: total });
        }

        Ok(Self { segments })
    }

    pub fn segments(&self) -> &[TimeSegment] {
        &self.segments
    }

    pub fn starts_at(&self) -> DTime {
        self.segments[0].start_time()
    }

    pub fn ends_at(&self) -> DTime {
        self.segments[self.segments.len() - 1].end_time()
    }

    pub fn total_span(&self) -> DTimeSpan {
        let first = self.segments[0].start_time();
        let last = self.segments[self.segments.len() - 1];

        (last.start_time() - first) + last.duration()
    }

    pub fn academic_segments(&self) -> impl Iterator<Item = &TimeSegment> {
        self.segments.iter().filter(|segment| segment.is_academic())
    }

    pub fn academic_capacity(&self) -> DTimeSpan {
        self.academic_segments().map(TimeSegment::duration).sum()
    }
}
