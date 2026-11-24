use crate::activity::{Activity, InterruptionPolicy};
use crate::structure::{DayStructure, SegmentKind};
use crate::time::{DTime, DTimeSpan};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Run {
    start: DTime,
    end: DTime,
    duration: DTimeSpan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeparatorKind {
    Break,
    EatingBreak,
    Unassigned,
    Hole,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interruption {
    kind: SeparatorKind,
    duration: DTimeSpan,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Separator {
    interruptions: Vec<Interruption>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DayProfile {
    runs: Vec<Run>,
    separators: Vec<Separator>,
    candidate_starts: Vec<DTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placement {
    start: DTime,
    end: DTime,
    pieces: Vec<DTimeSpan>,
    interruptions: Vec<Interruption>,
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

impl Interruption {
    fn new(kind: SeparatorKind, duration: DTimeSpan) -> Self {
        Self { kind, duration }
    }

    pub fn kind(&self) -> SeparatorKind {
        self.kind
    }

    pub fn duration(&self) -> DTimeSpan {
        self.duration
    }
}

impl Separator {
    pub fn interruptions(&self) -> &[Interruption] {
        &self.interruptions
    }

    pub fn duration(&self) -> DTimeSpan {
        self.interruptions.iter().map(Interruption::duration).sum()
    }
}

impl DayProfile {
    pub fn from_structure(day: &DayStructure) -> Self {
        let segments = day.segments();
        let mut runs = Vec::new();
        let mut ranges = Vec::new();
        let mut candidate_starts = Vec::new();

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
                candidate_starts.push(segment.start_time());
            }

            runs.push(Run::new(
                segments[first].start_time(),
                segments[last].end_time(),
            ));
            ranges.push((first, last));
            index = last + 1;
        }

        let mut separators = Vec::with_capacity(runs.len().saturating_sub(1));

        for index in 0..runs.len().saturating_sub(1) {
            let previous_last = ranges[index].1;
            let next_first = ranges[index + 1].0;
            let mut interruptions = Vec::new();
            let mut cursor = runs[index].end();

            for segment in &segments[previous_last + 1..next_first] {
                push_gap(&mut interruptions, segment.start_time() - cursor);
                push_interruption(&mut interruptions, segment.kind(), segment.duration());
                cursor = segment.end_time();
            }

            push_gap(&mut interruptions, runs[index + 1].start() - cursor);

            debug_assert!(!interruptions.is_empty());

            separators.push(Separator { interruptions });
        }

        Self {
            runs,
            separators,
            candidate_starts,
        }
    }

    pub fn runs(&self) -> &[Run] {
        &self.runs
    }

    pub fn separators(&self) -> &[Separator] {
        &self.separators
    }

    pub fn candidate_starts(&self) -> &[DTime] {
        &self.candidate_starts
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

    pub fn can_host(&self, activity: &Activity) -> bool {
        self.iter_placements(activity).next().is_some()
    }

    pub fn placements(&self, activity: &Activity) -> Vec<Placement> {
        self.iter_placements(activity).collect()
    }

    fn iter_placements<'a>(
        &'a self,
        activity: &'a Activity,
    ) -> impl Iterator<Item = Placement> + 'a {
        self.candidate_starts
            .iter()
            .filter_map(move |start| self.placement_at(*start, activity))
    }

    fn placement_at(&self, start: DTime, activity: &Activity) -> Option<Placement> {
        let (run_index, first_capacity) = self.locate(start)?;
        let duration = activity.duration();
        let (max_spans, breaks, eating_breaks, min_chunk) = limits(activity.interruption_policy());

        if first_capacity < min_chunk {
            return None;
        }

        let mut capacities = vec![first_capacity];
        let mut capacity = first_capacity;
        let mut interruptions: Vec<Interruption> = Vec::new();
        let mut intro_spans: u32 = 0;
        let mut index = run_index;

        loop {
            let pieces = capacities.len() as u32;

            if duration <= capacity {
                if duration < min_chunk * pieces {
                    return None;
                }

                let pieces = distribute(&capacities, duration, min_chunk);
                let crossed: DTimeSpan = interruptions.iter().map(Interruption::duration).sum();

                return Some(Placement {
                    start,
                    end: start + duration + crossed,
                    pieces,
                    interruptions,
                });
            }

            if index + 1 >= self.runs.len() {
                return None;
            }

            let separator = &self.separators[index];
            let noticed = separator.interruptions().len() as u32;

            if intro_spans.saturating_add(noticed) > max_spans {
                return None;
            }

            if !separator
                .interruptions()
                .iter()
                .all(|interruption| allows(interruption.kind(), breaks, eating_breaks))
            {
                return None;
            }

            let next_capacity = self.runs[index + 1].duration();

            if next_capacity < min_chunk {
                return None;
            }

            intro_spans += noticed;
            interruptions.extend(separator.interruptions().iter().copied());
            capacities.push(next_capacity);
            capacity = capacity + next_capacity;
            index += 1;
        }
    }

    fn locate(&self, start: DTime) -> Option<(usize, DTimeSpan)> {
        self.runs.iter().enumerate().find_map(|(index, run)| {
            if run.contains(start) {
                Some((index, run.end() - start))
            } else {
                None
            }
        })
    }
}

impl Placement {
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

impl DayStructure {
    pub fn profile(&self) -> DayProfile {
        DayProfile::from_structure(self)
    }
}

fn push_interruption(
    interruptions: &mut Vec<Interruption>,
    kind: SegmentKind,
    duration: DTimeSpan,
) {
    let kind = match kind {
        SegmentKind::Break => SeparatorKind::Break,
        SegmentKind::EatingBreak => SeparatorKind::EatingBreak,
        SegmentKind::Unassigned => SeparatorKind::Unassigned,
        SegmentKind::Academic => {
            debug_assert!(false, "un separador no contiene tramos académicos");
            return;
        }
    };

    interruptions.push(Interruption::new(kind, duration));
}

fn push_gap(interruptions: &mut Vec<Interruption>, gap: DTimeSpan) {
    if !gap.is_zero() {
        interruptions.push(Interruption::new(SeparatorKind::Hole, gap));
    }
}

fn limits(policy: InterruptionPolicy) -> (u32, bool, bool, DTimeSpan) {
    match policy {
        InterruptionPolicy::None => (0, false, false, DTimeSpan::zero()),
        InterruptionPolicy::Breaks {
            max,
            breaks,
            eating_breaks,
            min_chunk,
        } => (max, breaks, eating_breaks, min_chunk),
    }
}

fn allows(kind: SeparatorKind, breaks: bool, eating_breaks: bool) -> bool {
    match kind {
        SeparatorKind::Break => breaks,
        SeparatorKind::EatingBreak => eating_breaks,
        SeparatorKind::Unassigned | SeparatorKind::Hole => false,
    }
}

fn distribute(capacities: &[DTimeSpan], total: DTimeSpan, min_chunk: DTimeSpan) -> Vec<DTimeSpan> {
    let mut remaining = total;
    let mut pieces = Vec::with_capacity(capacities.len());

    for (index, capacity) in capacities.iter().enumerate() {
        let rest = (capacities.len() - index - 1) as u32;
        let floor = min_chunk * rest;
        let room = remaining - floor;
        let piece = if *capacity < room { *capacity } else { room };

        pieces.push(piece);
        remaining = remaining - piece;
    }

    debug_assert!(remaining.is_zero());

    pieces
}
