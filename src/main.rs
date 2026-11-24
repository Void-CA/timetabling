pub mod activity;
pub mod structure;
pub mod time;

use activity::{Activity, ActivityType};
use structure::{DayStructure, SegmentKind, TimeSegment};
use time::{DTime, DTimeSpan};

fn at(hour: u8, minutes: u8) -> DTime {
    DTime::new(hour, minutes, 0).expect("hora válida")
}

fn segment(start: DTime, end: DTime, kind: SegmentKind) -> TimeSegment {
    TimeSegment::new(start, end, kind).expect("tramo válido")
}

fn jornada() -> DayStructure {
    DayStructure::new(vec![
        segment(at(8, 0), at(8, 50), SegmentKind::Academic),
        segment(at(8, 50), at(9, 40), SegmentKind::Academic),
        segment(at(9, 40), at(10, 0), SegmentKind::Break),
        segment(at(10, 0), at(10, 50), SegmentKind::Academic),
        segment(at(10, 50), at(11, 40), SegmentKind::Academic),
        segment(at(11, 40), at(13, 00), SegmentKind::EatingBreak),
        segment(at(13, 0), at(13, 50), SegmentKind::Academic),
        segment(at(13, 50), at(14, 40), SegmentKind::Academic),
        segment(at(14, 40), at(15, 0), SegmentKind::Break),
        segment(at(15, 0), at(15, 50), SegmentKind::Academic),
        segment(at(15, 50), at(16, 40), SegmentKind::Academic),
    ])
    .expect("jornada válida")
}

fn main() {
    let day = jornada();

    for segment in &day {
        println!("{segment}");
    }

    println!();
    println!("Jornada:   {} -> {}", day.starts_at(), day.ends_at());
    println!("Extensión: {}", day.total_span());
    println!("Capacidad académica: {}", day.academic_capacity());
    println!("Tramos académicos: {}", day.academic_segments().count());

    let clase =
        Activity::new(ActivityType::Class, DTimeSpan::from_minutes(150)).expect("actividad válida");
    let practica = Activity::new(ActivityType::Lab, DTimeSpan::from_minutes(100))
        .expect("actividad válida")
        .interruptible(true);
    let conferencia = Activity::new(ActivityType::Conference, DTimeSpan::from_minutes(400))
        .expect("actividad válida");

    println!();
    println!("Actividad: {clase}");
    println!("Actividad: {practica}");
    println!("Actividad: {conferencia}");

    let capacity = day.academic_capacity();

    println!();
    println!(
        "Capacidad {} alcanza para {}: {}",
        capacity,
        clase.duration(),
        capacity >= clase.duration()
    );
    println!(
        "Capacidad {} alcanza para {}: {}",
        capacity,
        conferencia.duration(),
        capacity >= conferencia.duration()
    );
}
