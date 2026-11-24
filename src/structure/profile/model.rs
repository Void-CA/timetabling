use crate::structure::{DayStructure, SegmentKind};
use crate::time::{DTime, DTimeSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
    start: DTime,
    end: DTime,
    duration: DTimeSpan,
}

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayProfile {
    runs: Vec<Run>,
    interruptions: Vec<Interruption>,
    academic_starts: Vec<DTime>,
}

impl Run {
    fn new(start: DTime, end: DTime) -> Self {
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

    fn contains(&self, time: DTime) -> bool {
        (time - self.start) < self.duration
    }
}

impl InterruptionSpan {
    fn new(kind: InterruptionSpanKind, duration: DTimeSpan) -> Self {
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
    fn new(spans: Vec<InterruptionSpan>) -> Self {
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

impl DayProfile {
    pub fn from_structure(day: &DayStructure) -> Self {
        let segments = day.segments();
        let mut runs = Vec::new();
        let mut ranges = Vec::new();
        let mut academic_starts = Vec::new();

        let mut index = 0;

        while index < segments.len() {
            if !segments[index].is_academic() {
                index += 1;
                continue;
            }

            let first = index;
            let mut last = index;

            while last + 1 < segments.len()
                && segments[last + 1].is_academic()
                && segments[last + 1].start_time() == segments[last].end_time()
            {
                last += 1;
            }

            for segment in &segments[first..=last] {
                academic_starts.push(segment.start_time());
            }

            runs.push(Run::new(
                segments[first].start_time(),
                segments[last].end_time(),
            ));
            ranges.push((first, last));
            index = last + 1;
        }

        let mut interruptions = Vec::with_capacity(runs.len().saturating_sub(1));

        for index in 0..runs.len().saturating_sub(1) {
            let previous_last = ranges[index].1;
            let next_first = ranges[index + 1].0;
            let mut spans = Vec::new();
            let mut cursor = runs[index].end();

            for segment in &segments[previous_last + 1..next_first] {
                push_gap(&mut spans, segment.start_time() - cursor);
                push_span(&mut spans, segment.kind(), segment.duration());
                cursor = segment.end_time();
            }

            push_gap(&mut spans, runs[index + 1].start() - cursor);

            interruptions.push(Interruption::new(spans));
        }

        Self {
            runs,
            interruptions,
            academic_starts,
        }
    }

    pub fn runs(&self) -> &[Run] {
        &self.runs
    }

    pub fn interruptions(&self) -> &[Interruption] {
        &self.interruptions
    }

    pub fn academic_starts(&self) -> &[DTime] {
        &self.academic_starts
    }

    pub fn run_index_at(&self, time: DTime) -> Option<usize> {
        self.runs.iter().position(|run| run.contains(time))
    }

    pub fn longest_run(&self) -> Option<&Run> {
        self.runs.iter().max_by_key(|run| run.duration())
    }

    pub fn run_lengths(&self) -> impl Iterator<Item = DTimeSpan> + '_ {
        self.runs.iter().map(Run::duration)
    }

    pub fn mean_run(&self) -> Option<DTimeSpan> {
        if self.runs.is_empty() {
            return None;
        }

        let total: DTimeSpan = self.run_lengths().sum();

        Some(total / self.runs.len() as u32)
    }
}

impl DayStructure {
    pub fn profile(&self) -> DayProfile {
        DayProfile::from_structure(self)
    }
}

fn push_span(spans: &mut Vec<InterruptionSpan>, kind: SegmentKind, duration: DTimeSpan) {
    let kind = match kind {
        SegmentKind::Break => InterruptionSpanKind::Break,
        SegmentKind::EatingBreak => InterruptionSpanKind::EatingBreak,
        SegmentKind::Unassigned => InterruptionSpanKind::Unassigned,
        SegmentKind::Academic => {
            debug_assert!(false, "un episodio no contiene tramos académicos");
            return;
        }
    };

    spans.push(InterruptionSpan::new(kind, duration));
}

fn push_gap(spans: &mut Vec<InterruptionSpan>, gap: DTimeSpan) {
    if !gap.is_zero() {
        spans.push(InterruptionSpan::new(InterruptionSpanKind::Hole, gap));
    }
}
