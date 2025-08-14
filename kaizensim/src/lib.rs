mod point;
mod solution;

pub fn score(bytes: &[u8]) -> Result<Score, KaizenError> {
    let solution = solution::Solution::try_from(bytes)?;
    if solution.solved {
        Ok(Score {
            level: solution.level,
            time: solution.time,
            cost: solution.cost,
            area: solution.area,
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
}
impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\"level\": {}, \"time\": {}, \"cost\": {}, \"area\": {}}}", self.level, self.time, self.cost, self.area)
    }
}

#[derive(Debug)]
pub enum KaizenError {
    CorruptedFile,
    NumberOutsideAllowedRange(i32),
    SolutionIncomplete,
    UnknownInstruction(i32),
    UnknownPart(i32),
    UnknownVersion(i32),
}

pub trait ParseEnum<T> : Sized {
    fn try_from(value: T) -> Result<Self, KaizenError>;
}