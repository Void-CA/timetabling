//! Tipos del dominio de uso frecuente.
//!
//! Pensado para importarse en bloque:
//!
//! ```text
//! use crate::prelude::*;
//! ```

pub use crate::activity::{Activity, ActivityError, ActivityType, InterruptionPolicy};
pub use crate::placement::{Limits, Placement, StartPolicy};
pub use crate::structure::{
    DayProfile, DayStructure, Interruption, InterruptionSpan, InterruptionSpanKind, Pause,
    PauseKind, Run, ScheduleTemplate, SegmentError, SegmentKind, StructureError, TemplateError,
    TimeSegment,
};
pub use crate::time::{DTime, DTimeSpan, TimeError};
