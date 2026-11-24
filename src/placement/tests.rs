use crate::activity::{Activity, ActivityType, InterruptionPolicy};
use crate::structure::profile::DayProfile;
use crate::structure::{
    DayStructure, Pause, PauseKind, ScheduleTemplate, SegmentKind, TimeSegment,
};
use crate::time::{DTime, DTimeSpan};

use super::{Placement, StartPolicy, limits};

fn at(hour: u8, minutes: u8) -> DTime {
    DTime::new(hour, minutes, 0).unwrap()
}

fn minutes(value: u32) -> DTimeSpan {
    DTimeSpan::from_minutes(value)
}

fn segment(start: DTime, end: DTime, kind: SegmentKind) -> TimeSegment {
    TimeSegment::new(start, end, kind).unwrap()
}

fn academic(start: DTime, end: DTime) -> TimeSegment {
    segment(start, end, SegmentKind::Academic)
}

fn profile(segments: Vec<TimeSegment>) -> DayProfile {
    DayStructure::new(segments).unwrap().profile()
}

fn ulsa() -> DayProfile {
    ScheduleTemplate::new(
        at(8, 0),
        at(16, 40),
        minutes(50),
        2,
        vec![
            Pause::new(PauseKind::Break, minutes(20)).unwrap(),
            Pause::new(PauseKind::EatingBreak, minutes(80)).unwrap(),
            Pause::new(PauseKind::Break, minutes(20)).unwrap(),
        ],
    )
    .unwrap()
    .build()
    .profile()
}

fn class(duration: u32) -> Activity {
    Activity::new(ActivityType::Class, minutes(duration)).unwrap()
}

fn class_with(duration: u32, policy: InterruptionPolicy) -> Activity {
    class(duration).policy(policy)
}

fn breaks(max_interruptions: u32, eating_breaks: bool, min_chunk: u32) -> InterruptionPolicy {
    InterruptionPolicy::Breaks {
        max_interruptions,
        breaks: true,
        eating_breaks,
        min_chunk: minutes(min_chunk),
    }
}

fn lunch_only() -> InterruptionPolicy {
    InterruptionPolicy::Breaks {
        max_interruptions: 1,
        breaks: false,
        eating_breaks: true,
        min_chunk: minutes(0),
    }
}

fn find(profile: &DayProfile, activity: &Activity) -> Vec<Placement> {
    Placement::find(profile, activity, StartPolicy::PeriodBoundaries)
}

fn can_host(profile: &DayProfile, activity: &Activity) -> bool {
    Placement::can_host(profile, activity, StartPolicy::PeriodBoundaries)
}

fn assert_invariants(placement: &Placement, activity: &Activity) {
    let pieces: DTimeSpan = placement.pieces().iter().copied().sum();
    let crossed: DTimeSpan = placement
        .interruptions()
        .iter()
        .map(|interruption| interruption.duration())
        .sum();

    assert_eq!(pieces, activity.duration());
    assert_eq!(
        placement.pieces().len() - 1,
        placement.interruptions().len()
    );
    assert_eq!(
        placement.end() - placement.start(),
        activity.duration() + crossed
    );
}

#[test]
fn limits_describe_un_rango_de_runs() {
    let profile = ulsa();
    let activity = class_with(150, breaks(1, false, 0));

    let two = limits(&profile, at(8, 0), 0..2, &activity).unwrap();

    assert_eq!(two.pieces(), 2);
    assert_eq!(two.capacity(), minutes(200));
    assert_eq!(two.minimum(), minutes(0));
    assert_eq!(two.interruptions().len(), 1);
    assert!(two.fits(minutes(150)));

    let one = limits(&profile, at(8, 0), 0..1, &activity).unwrap();

    assert!(!one.fits(minutes(150)));
    assert!(one.fits(minutes(100)));

    assert!(limits(&profile, at(10, 0), 0..2, &activity).is_none());
}

#[test]
fn limits_aplica_min_chunk_desde_el_primer_trozo() {
    let profile = ulsa();
    let activity = class_with(150, breaks(1, false, 60));

    let window = limits(&profile, at(8, 0), 0..2, &activity).unwrap();

    assert_eq!(window.minimum(), minutes(120));
    assert!(window.fits(minutes(150)));
    assert!(limits(&profile, at(8, 50), 0..2, &activity).is_none());
}

#[test]
fn limits_rechaza_episodios_no_permitidos() {
    let profile = ulsa();

    assert!(limits(&profile, at(10, 0), 1..3, &class_with(150, lunch_only())).is_some());
    assert!(
        limits(
            &profile,
            at(10, 0),
            1..3,
            &class_with(150, breaks(1, false, 0))
        )
        .is_none()
    );
}

#[test]
fn no_alberga_una_actividad_mas_larga_que_el_run() {
    let profile = ulsa();
    let activity = class(150);

    assert!(!can_host(&profile, &activity));
    assert!(find(&profile, &activity).is_empty());
}

#[test]
fn alberga_una_conferencia_de_cien_minutos() {
    let profile = ulsa();
    let activity = Activity::new(ActivityType::Conference, minutes(100)).unwrap();
    let placements = find(&profile, &activity);

    assert!(can_host(&profile, &activity));
    assert_eq!(placements.len(), 4);
    assert_eq!(
        placements
            .iter()
            .map(|placement| placement.start())
            .collect::<Vec<DTime>>(),
        vec![at(8, 0), at(10, 0), at(13, 0), at(15, 0)]
    );

    for placement in &placements {
        assert!(placement.interruptions().is_empty());
        assert_eq!(placement.pieces(), &[minutes(100)]);
        assert_eq!(placement.end() - placement.start(), minutes(100));
        assert_invariants(placement, &activity);
    }
}

#[test]
fn una_clase_de_150_cruza_un_descanso() {
    let profile = ulsa();
    let activity = class_with(150, breaks(1, false, 0));
    let placements = find(&profile, &activity);

    assert_eq!(placements.len(), 4);

    for placement in &placements {
        assert_eq!(placement.interruptions().len(), 1);
        assert_eq!(
            placement.interruptions()[0].spans()[0].kind().to_string(),
            "Break"
        );
        assert_invariants(placement, &activity);
    }
}

#[test]
fn una_clase_de_150_no_cruza_el_almuerzo_por_defecto() {
    let profile = ulsa();
    let activity = class_with(150, breaks(1, false, 0));

    for placement in find(&profile, &activity) {
        assert!(
            placement
                .interruptions()
                .iter()
                .flat_map(|episode| episode.spans())
                .all(|span| span.kind().to_string() != "EatingBreak")
        );
    }
}

#[test]
fn solo_cruza_el_almuerzo_si_esta_habilitado() {
    let profile = ulsa();
    let flexible = class_with(150, breaks(1, true, 0));
    let adjusting = class_with(150, lunch_only());

    assert_eq!(find(&profile, &flexible).len(), 6);
    assert_eq!(find(&profile, &adjusting).len(), 2);

    for placement in find(&profile, &adjusting) {
        assert_eq!(
            placement.interruptions()[0].spans()[0].kind().to_string(),
            "EatingBreak"
        );
    }
}

#[test]
fn min_chunk_obliga_a_repartir_los_trozos() {
    let profile = ulsa();
    let activity = class_with(150, breaks(1, false, 60));
    let placements = find(&profile, &activity);

    assert_eq!(placements.len(), 2);
    assert!(
        placements
            .iter()
            .all(|placement| placement.start() == at(8, 0) || placement.start() == at(13, 0))
    );

    for placement in &placements {
        assert_eq!(placement.pieces(), &[minutes(90), minutes(60)]);
        assert!(placement.pieces().iter().all(|piece| *piece >= minutes(60)));
        assert_invariants(placement, &activity);
    }
}

#[test]
fn min_chunk_tambien_acota_la_actividad_entera() {
    let profile = ulsa();
    let activity = class_with(50, breaks(2, true, 60));

    assert!(find(&profile, &activity).is_empty());
}

#[test]
fn un_hueco_nunca_es_puente() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        academic(at(10, 0), at(10, 50)),
    ]);
    let puente = class(100).interruptible(true);

    assert!(!can_host(&profile, &puente));
    assert!(find(&profile, &puente).is_empty());
    assert_eq!(find(&profile, &class(50)).len(), 2);
}

#[test]
fn max_cuenta_episodios_y_no_spans() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        segment(at(8, 50), at(9, 10), SegmentKind::Break),
        segment(at(9, 10), at(9, 30), SegmentKind::EatingBreak),
        academic(at(9, 30), at(9, 50)),
    ]);
    let activity = class_with(70, breaks(1, true, 0));

    assert_eq!(profile.interruptions()[0].spans().len(), 2);
    assert_eq!(find(&profile, &activity).len(), 1);
    assert!(find(&profile, &class_with(70, breaks(0, true, 0))).is_empty());
    assert!(find(&profile, &class_with(70, breaks(1, false, 0))).is_empty());
}

#[test]
fn una_actividad_de_400_solo_cabe_si_es_interrumpible() {
    let profile = ulsa();

    assert!(find(&profile, &class(400)).is_empty());

    let flexible = class(400).interruptible(true);
    let placements = find(&profile, &flexible);

    assert_eq!(placements.len(), 1);
    assert_eq!(placements[0].start(), at(8, 0));
    assert_eq!(placements[0].interruptions().len(), 3);
    assert_eq!(placements[0].end(), at(16, 40));
    assert_invariants(&placements[0], &flexible);
}

#[test]
fn can_host_coincide_con_placements() {
    let profile = ulsa();
    let activities = [
        class(50),
        class(100),
        class(150),
        class_with(150, breaks(1, false, 0)),
        class(400),
        class(400).interruptible(true),
    ];

    for activity in &activities {
        assert_eq!(
            can_host(&profile, activity),
            !find(&profile, activity).is_empty()
        );
    }
}

#[test]
fn muestra_un_placement() {
    let profile = ulsa();

    assert_eq!(
        find(
            &profile,
            &Activity::new(ActivityType::Conference, minutes(100)).unwrap()
        )[0]
        .to_string(),
        "08:00 -> 09:40 (01:40:00)"
    );
    assert_eq!(
        find(&profile, &class_with(150, breaks(1, false, 0)))[0].to_string(),
        "08:00 -> 10:50 (01:40:00 + 00:50:00 | Break 00:20:00)"
    );
}
