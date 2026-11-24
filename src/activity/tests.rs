use super::{Activity, ActivityError, ActivityType, InterruptionPolicy};
use crate::time::DTimeSpan;

#[test]
fn rechaza_duracion_cero() {
    assert_eq!(
        Activity::new(ActivityType::Class, DTimeSpan::zero()),
        Err(ActivityError::EmptyDuration)
    );
}

#[test]
fn guarda_tipo_y_duracion() {
    let clase = Activity::new(ActivityType::Class, DTimeSpan::from_minutes(150)).unwrap();

    assert_eq!(clase.kind(), ActivityType::Class);
    assert_eq!(clase.duration(), DTimeSpan::from_minutes(150));
    assert!(!clase.is_interruptible());
}

#[test]
fn permite_marcar_interrumpible() {
    let practica = Activity::new(ActivityType::Lab, DTimeSpan::from_minutes(100))
        .unwrap()
        .interruptible(true);

    assert!(practica.is_interruptible());

    let fija = practica.clone().interruptible(false);

    assert!(!fija.is_interruptible());
}

#[test]
fn no_depende_de_la_unidad_temporal() {
    let clase = Activity::new(ActivityType::Class, DTimeSpan::from_minutes(150)).unwrap();
    let bloque = DTimeSpan::from_minutes(50);

    assert!(clase.duration() > bloque);
    assert_eq!(clase.duration().as_minutes(), 150);
}

#[test]
fn convierte_desde_tupla() {
    let actividad =
        Activity::try_from((ActivityType::Meeting, DTimeSpan::from_minutes(60))).unwrap();

    assert_eq!(actividad.kind(), ActivityType::Meeting);
    assert!(Activity::try_from((ActivityType::Meeting, DTimeSpan::zero())).is_err());
}

#[test]
fn muestra_tipo_y_duracion() {
    let conferencia = Activity::new(ActivityType::Conference, DTimeSpan::from_minutes(90)).unwrap();

    assert_eq!(conferencia.to_string(), "Conference 01:30:00");
    assert_eq!(
        conferencia.clone().interruptible(true).to_string(),
        "Conference 01:30:00 (interruptible)"
    );
    assert_eq!(ActivityType::Lab.to_string(), "Lab");
    assert_eq!(
        ActivityError::EmptyDuration.to_string(),
        "la duración de una actividad no puede ser cero"
    );
}

#[test]
fn ordena_tipos_de_actividad() {
    let mut tipos = vec![
        ActivityType::Lab,
        ActivityType::Class,
        ActivityType::Meeting,
    ];

    tipos.sort();

    assert_eq!(
        tipos,
        vec![
            ActivityType::Class,
            ActivityType::Meeting,
            ActivityType::Lab
        ]
    );
}

#[test]
fn por_defecto_es_contigua() {
    let clase = Activity::new(ActivityType::Class, DTimeSpan::from_minutes(150)).unwrap();

    assert_eq!(clase.interruption_policy(), InterruptionPolicy::None);
    assert!(!clase.is_interruptible());
}

#[test]
fn interruptible_construye_una_politica_permisiva() {
    let practica = Activity::new(ActivityType::Lab, DTimeSpan::from_minutes(100))
        .unwrap()
        .interruptible(true);

    assert_eq!(
        practica.interruption_policy(),
        InterruptionPolicy::Breaks {
            max_interruptions: u32::MAX,
            breaks: true,
            eating_breaks: true,
            min_chunk: DTimeSpan::zero(),
        }
    );
    assert!(practica.is_interruptible());
}

#[test]
fn una_politica_explicita_sustituye_al_booleano() {
    let politica = InterruptionPolicy::Breaks {
        max_interruptions: 1,
        breaks: true,
        eating_breaks: false,
        min_chunk: DTimeSpan::from_minutes(50),
    };

    let clase = Activity::new(ActivityType::Class, DTimeSpan::from_minutes(150))
        .unwrap()
        .interruptible(true)
        .policy(politica);

    assert_eq!(clase.interruption_policy(), politica);
    assert!(clase.is_interruptible());
}

#[test]
fn describe_politicas() {
    assert_eq!(InterruptionPolicy::None.to_string(), "None");
    assert_eq!(
        InterruptionPolicy::Breaks {
            max_interruptions: 1,
            breaks: true,
            eating_breaks: false,
            min_chunk: DTimeSpan::from_minutes(50),
        }
        .to_string(),
        "Breaks max_interruptions=1 breaks=true eating_breaks=false min_chunk=00:50:00"
    );
}
