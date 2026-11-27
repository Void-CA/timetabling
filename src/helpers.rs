use crate::prelude::*;

pub fn at(hour: u8, minutes: u8) -> DTime {
    DTime::new(hour, minutes, 0).expect("hora válida")
}

pub fn pause(kind: PauseKind, minutes: u32) -> Pause {
    Pause::new(kind, DTimeSpan::from_minutes(minutes)).expect("pausa válida")
}

pub fn ulsa() -> ScheduleTemplate {
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
