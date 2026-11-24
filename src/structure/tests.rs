use crate::time::{DTime, DTimeSpan};

use super::timesegment::{SegmentKind, TimeSegment};
use super::{DayStructure, StructureError};

fn at(hour: u8, minutes: u8) -> DTime {
    DTime::new(hour, minutes, 0).unwrap()
}

fn segment(start: DTime, end: DTime, kind: SegmentKind) -> TimeSegment {
    TimeSegment::new(start, end, kind).unwrap()
}

fn academic(start: DTime, end: DTime) -> TimeSegment {
    segment(start, end, SegmentKind::Academic)
}

fn jornada() -> Vec<TimeSegment> {
    vec![
        academic(at(8, 0), at(8, 50)),
        segment(at(8, 50), at(9, 10), SegmentKind::Break),
        academic(at(9, 10), at(10, 0)),
        segment(at(10, 0), at(10, 20), SegmentKind::Break),
        academic(at(10, 20), at(11, 10)),
        segment(at(11, 10), at(11, 30), SegmentKind::Break),
        academic(at(11, 30), at(12, 20)),
        segment(at(12, 20), at(13, 40), SegmentKind::EatingBreak),
        academic(at(13, 40), at(14, 30)),
        segment(at(14, 30), at(14, 50), SegmentKind::Break),
        academic(at(14, 50), at(15, 40)),
        segment(at(15, 40), at(16, 0), SegmentKind::Break),
        academic(at(16, 0), at(16, 50)),
        segment(at(16, 50), at(17, 0), SegmentKind::Other),
    ]
}

#[test]
fn rechaza_estructura_vacia() {
    assert_eq!(
        DayStructure::new(Vec::new()),
        Err(StructureError::EmptyStructure)
    );
}

#[test]
fn rechaza_solapamiento() {
    let segments = vec![
        academic(at(8, 0), at(8, 50)),
        academic(at(8, 30), at(9, 20)),
    ];

    assert_eq!(
        DayStructure::new(segments),
        Err(StructureError::Overlapping {
            previous: 0,
            next: 1,
        })
    );
}

#[test]
fn rechaza_estructura_mayor_a_un_dia() {
    let segments = vec![
        academic(at(8, 0), at(8, 50)),
        academic(at(8, 50), at(9, 40)),
        academic(at(8, 0), at(9, 40)),
    ];

    assert!(matches!(
        DayStructure::new(segments),
        Err(StructureError::ExceedsDay { .. })
    ));
}

#[test]
fn acepta_tramos_contiguos() {
    let day = DayStructure::new(vec![
        academic(at(8, 0), at(8, 50)),
        segment(at(8, 50), at(9, 0), SegmentKind::Break),
        academic(at(9, 0), at(9, 50)),
    ])
    .unwrap();

    assert_eq!(day.segments().len(), 3);
}

#[test]
fn acepta_huecos_sin_capacidad() {
    let day = DayStructure::new(vec![
        academic(at(8, 0), at(8, 50)),
        academic(at(10, 0), at(10, 50)),
    ])
    .unwrap();

    assert_eq!(day.total_span(), DTimeSpan::from_minutes(170));
}

#[test]
fn suma_solo_la_capacidad_academica() {
    let day = DayStructure::new(jornada()).unwrap();

    assert_eq!(day.academic_capacity(), DTimeSpan::from_minutes(350));
    assert_eq!(day.academic_segments().count(), 7);
}

#[test]
fn calcula_limites_y_extension_de_la_jornada() {
    let day = DayStructure::new(jornada()).unwrap();

    assert_eq!(day.starts_at(), at(8, 0));
    assert_eq!(day.ends_at(), at(17, 0));
    assert_eq!(day.total_span(), DTimeSpan::from_hours(9));
}

#[test]
fn acepta_jornada_que_cruza_medianoche() {
    let day = DayStructure::new(vec![
        academic(at(22, 0), at(2, 0)),
        segment(at(2, 0), at(2, 30), SegmentKind::Break),
        academic(at(2, 30), at(5, 30)),
    ])
    .unwrap();

    assert_eq!(day.starts_at(), at(22, 0));
    assert_eq!(day.ends_at(), at(5, 30));
    assert_eq!(day.total_span(), DTimeSpan::from_minutes(450));
    assert_eq!(day.academic_capacity(), DTimeSpan::from_minutes(420));
}

#[test]
fn interpreta_salto_largo_como_siguiente_dia() {
    let day = DayStructure::new(vec![
        academic(at(10, 0), at(10, 50)),
        academic(at(8, 0), at(8, 50)),
    ])
    .unwrap();

    assert_eq!(day.total_span(), DTimeSpan::from_minutes(22 * 60 + 50));
}

#[test]
fn itera_y_accede_por_indice() {
    let day = DayStructure::new(jornada()).unwrap();

    assert_eq!(day[0].start_time(), at(8, 0));

    let starts: Vec<DTime> = (&day).into_iter().map(TimeSegment::start_time).collect();

    assert_eq!(starts.len(), day.segments().len());
    assert_eq!(starts[1], at(8, 50));
}

#[test]
fn describe_errores() {
    assert_eq!(
        StructureError::EmptyStructure.to_string(),
        "la estructura temporal no puede estar vacía"
    );
    assert_eq!(
        StructureError::Overlapping {
            previous: 0,
            next: 1,
        }
        .to_string(),
        "el tramo 1 se solapa con el tramo 0"
    );
}

#[test]
fn compara_capacidad_con_actividad() {
    use crate::activity::{Activity, ActivityType};

    let day = DayStructure::new(jornada()).unwrap();
    let clase = Activity::new(ActivityType::Class, DTimeSpan::from_minutes(150)).unwrap();
    let conferencia =
        Activity::new(ActivityType::Conference, DTimeSpan::from_minutes(400)).unwrap();

    assert!(day.academic_capacity() >= clase.duration());
    assert!(day.academic_capacity() < conferencia.duration());
}
