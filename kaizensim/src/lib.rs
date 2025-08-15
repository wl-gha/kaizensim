use crate::solution::*;

mod point;
mod solution;

pub fn score(bytes: &[u8]) -> Result<Score, KaizenError> {
    let solution = Solution::try_from(bytes)?;
    let manipulated = is_manipulated(&solution);
    if solution.solved {
        Ok(Score {
            level: solution.level,
            time: solution.time,
            cost: solution.cost,
            area: solution.area,
            manipulated,
        })
    } else {
        Err(KaizenError::SolutionIncomplete)
    }
}

pub struct Score {
    pub level: i32,
    pub time: i32,
    pub cost: i32,
    pub area: i32,
    pub manipulated: bool,
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"level\": {}, \"time\": {}, \"cost\": {}, \"area\": {}, \"manipulated\": {}}}", self.level, self.time, self.cost, self.area, self.manipulated)
    }
}

#[derive(Debug)]
pub enum KaizenError {
    CorruptedFile,
    SolutionIncomplete,
    UnknownInstruction(i32),
    UnknownPart(i32),
    UnknownVersion(i32),
}

impl std::fmt::Display for KaizenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"error\": \"{self:?}\"}}")
    }
}

pub trait ParseEnum<T> : Sized {
    fn try_from(value: T) -> Result<Self, KaizenError>;
}