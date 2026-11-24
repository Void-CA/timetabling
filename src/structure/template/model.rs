use crate::structure::{DayStructure, SegmentKind, TimeSegment};
use crate::time::{DTime, DTimeSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PauseKind {
    Break,
    EatingBreak,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pause {
    kind: PauseKind,
    duration: DTimeSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateError {
    ZeroPeriod,
    ZeroGroupSize,
    ZeroPause,
    PeriodExceedsDay { period: DTimeSpan, span: DTimeSpan },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScheduleTemplate {
    start: DTime,
    end: DTime,
    period: DTimeSpan,
    group_size: u32,
    pauses: Vec<Pause>,
}

impl Pause {
    pub fn new(kind: PauseKind, duration: DTimeSpan) -> Result<Self, TemplateError> {
        if duration.is_zero() {
            return Err(TemplateError::ZeroPause);
        }

        Ok(Self { kind, duration })
    }

    pub fn kind(&self) -> PauseKind {
        self.kind
    }

    pub fn duration(&self) -> DTimeSpan {
        self.duration
    }
}

impl ScheduleTemplate {
    pub fn new(
        start: DTime,
        end: DTime,
        period: DTimeSpan,
        group_size: u32,
        pauses: Vec<Pause>,
    ) -> Result<Self, TemplateError> {
        if period.is_zero() {
            return Err(TemplateError::ZeroPeriod);
        }

        if group_size == 0 {
            return Err(TemplateError::ZeroGroupSize);
        }

        let span = end - start;

        if period > span {
            return Err(TemplateError::PeriodExceedsDay { period, span });
        }

        Ok(Self {
            start,
            end,
            period,
            group_size,
            pauses,
        })
    }

    pub fn start(&self) -> DTime {
        self.start
    }

    pub fn end(&self) -> DTime {
        self.end
    }

    pub fn period(&self) -> DTimeSpan {
        self.period
    }

    pub fn group_size(&self) -> u32 {
        self.group_size
    }

    pub fn pauses(&self) -> &[Pause] {
        &self.pauses
    }

    pub fn declared_span(&self) -> DTimeSpan {
        self.end - self.start
    }

    pub fn build(&self) -> DayStructure {
        let span = self.declared_span();
        let group = self.period * self.group_size;

        let mut segments = Vec::new();
        let mut cursor = self.start;
        let mut elapsed = DTimeSpan::zero();
        let mut pauses = self.pauses.iter();

        loop {
            if elapsed + group > span {
                break;
            }

            for _ in 0..self.group_size {
                segments.push(academic(cursor, self.period));
                cursor = cursor + self.period;
                elapsed = elapsed + self.period;
            }

            let Some(pause) = pauses.next() else {
                break;
            };

            if elapsed + pause.duration() > span {
                break;
            }

            segments.push(segment(
                cursor,
                cursor + pause.duration(),
                pause.kind().into(),
            ));
            cursor = cursor + pause.duration();
            elapsed = elapsed + pause.duration();
        }

        if cursor != self.end {
            segments.push(segment(cursor, self.end, SegmentKind::Other));
        }

        DayStructure::new(segments).expect("la plantilla genera una estructura válida")
    }
}

fn academic(start: DTime, duration: DTimeSpan) -> TimeSegment {
    segment(start, start + duration, SegmentKind::Academic)
}

fn segment(start: DTime, end: DTime, kind: SegmentKind) -> TimeSegment {
    TimeSegment::new(start, end, kind).expect("el tramo generado es válido")
}
