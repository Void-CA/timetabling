use crate::activity::{Activity, ActivityType, InterruptionPolicy};
use crate::structure::{
    DayStructure, Pause, PauseKind, ScheduleTemplate, SegmentKind, TimeSegment,
};
use crate::time::{DTime, DTimeSpan};

use super::{DayProfile, SeparatorKind};

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

fn breaks(max: u32, eating_breaks: bool, min_chunk: u32) -> InterruptionPolicy {
    InterruptionPolicy::Breaks {
        max,
        breaks: true,
        eating_breaks,
        min_chunk: minutes(min_chunk),
    }
}

#[test]
fn ulsa_produce_cuatro_runs_de_cien_minutos() {
    let profile = ulsa();

    assert_eq!(profile.runs().len(), 4);
    assert_eq!(
        profile
            .run_lengths()
            .map(|span| span.as_minutes())
            .collect::<Vec<u32>>(),
        vec![100, 100, 100, 100]
    );
    assert_eq!(profile.longest_run().unwrap().duration(), minutes(100));
    assert_eq!(profile.mean_run(), Some(minutes(100)));
    assert_eq!(profile.runs()[0].start(), at(8, 0));
    assert_eq!(profile.runs()[0].end(), at(9, 40));
    assert_eq!(profile.runs()[3].start(), at(15, 0));
    assert_eq!(profile.runs()[3].end(), at(16, 40));
}

#[test]
fn ulsa_separa_los_runs_con_sus_pausas() {
    let profile = ulsa();

    assert_eq!(profile.separators().len(), profile.runs().len() - 1);
    assert_eq!(
        profile
            .separators()
            .iter()
            .map(|separator| separator.duration().as_minutes())
            .collect::<Vec<u32>>(),
        vec![20, 80, 20]
    );
    assert!(
        profile
            .separators()
            .iter()
            .all(|separator| separator.interruptions().len() == 1)
    );
    assert_eq!(
        profile
            .separators()
            .iter()
            .flat_map(|separator| separator.interruptions())
            .map(|interruption| interruption.kind())
            .collect::<Vec<SeparatorKind>>(),
        vec![
            SeparatorKind::Break,
            SeparatorKind::EatingBreak,
            SeparatorKind::Break
        ]
    );
}

#[test]
fn conserva_los_inicios_candidatos() {
    let profile = ulsa();
    let starts = profile.candidate_starts();

    assert_eq!(starts.len(), 8);
    assert_eq!(starts[0], at(8, 0));
    assert_eq!(starts[7], at(15, 50));
    assert!(starts.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(starts.iter().all(|start| {
        profile
            .runs()
            .iter()
            .any(|run| run.start() <= *start && *start < run.end())
    }));
}

#[test]
fn fusiona_segmentos_academicos_contiguos() {
    let profile = profile(vec![
        academic(at(8, 0), at(9, 40)),
        academic(at(9, 40), at(11, 20)),
    ]);

    assert_eq!(profile.runs().len(), 1);
    assert_eq!(profile.runs()[0].duration(), minutes(200));
    assert_eq!(profile.candidate_starts().len(), 2);
    assert!(profile.separators().is_empty());
}

#[test]
fn detecta_huecos_entre_runs() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        academic(at(10, 0), at(10, 50)),
    ]);

    assert_eq!(profile.runs().len(), 2);
    assert_eq!(profile.separators().len(), 1);
    assert_eq!(profile.separators()[0].interruptions().len(), 1);
    assert_eq!(
        profile.separators()[0].interruptions()[0].kind(),
        SeparatorKind::Hole
    );
    assert_eq!(
        profile.separators()[0].interruptions()[0].duration(),
        minutes(70)
    );
    assert_eq!(profile.separators()[0].to_string(), "Hole 01:10:00");
}

#[test]
fn distingue_unassigned_de_un_hueco() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        segment(at(8, 50), at(9, 30), SegmentKind::Unassigned),
        academic(at(9, 30), at(10, 20)),
    ]);

    assert_eq!(profile.separators().len(), 1);
    assert_eq!(
        profile.separators()[0].interruptions()[0].kind(),
        SeparatorKind::Unassigned
    );
    assert_eq!(profile.separators()[0].duration(), minutes(40));
}

#[test]
fn soporta_separadores_compuestos() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        segment(at(8, 50), at(9, 10), SegmentKind::Break),
        segment(at(9, 10), at(9, 30), SegmentKind::EatingBreak),
        academic(at(9, 30), at(10, 20)),
    ]);

    let separator = &profile.separators()[0];

    assert_eq!(separator.interruptions().len(), 2);
    assert_eq!(separator.duration(), minutes(40));
    assert_eq!(
        separator.to_string(),
        "Break 00:20:00 + EatingBreak 00:20:00"
    );
}

#[test]
fn acepta_una_estructura_sin_runs() {
    let profile = profile(vec![segment(at(8, 0), at(8, 20), SegmentKind::Break)]);

    assert!(profile.runs().is_empty());
    assert!(profile.separators().is_empty());
    assert!(profile.candidate_starts().is_empty());
    assert!(profile.longest_run().is_none());
    assert!(profile.mean_run().is_none());
    assert_eq!(profile.run_lengths().count(), 0);
    assert!(!profile.can_host(&class(50)));
}

#[test]
fn calcula_runs_cruzando_medianoche() {
    let profile = profile(vec![
        academic(at(22, 0), at(23, 30)),
        academic(at(23, 30), at(1, 0)),
        segment(at(1, 0), at(1, 30), SegmentKind::Break),
        academic(at(1, 30), at(4, 30)),
    ]);

    assert_eq!(profile.runs().len(), 2);
    assert_eq!(
        profile
            .run_lengths()
            .map(|span| span.as_minutes())
            .collect::<Vec<u32>>(),
        vec![180, 180]
    );
    assert_eq!(profile.separators()[0].duration(), minutes(30));
    assert_eq!(
        profile.candidate_starts(),
        &[at(22, 0), at(23, 30), at(1, 30)]
    );
}

#[test]
fn muestra_runs_y_separadores() {
    let profile = profile(vec![
        academic(at(8, 0), at(9, 40)),
        segment(at(9, 40), at(10, 0), SegmentKind::Break),
        academic(at(10, 0), at(10, 50)),
    ]);

    assert_eq!(profile.runs()[0].to_string(), "08:00 -> 09:40 (01:40:00)");
    assert_eq!(profile.separators()[0].to_string(), "Break 00:20:00");
}

#[test]
fn no_alberga_una_actividad_mas_larga_que_el_run() {
    let profile = ulsa();
    let activity = class(150);

    assert!(!profile.can_host(&activity));
    assert!(profile.placements(&activity).is_empty());
}

#[test]
fn alberga_una_conferencia_de_cien_minutos() {
    let profile = ulsa();
    let activity = Activity::new(ActivityType::Conference, minutes(100)).unwrap();
    let placements = profile.placements(&activity);

    assert!(profile.can_host(&activity));
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
    }
}

#[test]
fn una_clase_de_150_cruza_un_descanso() {
    let profile = ulsa();
    let activity = class_with(150, breaks(1, false, 0));
    let placements = profile.placements(&activity);

    assert_eq!(placements.len(), 4);

    for placement in &placements {
        assert_eq!(placement.interruptions().len(), 1);
        assert_eq!(placement.interruptions()[0].kind(), SeparatorKind::Break);
        assert_eq!(
            placement
                .pieces()
                .iter()
                .map(|piece| piece.as_minutes())
                .sum::<u32>(),
            150
        );

        let crossed: DTimeSpan = placement
            .interruptions()
            .iter()
            .map(|interruption| interruption.duration())
            .sum();

        assert_eq!(placement.end() - placement.start(), minutes(150) + crossed);
    }
}

#[test]
fn una_clase_de_150_no_cruza_el_almuerzo_por_defecto() {
    let profile = ulsa();
    let activity = class_with(150, breaks(1, false, 0));

    assert!(
        activity
            .interruption_policy()
            .to_string()
            .contains("eating_breaks=false")
    );

    for placement in profile.placements(&activity) {
        assert!(
            placement
                .interruptions()
                .iter()
                .all(|interruption| interruption.kind() != SeparatorKind::EatingBreak)
        );
    }
}

#[test]
fn solo_cruza_el_almuerzo_si_esta_habilitado() {
    let profile = ulsa();
    let activity = class_with(150, breaks(1, true, 0));
    let adjusting = class_with(
        150,
        InterruptionPolicy::Breaks {
            max: 1,
            breaks: false,
            eating_breaks: true,
            min_chunk: minutes(0),
        },
    );

    assert_eq!(profile.placements(&activity).len(), 6);
    assert_eq!(profile.placements(&adjusting).len(), 2);

    for placement in profile.placements(&adjusting) {
        assert_eq!(
            placement.interruptions()[0].kind(),
            SeparatorKind::EatingBreak
        );
    }
}

#[test]
fn min_chunk_obliga_a_repartir_los_trozos() {
    let profile = ulsa();
    let activity = class_with(150, breaks(1, false, 60));
    let placements = profile.placements(&activity);

    assert_eq!(placements.len(), 2);
    assert!(
        placements
            .iter()
            .all(|placement| placement.start() == at(8, 0) || placement.start() == at(13, 0))
    );

    for placement in &placements {
        assert_eq!(placement.pieces(), &[minutes(90), minutes(60)]);
        assert!(placement.pieces().iter().all(|piece| *piece >= minutes(60)));
    }
}

#[test]
fn min_chunk_tambien_acota_la_actividad_entera() {
    let profile = ulsa();
    let activity = class_with(50, breaks(2, true, 60));

    assert!(profile.placements(&activity).is_empty());
}

#[test]
fn un_hueco_nunca_es_puente() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        academic(at(10, 0), at(10, 50)),
    ]);
    let puente = class(100).interruptible(true);

    assert!(!profile.can_host(&puente));
    assert!(profile.placements(&puente).is_empty());
    assert_eq!(profile.placements(&class(50)).len(), 2);
}

#[test]
fn max_cuenta_spans_y_no_separadores() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        segment(at(8, 50), at(9, 10), SegmentKind::Break),
        segment(at(9, 10), at(9, 30), SegmentKind::EatingBreak),
        academic(at(9, 30), at(9, 50)),
    ]);

    assert_eq!(
        profile
            .placements(&class_with(70, breaks(2, true, 0)))
            .len(),
        1
    );
    assert!(
        profile
            .placements(&class_with(70, breaks(1, true, 0)))
            .is_empty()
    );
}

#[test]
fn una_actividad_de_400_solo_cabe_si_es_interrumpible() {
    let profile = ulsa();

    assert!(profile.placements(&class(400)).is_empty());

    let flexible = class(400).interruptible(true);
    let placements = profile.placements(&flexible);

    assert_eq!(placements.len(), 1);
    assert_eq!(placements[0].start(), at(8, 0));
    assert_eq!(placements[0].interruptions().len(), 3);
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
            profile.can_host(activity),
            !profile.placements(activity).is_empty()
        );
    }
}

#[test]
fn muestra_un_placement() {
    let profile = ulsa();

    assert_eq!(
        profile.placements(&Activity::new(ActivityType::Conference, minutes(100)).unwrap())[0]
            .to_string(),
        "08:00 -> 09:40 (01:40:00)"
    );
    assert_eq!(
        profile.placements(&class_with(150, breaks(1, false, 0)))[0].to_string(),
        "08:00 -> 10:50 (01:40:00 + 00:50:00 | Break 00:20:00)"
    );
}
