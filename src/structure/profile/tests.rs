use crate::structure::{
    DayStructure, Pause, PauseKind, ScheduleTemplate, SegmentKind, TimeSegment,
};
use crate::time::{DTime, DTimeSpan};

use super::{DayProfile, InterruptionSpanKind};

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
fn ulsa_episodios_de_una_pausa_cada_uno() {
    let profile = ulsa();

    assert_eq!(profile.interruptions().len(), profile.runs().len() - 1);
    assert_eq!(
        profile
            .interruptions()
            .iter()
            .map(|episode| episode.duration().as_minutes())
            .collect::<Vec<u32>>(),
        vec![20, 80, 20]
    );
    assert!(
        profile
            .interruptions()
            .iter()
            .all(|episode| episode.spans().len() == 1)
    );
    assert_eq!(
        profile
            .interruptions()
            .iter()
            .flat_map(|episode| episode.spans())
            .map(|span| span.kind())
            .collect::<Vec<InterruptionSpanKind>>(),
        vec![
            InterruptionSpanKind::Break,
            InterruptionSpanKind::EatingBreak,
            InterruptionSpanKind::Break
        ]
    );
}

#[test]
fn conserva_las_fronteras_de_periodo() {
    let profile = ulsa();
    let starts = profile.academic_starts();

    assert_eq!(starts.len(), 8);
    assert_eq!(starts[0], at(8, 0));
    assert_eq!(starts[7], at(15, 50));
    assert!(starts.windows(2).all(|pair| pair[0] < pair[1]));
    assert!(
        starts
            .iter()
            .all(|start| profile.run_index_at(*start).is_some())
    );
}

#[test]
fn fusiona_segmentos_academicos_contiguos() {
    let profile = profile(vec![
        academic(at(8, 0), at(9, 40)),
        academic(at(9, 40), at(11, 20)),
    ]);

    assert_eq!(profile.runs().len(), 1);
    assert_eq!(profile.runs()[0].duration(), minutes(200));
    assert_eq!(profile.academic_starts().len(), 2);
    assert!(profile.interruptions().is_empty());
}

#[test]
fn detecta_huecos_entre_runs() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        academic(at(10, 0), at(10, 50)),
    ]);

    assert_eq!(profile.runs().len(), 2);
    assert_eq!(profile.interruptions().len(), 1);
    assert_eq!(profile.interruptions()[0].spans().len(), 1);
    assert_eq!(
        profile.interruptions()[0].spans()[0].kind(),
        InterruptionSpanKind::Hole
    );
    assert_eq!(
        profile.interruptions()[0].spans()[0].duration(),
        minutes(70)
    );
    assert_eq!(profile.interruptions()[0].to_string(), "Hole 01:10:00");
}

#[test]
fn distingue_unassigned_de_un_hueco() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        segment(at(8, 50), at(9, 30), SegmentKind::Unassigned),
        academic(at(9, 30), at(10, 20)),
    ]);

    assert_eq!(profile.interruptions().len(), 1);
    assert_eq!(
        profile.interruptions()[0].spans()[0].kind(),
        InterruptionSpanKind::Unassigned
    );
    assert_eq!(profile.interruptions()[0].duration(), minutes(40));
}

#[test]
fn agrupa_pausas_adyacentes_en_un_solo_episodio() {
    let profile = profile(vec![
        academic(at(8, 0), at(8, 50)),
        segment(at(8, 50), at(9, 10), SegmentKind::Break),
        segment(at(9, 10), at(9, 30), SegmentKind::EatingBreak),
        academic(at(9, 30), at(10, 20)),
    ]);

    let episode = &profile.interruptions()[0];

    assert_eq!(episode.spans().len(), 2);
    assert_eq!(episode.duration(), minutes(40));
    assert_eq!(episode.to_string(), "Break 00:20:00 + EatingBreak 00:20:00");
}

#[test]
fn acepta_una_estructura_sin_runs() {
    let profile = profile(vec![segment(at(8, 0), at(8, 20), SegmentKind::Break)]);

    assert!(profile.runs().is_empty());
    assert!(profile.interruptions().is_empty());
    assert!(profile.academic_starts().is_empty());
    assert!(profile.longest_run().is_none());
    assert!(profile.mean_run().is_none());
    assert_eq!(profile.run_lengths().count(), 0);
    assert_eq!(profile.run_index_at(at(8, 10)), None);
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
    assert_eq!(profile.interruptions()[0].duration(), minutes(30));
    assert_eq!(
        profile.academic_starts(),
        &[at(22, 0), at(23, 30), at(1, 30)]
    );
}

#[test]
fn ubica_el_run_que_contiene_un_instante() {
    let profile = profile(vec![
        academic(at(8, 0), at(9, 40)),
        academic(at(10, 0), at(10, 50)),
    ]);

    assert_eq!(profile.run_index_at(at(8, 0)), Some(0));
    assert_eq!(profile.run_index_at(at(9, 39)), Some(0));
    assert_eq!(profile.run_index_at(at(9, 40)), None);
    assert_eq!(profile.run_index_at(at(10, 0)), Some(1));
    assert_eq!(profile.run_index_at(at(11, 0)), None);
}

#[test]
fn muestra_runs_y_episodios() {
    let profile = profile(vec![
        academic(at(8, 0), at(9, 40)),
        segment(at(9, 40), at(10, 0), SegmentKind::Break),
        academic(at(10, 0), at(10, 50)),
    ]);

    assert_eq!(profile.runs()[0].to_string(), "08:00 -> 09:40 (01:40:00)");
    assert_eq!(profile.interruptions()[0].to_string(), "Break 00:20:00");
    assert_eq!(InterruptionSpanKind::Unassigned.to_string(), "Unassigned");
}
