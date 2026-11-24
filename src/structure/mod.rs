mod model;
mod ops;
mod timesegment;

#[cfg(test)]
mod tests;

pub use self::model::{DayStructure, StructureError};
pub use self::timesegment::{SegmentError, SegmentKind, TimeSegment};
