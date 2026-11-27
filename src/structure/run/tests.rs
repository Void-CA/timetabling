use crate::time::{DTime, DTimeSpan};

use super::Run;

fn at(hour: u8, minutes: u8) -> DTime {
    DTime::new(hour, minutes, 0).unwrap()
}

#[test]
fn guarda_limites_y_duracion() {
    let run = Run::new(at(8, 0), at(9, 40));

    assert_eq!(run.start(), at(8, 0));
    assert_eq!(run.end(), at(9, 40));
    assert_eq!(run.duration(), DTimeSpan::from_minutes(100));
}

#[test]
fn contiene_instantes_dentro_del_rango() {
    let run = Run::new(at(8, 0), at(9, 40));

    assert!(run.contains(at(8, 0)));
    assert!(run.contains(at(9, 39)));
    assert!(!run.contains(at(9, 40)));
    assert!(!run.contains(at(10, 0)));
}

#[test]
fn muestra_rango_y_duracion() {
    let run = Run::new(at(8, 0), at(9, 40));

    assert_eq!(run.to_string(), "08:00 -> 09:40 (01:40:00)");
}
