use crate::time::DTimeSpan;

use super::{Interruption, InterruptionSpan, InterruptionSpanKind};

fn minutes(value: u32) -> DTimeSpan {
    DTimeSpan::from_minutes(value)
}

#[test]
fn suma_la_duracion_de_sus_tramos() {
    let interruption = Interruption::new(vec![
        InterruptionSpan::new(InterruptionSpanKind::Break, minutes(20)),
        InterruptionSpan::new(InterruptionSpanKind::EatingBreak, minutes(20)),
    ]);

    assert_eq!(interruption.spans().len(), 2);
    assert_eq!(interruption.duration(), minutes(40));
}

#[test]
fn describe_tipos_y_episodios() {
    let interruption = Interruption::new(vec![
        InterruptionSpan::new(InterruptionSpanKind::Break, minutes(20)),
        InterruptionSpan::new(InterruptionSpanKind::EatingBreak, minutes(20)),
    ]);

    assert_eq!(
        interruption.to_string(),
        "Break 00:20:00 + EatingBreak 00:20:00"
    );
    assert_eq!(InterruptionSpanKind::Unassigned.to_string(), "Unassigned");
    assert_eq!(
        Interruption::new(vec![InterruptionSpan::new(
            InterruptionSpanKind::Hole,
            minutes(70)
        )])
        .to_string(),
        "Hole 01:10:00"
    );
}
