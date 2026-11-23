pub mod time;

use time::{DTime, DTimeSpan};

struct Schedule {
    block_duration: DTimeSpan,
    start_time: DTime,
    end_time: DTime,
}

impl Schedule {
    pub fn new(block_duration: DTimeSpan, start_time: DTime, end_time: DTime) -> Schedule {
        Schedule {
            block_duration,
            start_time,
            end_time,
        }
    }

    pub fn duration(&self) -> DTimeSpan {
        self.end_time - self.start_time
    }
}

fn main() {
    let block = DTimeSpan::from_minutes(100);
    let start = DTime::new(8, 50, 0).unwrap();
    let end = start + block;

    println!("Inicio:   {start}");
    println!("Duración: {block}");
    println!("Fin:      {end}");
    println!("Diferencia: {}", end - start);

    let late = DTime::new(23, 30, 0).unwrap();
    println!("23:30 + 90 min = {}", late + DTimeSpan::from_minutes(90));

    let schedule = Schedule::new(
        DTimeSpan::from_minutes(50),
        DTime::new(8, 0, 0).unwrap(),
        DTime::new(17, 0, 0).unwrap(),
    );

    println!(
        "Jornada {} -> {}: {} ({} bloques de {})",
        schedule.start_time,
        schedule.end_time,
        schedule.duration(),
        schedule.duration().as_minutes() / schedule.block_duration.as_minutes(),
        schedule.block_duration,
    );
}
