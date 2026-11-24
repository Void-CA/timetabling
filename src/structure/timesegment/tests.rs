use super::{SegmentError, SegmentKind, TimeSegment};
use crate::time::{DTime, DTimeSpan};

fn at(hour: u8, minutes: u8) -> DTime {
    DTime::new(hour, minutes, 0).unwrap()
}

#[test]
fn rechaza_tramo_de_duracion_cero() {
    assert_eq!(
        TimeSegment::new(at(8, 0), at(8, 0), SegmentKind::Academic),
        Err(SegmentError::EmptySegment)
    );
}

#[test]
fn guarda_limites_y_tipo() {
    let tramo = TimeSegment::new(at(8, 0), at(8, 50), SegmentKind::Academic).unwrap();

    assert_eq!(tramo.start_time(), at(8, 0));
    assert_eq!(tramo.end_time(), at(8, 50));
    assert_eq!(tramo.duration(), DTimeSpan::from_minutes(50));
    assert_eq!(tramo.kind(), SegmentKind::Academic);
    assert!(tramo.is_academic());
}

#[test]
fn calcula_duracion_cruzando_medianoche() {
    let tramo = TimeSegment::new(at(23, 30), at(1, 0), SegmentKind::Academic).unwrap();

    assert_eq!(tramo.duration(), DTimeSpan::from_minutes(90));
}

#[test]
fn ordena_por_hora_de_inicio() {
    let mut tramos = [
        TimeSegment::new(at(10, 0), at(10, 50), SegmentKind::Academic).unwrap(),
        TimeSegment::new(at(8, 0), at(8, 10), SegmentKind::Break).unwrap(),
        TimeSegment::new(at(9, 0), at(9, 50), SegmentKind::Academic).unwrap(),
    ];

    tramos.sort();

    assert_eq!(tramos[0].start_time(), at(8, 0));
    assert_eq!(tramos[2].start_time(), at(10, 0));
}

#[test]
fn muestra_tramo_y_errores() {
    let tramo = TimeSegment::new(at(8, 0), at(8, 50), SegmentKind::Academic).unwrap();

    assert_eq!(tramo.to_string(), "08:00 -> 08:50 (Academic)");
    assert_eq!(SegmentKind::Unassigned.to_string(), "Unassigned");
    assert_eq!(SegmentKind::EatingBreak.to_string(), "EatingBreak");
    assert_eq!(
        SegmentError::EmptySegment.to_string(),
        "un tramo no puede tener duración cero"
    );
}
