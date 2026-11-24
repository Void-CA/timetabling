use crate::activity::Activity;
use crate::structure::profile::DayProfile;
use crate::time::{DTime, DTimeSpan};

use super::compatibility::limits;
use super::model::{Placement, StartPolicy};

impl Placement {
    pub fn find(profile: &DayProfile, activity: &Activity, starts: StartPolicy) -> Vec<Placement> {
        Self::iter(profile, activity, starts).collect()
    }

    pub fn can_host(profile: &DayProfile, activity: &Activity, starts: StartPolicy) -> bool {
        Self::iter(profile, activity, starts).next().is_some()
    }

    fn iter<'a>(
        profile: &'a DayProfile,
        activity: &'a Activity,
        policy: StartPolicy,
    ) -> impl Iterator<Item = Placement> + 'a {
        let starts = match policy {
            StartPolicy::PeriodBoundaries => profile.academic_starts(),
        };

        starts
            .iter()
            .copied()
            .filter_map(move |start| Self::at(profile, start, activity))
    }

    fn at(profile: &DayProfile, start: DTime, activity: &Activity) -> Option<Placement> {
        let run_index = profile.run_index_at(start)?;
        let duration = activity.duration();

        for end in run_index + 1..=profile.runs().len() {
            let limits = limits(profile, start, run_index..end, activity)?;

            if limits.fits(duration) {
                let pieces = distribute(limits.capacities(), duration, limits.min_chunk());
                let crossed: DTimeSpan = limits
                    .interruptions()
                    .iter()
                    .map(|interruption| interruption.duration())
                    .sum();

                return Some(Placement::new(
                    start,
                    start + duration + crossed,
                    pieces,
                    limits.interruptions().to_vec(),
                ));
            }

            if duration < limits.minimum() {
                return None;
            }
        }

        None
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
