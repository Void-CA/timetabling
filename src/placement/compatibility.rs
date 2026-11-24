use std::ops::Range;

use crate::activity::{Activity, InterruptionPolicy};
use crate::structure::profile::{DayProfile, Interruption, InterruptionSpanKind};
use crate::time::{DTime, DTimeSpan};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limits {
    capacities: Vec<DTimeSpan>,
    capacity: DTimeSpan,
    minimum: DTimeSpan,
    min_chunk: DTimeSpan,
    interruptions: Vec<Interruption>,
}

impl Limits {
    pub fn pieces(&self) -> u32 {
        self.capacities.len() as u32
    }

    pub fn capacities(&self) -> &[DTimeSpan] {
        &self.capacities
    }

    pub fn capacity(&self) -> DTimeSpan {
        self.capacity
    }

    pub fn minimum(&self) -> DTimeSpan {
        self.minimum
    }

    pub fn min_chunk(&self) -> DTimeSpan {
        self.min_chunk
    }

    pub fn interruptions(&self) -> &[Interruption] {
        &self.interruptions
    }

    pub fn fits(&self, duration: DTimeSpan) -> bool {
        self.minimum <= duration && duration <= self.capacity
    }
}

pub fn limits(
    profile: &DayProfile,
    start: DTime,
    runs: Range<usize>,
    activity: &Activity,
) -> Option<Limits> {
    if runs.start >= runs.end || runs.end > profile.runs().len() {
        return None;
    }

    if profile.run_index_at(start) != Some(runs.start) {
        return None;
    }

    let (max_interruptions, breaks, eating_breaks, min_chunk) =
        policy_limits(activity.interruption_policy());

    let mut capacities = Vec::with_capacity(runs.len());
    let first = profile.runs()[runs.start].end() - start;

    if first < min_chunk {
        return None;
    }

    capacities.push(first);

    for index in runs.start + 1..runs.end {
        let duration = profile.runs()[index].duration();

        if duration < min_chunk {
            return None;
        }

        capacities.push(duration);
    }

    let interruptions = profile.interruptions()[runs.start..runs.end - 1].to_vec();

    if interruptions.len() as u32 > max_interruptions {
        return None;
    }

    let allowed = interruptions.iter().all(|interruption| {
        interruption
            .spans()
            .iter()
            .all(|span| allows(span.kind(), breaks, eating_breaks))
    });

    if !allowed {
        return None;
    }

    let capacity = capacities.iter().copied().sum();
    let minimum = min_chunk * capacities.len() as u32;

    Some(Limits {
        capacities,
        capacity,
        minimum,
        min_chunk,
        interruptions,
    })
}

fn policy_limits(policy: InterruptionPolicy) -> (u32, bool, bool, DTimeSpan) {
    match policy {
        InterruptionPolicy::None => (0, false, false, DTimeSpan::zero()),
        InterruptionPolicy::Breaks {
            max_interruptions,
            breaks,
            eating_breaks,
            min_chunk,
        } => (max_interruptions, breaks, eating_breaks, min_chunk),
    }
}

fn allows(kind: InterruptionSpanKind, breaks: bool, eating_breaks: bool) -> bool {
    match kind {
        InterruptionSpanKind::Break => breaks,
        InterruptionSpanKind::EatingBreak => eating_breaks,
        InterruptionSpanKind::Unassigned | InterruptionSpanKind::Hole => false,
    }
}
