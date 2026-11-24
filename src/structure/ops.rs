use std::fmt;
use std::ops;

use super::model::{DayStructure, StructureError};
use super::timesegment::TimeSegment;

impl ops::Index<usize> for DayStructure {
    type Output = TimeSegment;

    fn index(&self, index: usize) -> &Self::Output {
        &self.segments()[index]
    }
}

impl<'a> IntoIterator for &'a DayStructure {
    type Item = &'a TimeSegment;
    type IntoIter = std::slice::Iter<'a, TimeSegment>;

    fn into_iter(self) -> Self::IntoIter {
        self.segments().iter()
    }
}

impl fmt::Display for StructureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StructureError::EmptyStructure => {
                f.write_str("la estructura temporal no puede estar vacía")
            }
            StructureError::Overlapping { previous, next } => {
                write!(f, "el tramo {next} se solapa con el tramo {previous}")
            }
            StructureError::ExceedsDay { duration } => {
                write!(f, "la estructura excede un día: {duration}")
            }
        }
    }
}

impl std::error::Error for StructureError {}
