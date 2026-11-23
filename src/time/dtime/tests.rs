use super::{DTime, TimeError};
use crate::time::DTimeSpan;

#[test]
fn valida_componentes() {
    assert!(matches!(
        DTime::new(24, 0, 0),
        Err(TimeError::InvalidHour(24))
    ));
    assert!(matches!(
        DTime::new(0, 60, 0),
        Err(TimeError::InvalidMinutes(60))
    ));
    assert!(matches!(
        DTime::new(0, 0, 60),
        Err(TimeError::InvalidSeconds(60))
    ));
    assert!(DTime::new(23, 59, 59).is_ok());
}

#[test]
fn convierte_desde_y_hacia_segundos() {
    let time = DTime::new(8, 50, 0).unwrap();

    assert_eq!(time.as_seconds(), 31_800);
    assert_eq!(DTime::from_seconds(31_800), time);
    assert_eq!(DTime::from_seconds(86_400 + 31_800), time);
}

#[test]
fn muestra_formato_de_24_horas() {
    assert_eq!(DTime::new(8, 50, 5).unwrap().to_string(), "08:50:05");
    assert_eq!(DTime::new(0, 0, 0).unwrap().to_string(), "00:00:00");
}

#[test]
fn ordena_cronologicamente() {
    let madrugada = DTime::new(1, 0, 0).unwrap();
    let manana = DTime::new(8, 50, 0).unwrap();
    let noche = DTime::new(23, 30, 0).unwrap();

    assert!(madrugada < manana);
    assert!(manana < noche);
}

#[test]
fn suma_duracion_dentro_del_dia() {
    let start = DTime::new(8, 50, 0).unwrap();
    let duration = DTimeSpan::from_minutes(100);

    assert_eq!(start + duration, DTime::new(10, 30, 0).unwrap());
}

#[test]
fn suma_duracion_cruzando_medianoche() {
    let start = DTime::new(23, 30, 0).unwrap();

    assert_eq!(
        start + DTimeSpan::from_minutes(90),
        DTime::new(1, 0, 0).unwrap()
    );
}

#[test]
fn suma_duracion_mayor_a_un_dia() {
    let start = DTime::new(10, 0, 0).unwrap();

    assert_eq!(start + DTimeSpan::from_hours(48), start);
    assert_eq!(
        start + DTimeSpan::from_hours(49),
        DTime::new(11, 0, 0).unwrap()
    );
    assert_eq!(start + DTimeSpan::from_seconds(u32::MAX), {
        let total = start.as_seconds() as u64 + u32::MAX as u64;
        DTime::from_seconds((total % 86_400) as u32)
    });
}

#[test]
fn resta_duracion_retrocediendo_en_el_reloj() {
    let end = DTime::new(0, 30, 0).unwrap();

    assert_eq!(
        end - DTimeSpan::from_minutes(90),
        DTime::new(23, 0, 0).unwrap()
    );
}

#[test]
fn diferencia_entre_horas() {
    let start = DTime::new(8, 50, 0).unwrap();
    let end = DTime::new(10, 30, 0).unwrap();

    assert_eq!(end - start, DTimeSpan::from_minutes(100));
}

#[test]
fn diferencia_entre_horas_cruzando_medianoche() {
    let start = DTime::new(23, 30, 0).unwrap();
    let end = DTime::new(1, 0, 0).unwrap();

    assert_eq!(end - start, DTimeSpan::from_minutes(90));
    assert_eq!(start - end, DTimeSpan::from_minutes(22 * 60 + 30));
}

#[test]
fn resta_es_inversa_de_la_suma() {
    let start = DTime::new(14, 15, 0).unwrap();
    let duration = DTimeSpan::from_minutes(250);

    assert_eq!(start + duration - duration, start);
}
