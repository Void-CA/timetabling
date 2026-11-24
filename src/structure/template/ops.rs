use std::fmt;

use super::model::{Pause, PauseKind, TemplateError};
use crate::structure::SegmentKind;

impl fmt::Display for PauseKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            PauseKind::Break => "Break",
            PauseKind::EatingBreak => "EatingBreak",
        };

        f.write_str(name)
    }
}

impl fmt::Display for Pause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.kind(), self.duration())
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateError::ZeroPeriod => {
                f.write_str("el período académico no puede tener duración cero")
            }
            TemplateError::ZeroGroupSize => {
                f.write_str("un grupo debe tener al menos un período académico")
            }
            TemplateError::ZeroPause => f.write_str("una pausa no puede tener duración cero"),
            TemplateError::PeriodExceedsDay { period, span } => {
                write!(f, "el período {period} no cabe en una jornada de {span}")
            }
        }
    }
}

impl std::error::Error for TemplateError {}

impl From<PauseKind> for SegmentKind {
    fn from(kind: PauseKind) -> Self {
        match kind {
            PauseKind::Break => SegmentKind::Break,
            PauseKind::EatingBreak => SegmentKind::EatingBreak,
        }
    }
}
