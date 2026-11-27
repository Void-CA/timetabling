use crate::time::DTimeSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptionSpanKind {
    Break,
    EatingBreak,
    Unassigned,
    Hole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterruptionSpan {
    kind: InterruptionSpanKind,
    duration: DTimeSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interruption {
    spans: Vec<InterruptionSpan>,
}

impl InterruptionSpan {
    pub(crate) fn new(kind: InterruptionSpanKind, duration: DTimeSpan) -> Self {
        Self { kind, duration }
    }

    pub fn kind(&self) -> InterruptionSpanKind {
        self.kind
    }

    pub fn duration(&self) -> DTimeSpan {
        self.duration
    }
}

impl Interruption {
    pub(crate) fn new(spans: Vec<InterruptionSpan>) -> Self {
        debug_assert!(!spans.is_empty());

        Self { spans }
    }

    pub fn spans(&self) -> &[InterruptionSpan] {
        &self.spans
    }

    pub fn duration(&self) -> DTimeSpan {
        self.spans.iter().map(InterruptionSpan::duration).sum()
    }
}
