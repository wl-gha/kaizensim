use std::collections::HashMap;

mod point;
mod solution;

pub fn score(bytes: &Vec<u8>) -> Result<HashMap<&str, i32>, KaizenError> {
    let solution = solution::Solution::try_from(bytes)?;
    if solution.solved {
        Ok(HashMap::from([
            ("level", solution.level),
            ("time", solution.time),
            ("cost", solution.cost),
            ("area", solution.area),
        ]))
    } else {
        Err(KaizenError::SolutionIncomplete)
    }
}

#[derive(Debug)]
pub enum KaizenError {
    CorruptedFile,
    CouldNotReadFile,
    CouldNotReadStdIn,
    NumberOutsideAllowedRange(i32),
    SolutionIncomplete,
    UnknownInput(i32),
    UnknownInstruction(i32),
    UnknownCommand(String),
    UnknownLevel(i32),
    UnknownPart(i32),
    UnknownVersion(i32),
}

pub trait ParseEnum<T> : Sized {
    fn try_from(value: T) -> Result<Self, KaizenError>;
}