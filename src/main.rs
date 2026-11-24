pub mod activity;
pub mod structure;
pub mod time;

use activity::{Activity, ActivityType};
use structure::{Pause, PauseKind, ScheduleTemplate};
use time::{DTime, DTimeSpan};

fn at(hour: u8, minutes: u8) -> DTime {
    DTime::new(hour, minutes, 0).expect("hora válida")
}

fn pause(kind: PauseKind, minutes: u32) -> Pause {
    Pause::new(kind, DTimeSpan::from_minutes(minutes)).expect("pausa válida")
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
    .expect("plantilla válida")
}

fn main() {
    let day = ulsa().build();

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
    let conferencia = Activity::new(ActivityType::Conference, DTimeSpan::from_minutes(100))
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
