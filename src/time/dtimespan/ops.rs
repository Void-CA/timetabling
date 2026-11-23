use std::iter::Sum;
use std::ops;

use crate::time::DTime;

use super::model::DTimeSpan;

impl ops::Add for DTimeSpan {
    type Output = DTimeSpan;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_seconds(
            self.as_seconds()
                .checked_add(rhs.as_seconds())
                .expect("DTimeSpan: la duración excede u32::MAX segundos"),
        )
    }
}

impl ops::Sub for DTimeSpan {
    type Output = DTimeSpan;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs).expect(
            "DTimeSpan: no se puede restar una duración mayor (el resultado sería negativo)",
        )
    }
}

impl ops::Mul<u32> for DTimeSpan {
    type Output = DTimeSpan;

    fn mul(self, rhs: u32) -> Self::Output {
        Self::from_seconds(
            self.as_seconds()
                .checked_mul(rhs)
                .expect("DTimeSpan: la duración excede u32::MAX segundos"),
        )
    }
}

impl ops::Div<u32> for DTimeSpan {
    type Output = DTimeSpan;

    fn div(self, rhs: u32) -> Self::Output {
        Self::from_seconds(self.as_seconds() / rhs)
    }
}

impl Sum for DTimeSpan {
    fn sum<I: Iterator<Item = DTimeSpan>>(iter: I) -> Self {
        iter.fold(Self::zero(), |acc, span| acc + span)
    }
}

impl ops::Add<DTime> for DTimeSpan {
    type Output = DTime;

    fn add(self, rhs: DTime) -> Self::Output {
        rhs + self
    }
}
