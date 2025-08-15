use std::collections::HashSet;
use std::io::Read;

use crate::*;
use crate::point::*;

const MAX_COORD: i32 = 4096;
const MAX_INSTRUCTIONS_PER_CYCLE: i32 = 4;
const MAX_EXTEND: i32 = 3;
const MAX_SLIDE: i32 = 9;
const MIN_TRACK_LEN: i32 = 1;
const MAX_TRACK_LEN: i32 = 9;
const MIN_ARM_LEN: i32 = 1;
const MAX_ARM_LEN: i32 = 4;
const MIN_ARM_LABEL: i32 = 0;
const MAX_ARM_LABEL: i32 = 25;

pub fn is_manipulated(solution: &Solution) -> bool {
    let mut arms = HashSet::new();
    let mut inputs = HashSet::new();
    for part in &solution.parts {
        if !(-MAX_COORD..=MAX_COORD).contains(&part.pos.x) { return true; }
        if !(-MAX_COORD..=MAX_COORD).contains(&part.pos.y) { return true; }
        if !part.size.is_line() { return true; }
        match part.kind {
            PartKind::Arm => {
                if !(MIN_ARM_LABEL..=MAX_ARM_LABEL).contains(&part.arm) { return true; }
                if !(MIN_ARM_LEN..=MAX_ARM_LEN).contains(&part.size.sum().abs()) { return true; }
                if !arms.insert(part.arm) { return true; }
                if part.input != -1 { return true; }
            },
            PartKind::Cutter => {
                if part.arm != -1 { return true; }
                if !part.size.is_empty() { return true; }
                if part.input != -1 { return true; }
            },
            PartKind::Drill => {
                if !(MIN_ARM_LABEL..=MAX_ARM_LABEL).contains(&part.arm) { return true; }
                if !(MIN_ARM_LEN..=MAX_ARM_LEN).contains(&part.size.sum().abs()) { return true; }
                if !arms.insert(part.arm) { return true; }
                if part.input != -1 { return true; }
            },
            PartKind::Input => {
                if part.arm != -1 { return true; }
                if !inputs.insert(part.input) { return true; }
            },
            PartKind::Riveter => {
                if part.arm != -1 { return true; }
                if part.size.sum().abs() != 1 { return true; }
                if part.input != -1 { return true; }
            },
            PartKind::Track => {
                if part.arm != -1 { return true; }
                if !(MIN_TRACK_LEN..=MAX_TRACK_LEN).contains(&part.size.sum().abs()) { return true; }
                if part.input != -1 { return true; }
            },
            PartKind::Welder => {
                if part.arm != -1 { return true; }
                if part.size.sum().abs() != 1 { return true; }
                if part.input != -1 { return true; }
            },
        }
    }
    let mut instructions = HashSet::new();
    for instruction in &solution.instructions {
        if instruction.column < 0 { return true; }
        if !(0..MAX_INSTRUCTIONS_PER_CYCLE).contains(&instruction.row) { return true; }
        if !instructions.insert(Point { x: instruction.column, y: instruction.row }) { return true; }
        if !arms.contains(&instruction.arm) { return true; }
        match instruction.kind {
            InstructionKind::Extend => {
                if !(1..=MAX_EXTEND).contains(&instruction.distance.abs()) { return true; }
            },
            InstructionKind::Flip => {
            },
            InstructionKind::Poke => {
            },
            InstructionKind::Slide => {
                if !(1..=MAX_SLIDE).contains(&instruction.distance.abs()) { return true; }
            },
        }
    }
    false
}

pub struct Solution {
    pub level: i32,
    pub name: String,
    pub solved: bool,
    pub time: i32,
    pub cost: i32,
    pub area: i32,
    pub parts: Vec<Part>,
    pub instructions: Vec<Instruction>,
}

impl Solution {
    pub fn try_from(data: &[u8]) -> Result<Self, KaizenError> {
        SolutionReader::from(data).read_solution()
    }
}

struct SolutionReader<'a> {
    data: &'a [u8],
    version: Version,
}

impl<'a> SolutionReader<'a> {
    pub fn from(data: &'a [u8]) -> Self {
        let version = Version::Unknown;
        SolutionReader { data, version }
    }
    pub fn read_solution(&mut self) -> Result<Solution, KaizenError> {
        self.version = self.read_enum::<Version>()?;
        let level = self.read_i32()?;
        let name = self.read_string()?;
        let solved = self.read_bool()?;
        let time = self.read_i32()?;
        let cost = self.read_i32()?;
        let area = self.read_i32()?;
        let parts = self.read_parts()?;
        let instructions = self.read_instructions()?;
        if self.data.is_empty() {
            Ok(Solution { level, name, solved, time, cost, area, parts, instructions })
        }
        else {
            Err(KaizenError::CorruptedFile)
        }
    }
    fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], KaizenError> {
        let mut buf = [0u8; N];
        match self.data.read_exact(&mut buf) {
            Ok(()) => Ok(buf),
            Err(_) => Err(KaizenError::CorruptedFile),
        }
    }
    fn read_bool(&mut self) -> Result<bool, KaizenError> {
        Ok(self.read_bytes::<1>()?[0] == 1)
    }
    fn read_i32(&mut self) -> Result<i32, KaizenError> {
        Ok(i32::from_le_bytes(self.read_bytes()?))
    }
    fn read_usize(&mut self) -> Result<usize, KaizenError> {
        usize::try_from(self.read_i32()?).or(Err(KaizenError::CorruptedFile))
    }
    fn read_point(&mut self) -> Result<Point, KaizenError> {
        let x = self.read_i32()?;
        let y = self.read_i32()?;
        Ok(Point { x, y })
    }
    fn read_enum<T: ParseEnum<i32>>(&mut self) -> Result<T, KaizenError> {
        T::try_from(self.read_i32()?)
    }
    fn read_string(&mut self) -> Result<String, KaizenError> {
        let len = self.read_usize()?;
        self.read_fixed_size_string(len)
    }
    fn read_fixed_size_string(&mut self, len: usize) -> Result<String, KaizenError> {
        let mut buf = vec![0u8; len];
        match self.data.read_exact(&mut buf) {
            Ok(()) => String::from_utf8(buf).or(Err(KaizenError::CorruptedFile)),
            Err(_) => Err(KaizenError::CorruptedFile),
        }
    }
    fn read_parts(&mut self) -> Result<Vec<Part>, KaizenError> {
        let len = self.read_usize()?;
        (0..len).map(|_| self.read_part()).collect()
    }
    fn read_part(&mut self) -> Result<Part, KaizenError> {
        let kind = self.read_enum::<PartKind>()?;
        let arm = self.read_i32()?;
        let pos = self.read_point()?;
        let size = self.read_point()?;
        let input = self.read_i32()?;
        Ok(Part { kind, arm, pos, size, input })
    }
    fn read_instructions(&mut self) -> Result<Vec<Instruction>, KaizenError> {
        let len = self.read_usize()?;
        (0..len).map(|_| self.read_instruction()).collect()
    }
    fn read_instruction(&mut self) -> Result<Instruction, KaizenError> {
        let column = self.read_i32()?;
        let row = self.read_i32()?;
        let kind = self.read_enum::<InstructionKind>()?;
        let arm = self.read_i32()?;
        let distance = self.read_i32()?;
        let grab = self.read_bool()?;
        if matches!(self.version, Version::V2) {
            self.read_i32()?;
        }
        Ok(Instruction { column, row, kind, arm, distance, grab })
    }
}

pub struct Part {
    pub kind: PartKind,
    pub arm: i32,
    pub pos: Point,
    pub size: Point,
    pub input: i32,
}

pub struct Instruction {
    pub column: i32,
    pub row: i32,
    pub kind: InstructionKind,
    pub arm: i32,
    pub distance: i32,
    pub grab: bool,
}

enum Version {
    Unknown,
    V1,
    V2,
}

impl ParseEnum<i32> for Version {
    fn try_from(value: i32) -> Result<Self, KaizenError> {
        match value {
            10 => Ok(Version::V1),
            11 => Ok(Version::V2),
            other => Err(KaizenError::UnknownVersion(other)),
        }
    }
}

pub enum PartKind {
    Arm,
    Cutter,
    Drill,
    Input,
    Riveter,
    Track,
    Welder,
}

impl ParseEnum<i32> for PartKind {
    fn try_from(value: i32) -> Result<Self, KaizenError> {
        match value {
            1 => Ok(PartKind::Arm),
            2 => Ok(PartKind::Track),
            3 => Ok(PartKind::Welder),
            4 => Ok(PartKind::Riveter),
            5 => Ok(PartKind::Cutter),
            6 => Ok(PartKind::Input),
            8 => Ok(PartKind::Drill),
            other => Err(KaizenError::UnknownPart(other)),
        }
    }
}

pub enum InstructionKind {
    Extend,
    Flip,
    Poke,
    Slide,
}

impl ParseEnum<i32> for InstructionKind {
    fn try_from(value: i32) -> Result<Self, KaizenError> {
        match value {
            1 => Ok(InstructionKind::Extend),
            2 => Ok(InstructionKind::Slide),
            3 => Ok(InstructionKind::Flip),
            4 => Ok(InstructionKind::Poke),
            other => Err(KaizenError::UnknownInstruction(other)),
        }
    }
}