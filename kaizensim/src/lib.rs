use serde::{Serialize, Serializer};
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

#[derive(Serialize)]
pub struct Score {
    pub level: i32,
    pub time: i32,
    pub cost: i32,
    pub area: i32,
    pub manipulated: bool,
}

#[derive(Debug)]
pub enum KaizenError {
    CorruptedFile,
    SolutionIncomplete,
    UnknownInstruction(i32),
    UnknownPart(i32),
    UnknownVersion(i32),
}

impl Serialize for KaizenError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Error", 1)?;
        state.serialize_field("error", &format!("{self:?}"))?;
        state.end()
    }
}

pub trait ParseEnum<T> : Sized {
    fn try_from(value: T) -> Result<Self, KaizenError>;
}