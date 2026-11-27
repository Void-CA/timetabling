use crate::time::DTimeSpan;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptionPolicy {
    None,
    Breaks {
        max_interruptions: u32,
        breaks: bool,
        eating_breaks: bool,
        min_chunk: DTimeSpan,
    },
}
