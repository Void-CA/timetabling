use super::DTimeSpan;
use crate::time::DTime;

#[test]
fn construye_desde_cada_unidad() {
    assert_eq!(DTimeSpan::from_seconds(90).as_seconds(), 90);
    assert_eq!(DTimeSpan::from_minutes(100).as_seconds(), 6_000);
    assert_eq!(DTimeSpan::from_hours(48).as_seconds(), 172_800);
    assert_eq!(DTimeSpan::from_days(2).as_seconds(), 172_800);
}

#[test]
fn descompone_en_horas_minutos_segundos() {
    let span = DTimeSpan::from_seconds(5_025);

    assert_eq!(span.hours(), 1);
    assert_eq!(span.minutes(), 23);
    assert_eq!(span.seconds(), 45);
    assert_eq!(span.as_minutes(), 83);
    assert_eq!(span.as_hours(), 1);
}

#[test]
fn una_duracion_puede_superar_un_dia() {
    let span = DTimeSpan::from_minutes(150 * 60);

    assert_eq!(span.as_hours(), 150);
    assert_eq!(span.to_string(), "150:00:00");
}

#[test]
fn muestra_formato_hh_mm_ss() {
    assert_eq!(DTimeSpan::from_minutes(100).to_string(), "01:40:00");
    assert_eq!(DTimeSpan::zero().to_string(), "00:00:00");
    assert!(DTimeSpan::zero().is_zero());
}

#[test]
fn suma_y_resta_de_duraciones() {
    let uno = DTimeSpan::from_minutes(100);
    let dos = DTimeSpan::from_minutes(40);

    assert_eq!((uno + dos).as_minutes(), 140);
    assert_eq!((uno - dos).as_minutes(), 60);
}

#[test]
fn resta_de_duraciones_no_negativa() {
    let corta = DTimeSpan::from_minutes(10);
    let larga = DTimeSpan::from_minutes(20);

    assert_eq!(larga.checked_sub(corta), Some(DTimeSpan::from_minutes(10)));
    assert_eq!(corta.checked_sub(larga), None);
    assert_eq!(corta.saturating_sub(larga), DTimeSpan::zero());
}

#[test]
#[should_panic(expected = "no se puede restar una duración mayor")]
fn resta_de_duraciones_negativa_entra_en_panico() {
    let _ = DTimeSpan::from_minutes(10) - DTimeSpan::from_minutes(20);
}

#[test]
fn multiplica_y_divide() {
    let bloque = DTimeSpan::from_minutes(50);

    assert_eq!((bloque * 3).as_minutes(), 150);
    assert_eq!((bloque / 2).as_minutes(), 25);
}

#[test]
fn acumula_con_sum() {
    let total: DTimeSpan = [DTimeSpan::from_minutes(50); 3].into_iter().sum();

    assert_eq!(total.as_minutes(), 150);
}

#[test]
fn ordena_por_duracion() {
    assert!(DTimeSpan::from_minutes(10) < DTimeSpan::from_minutes(20));
    assert_eq!(DTimeSpan::zero(), DTimeSpan::default());
}

#[test]
fn suma_duracion_mas_hora_es_conmutativa() {
    let hora = DTime::new(8, 50, 0).unwrap();
    let duracion = DTimeSpan::from_minutes(100);

    assert_eq!(hora + duracion, duracion + hora);
}
