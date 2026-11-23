use std::ops;

use crate::time::DTimeSpan;

use super::model::DTime;

impl ops::Add<DTimeSpan> for DTime {
    type Output = DTime;

    fn add(self, rhs: DTimeSpan) -> Self::Output {
        let total = self.as_seconds() as u64 + rhs.as_seconds() as u64;

        DTime::from_seconds((total % DTime::SECONDS_PER_DAY as u64) as u32)
    }
}

impl ops::Sub<DTimeSpan> for DTime {
    type Output = DTime;

    fn sub(self, rhs: DTimeSpan) -> Self::Output {
        let total = self.as_seconds() as u64 + DTime::SECONDS_PER_DAY as u64
            - (rhs.as_seconds() as u64 % DTime::SECONDS_PER_DAY as u64);

        DTime::from_seconds((total % DTime::SECONDS_PER_DAY as u64) as u32)
    }
}

impl ops::Sub for DTime {
    type Output = DTimeSpan;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs = self.as_seconds() as u64;
        let rhs = rhs.as_seconds() as u64;

        DTimeSpan::from_seconds(
            ((lhs + DTime::SECONDS_PER_DAY as u64 - rhs) % DTime::SECONDS_PER_DAY as u64) as u32,
        )
    }
}
