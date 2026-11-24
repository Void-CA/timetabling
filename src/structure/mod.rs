mod model;
mod ops;
mod template;
mod timesegment;

pub mod profile;

#[cfg(test)]
mod tests;

pub use self::model::{DayStructure, StructureError};
pub use self::template::{Pause, PauseKind, ScheduleTemplate, TemplateError};
pub use self::timesegment::{SegmentError, SegmentKind, TimeSegment};
