pub mod activity;
pub mod analysis;
pub mod structure;
pub mod time;

use activity::{Activity, ActivityType, InterruptionPolicy};
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

fn report(activity: &Activity, profile: &analysis::DayProfile) {
    println!();
    println!("Actividad: {activity}");
    println!("  Política:  {}", activity.interruption_policy());
    println!("  ¿Aloja?    {}", profile.can_host(activity));

    let placements = profile.placements(activity);

    println!("  Placements: {}", placements.len());

    for placement in &placements {
        println!("    {placement}");
    }
}

fn main() {
    let day = ulsa().build();
    let profile = day.profile();

    for segment in &day {
        println!("{segment}");
    }

    println!();
    println!("Jornada:   {} -> {}", day.starts_at(), day.ends_at());
    println!("Extensión: {}", day.total_span());
    println!("Capacidad académica: {}", day.academic_capacity());
    println!("Tramos académicos: {}", day.academic_segments().count());

    println!();
    println!("Runs contiguos:");

    for run in profile.runs() {
        println!("  {run}");
    }

    println!(
        "Continuidad máxima: {}",
        profile
            .longest_run()
            .map(|run| run.duration().to_string())
            .unwrap_or_else(|| "ninguna".to_string())
    );
    println!(
        "Continuidad media:  {}",
        profile
            .mean_run()
            .map(|span| span.to_string())
            .unwrap_or_else(|| "ninguna".to_string())
    );

    let clase =
        Activity::new(ActivityType::Class, DTimeSpan::from_minutes(150)).expect("actividad válida");
    let conferencia = Activity::new(ActivityType::Conference, DTimeSpan::from_minutes(100))
        .expect("actividad válida");
    let flexible = Activity::new(ActivityType::Class, DTimeSpan::from_minutes(150))
        .expect("actividad válida")
        .policy(InterruptionPolicy::Breaks {
            max: 1,
            breaks: true,
            eating_breaks: false,
            min_chunk: DTimeSpan::zero(),
        });
    let jornada_completa = Activity::new(ActivityType::Meeting, DTimeSpan::from_minutes(400))
        .expect("actividad válida")
        .interruptible(true);

    report(&clase, &profile);
    report(&conferencia, &profile);
    report(&flexible, &profile);
    report(&jornada_completa, &profile);
}
