use super::{Pause, PauseKind, ScheduleTemplate, TemplateError};
use crate::structure::{DayStructure, SegmentKind, TimeSegment};
use crate::time::{DTime, DTimeSpan};

fn at(hour: u8, minutes: u8) -> DTime {
    DTime::new(hour, minutes, 0).unwrap()
}

fn pause(kind: PauseKind, minutes: u32) -> Pause {
    Pause::new(kind, DTimeSpan::from_minutes(minutes)).unwrap()
}

fn segment(start: DTime, end: DTime, kind: SegmentKind) -> TimeSegment {
    TimeSegment::new(start, end, kind).unwrap()
}

fn ulsa() -> ScheduleTemplate {
    ScheduleTemplate::new(
        at(8, 0),
        at(16, 40),
        DTimeSpan::from_minutes(50),
        2,
        vec![
            pause(PauseKind::Break, 20),
            pause(PauseKind::EatingBreak, 80),
            pause(PauseKind::Break, 20),
        ],
    )
    .unwrap()
}

#[test]
fn reproduce_la_jornada_de_ulsa() {
    let expected = DayStructure::new(vec![
        segment(at(8, 0), at(8, 50), SegmentKind::Academic),
        segment(at(8, 50), at(9, 40), SegmentKind::Academic),
        segment(at(9, 40), at(10, 0), SegmentKind::Break),
        segment(at(10, 0), at(10, 50), SegmentKind::Academic),
        segment(at(10, 50), at(11, 40), SegmentKind::Academic),
        segment(at(11, 40), at(13, 0), SegmentKind::EatingBreak),
        segment(at(13, 0), at(13, 50), SegmentKind::Academic),
        segment(at(13, 50), at(14, 40), SegmentKind::Academic),
        segment(at(14, 40), at(15, 0), SegmentKind::Break),
        segment(at(15, 0), at(15, 50), SegmentKind::Academic),
        segment(at(15, 50), at(16, 40), SegmentKind::Academic),
    ])
    .unwrap();

    assert_eq!(ulsa().build(), expected);
}

#[test]
fn cubre_la_jornada_sin_tramos_vacios() {
    let day = ulsa().build();

    assert_eq!(day.segments().len(), 11);
    assert_eq!(day.academic_segments().count(), 8);
    assert_eq!(day.academic_capacity(), DTimeSpan::from_minutes(400));
    assert_eq!(day.total_span(), DTimeSpan::from_minutes(520));
    assert_eq!(day.starts_at(), at(8, 0));
    assert_eq!(day.ends_at(), at(16, 40));
    assert!(
        day.segments()
            .iter()
            .all(|segment| segment.kind() != SegmentKind::Other)
    );
}

#[test]
fn encadena_cada_grupo_con_su_pausa() {
    let day = ulsa().build();
    let kinds: Vec<SegmentKind> = day.segments().iter().map(TimeSegment::kind).collect();

    assert_eq!(
        kinds,
        vec![
            SegmentKind::Academic,
            SegmentKind::Academic,
            SegmentKind::Break,
            SegmentKind::Academic,
            SegmentKind::Academic,
            SegmentKind::EatingBreak,
            SegmentKind::Academic,
            SegmentKind::Academic,
            SegmentKind::Break,
            SegmentKind::Academic,
            SegmentKind::Academic,
        ]
    );
}

#[test]
fn deja_el_remanente_como_other() {
    let template = ScheduleTemplate::new(
        at(8, 0),
        at(17, 0),
        DTimeSpan::from_minutes(50),
        2,
        vec![
            pause(PauseKind::Break, 20),
            pause(PauseKind::EatingBreak, 80),
            pause(PauseKind::Break, 20),
        ],
    )
    .unwrap();

    let day = template.build();
    let last = day.segments().last().unwrap();

    assert_eq!(last.kind(), SegmentKind::Other);
    assert_eq!(last.start_time(), at(16, 40));
    assert_eq!(last.end_time(), at(17, 0));
}

#[test]
fn se_detiene_cuando_se_agotan_las_pausas() {
    let template = ScheduleTemplate::new(
        at(8, 0),
        at(10, 40),
        DTimeSpan::from_minutes(50),
        2,
        vec![pause(PauseKind::Break, 20)],
    )
    .unwrap();

    let day = template.build();
    let last = day.segments().last().unwrap();

    assert_eq!(day.segments().len(), 4);
    assert_eq!(last.kind(), SegmentKind::Other);
    assert_eq!(last.duration(), DTimeSpan::from_minutes(40));
}

#[test]
fn grupos_de_un_periodo_alternan_pausas() {
    let template = ScheduleTemplate::new(
        at(8, 0),
        at(10, 0),
        DTimeSpan::from_minutes(50),
        1,
        vec![pause(PauseKind::Break, 20)],
    )
    .unwrap();

    let day = template.build();
    let academic: Vec<DTimeSpan> = day.academic_segments().map(TimeSegment::duration).collect();

    assert_eq!(day.segments().len(), 3);
    assert_eq!(academic, vec![DTimeSpan::from_minutes(50); 2]);
}

#[test]
fn genera_lo_mismo_en_cada_construccion() {
    assert_eq!(ulsa().build(), ulsa().build());
}

#[test]
fn expone_sus_parametros() {
    let template = ulsa();

    assert_eq!(template.start(), at(8, 0));
    assert_eq!(template.end(), at(16, 40));
    assert_eq!(template.period(), DTimeSpan::from_minutes(50));
    assert_eq!(template.group_size(), 2);
    assert_eq!(template.pauses().len(), 3);
    assert_eq!(template.declared_span(), DTimeSpan::from_minutes(520));
}

#[test]
fn rechaza_periodo_cero() {
    assert_eq!(
        ScheduleTemplate::new(at(8, 0), at(16, 40), DTimeSpan::zero(), 2, vec![]),
        Err(TemplateError::ZeroPeriod)
    );
}

#[test]
fn rechaza_grupo_vacio() {
    assert_eq!(
        ScheduleTemplate::new(at(8, 0), at(16, 40), DTimeSpan::from_minutes(50), 0, vec![]),
        Err(TemplateError::ZeroGroupSize)
    );
}

#[test]
fn rechaza_periodo_mayor_que_la_jornada() {
    assert_eq!(
        ScheduleTemplate::new(at(8, 0), at(9, 0), DTimeSpan::from_minutes(90), 2, vec![]),
        Err(TemplateError::PeriodExceedsDay {
            period: DTimeSpan::from_minutes(90),
            span: DTimeSpan::from_minutes(60),
        })
    );
}

#[test]
fn rechaza_pausa_cero() {
    assert_eq!(
        Pause::new(PauseKind::Break, DTimeSpan::zero()),
        Err(TemplateError::ZeroPause)
    );
}

#[test]
fn muestra_pausas_y_errores() {
    assert_eq!(pause(PauseKind::Break, 20).to_string(), "Break 00:20:00");
    assert_eq!(PauseKind::EatingBreak.to_string(), "EatingBreak");
    assert_eq!(
        TemplateError::ZeroGroupSize.to_string(),
        "un grupo debe tener al menos un período académico"
    );
    assert_eq!(
        TemplateError::PeriodExceedsDay {
            period: DTimeSpan::from_minutes(90),
            span: DTimeSpan::from_minutes(60),
        }
        .to_string(),
        "el período 01:30:00 no cabe en una jornada de 01:00:00"
    );
}
