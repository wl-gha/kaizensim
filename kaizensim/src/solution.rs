use crate::*;
use crate::point::*;

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
    pub fn try_from(bytes: &Vec<u8>) -> Result<Self, KaizenError> {
        let mut data = &bytes[..];
        let _version = data.read_enum::<Version>()?;
        let level = data.read_i32()?;
        let name = data.read_string()?;
        let solved = data.read_bool()?;
        let time = data.read_i32()?;
        let cost = data.read_i32()?;
        let area = data.read_i32()?;
        let parts = data.read_parts()?;
        let instructions = data.read_instructions()?;
        if data.len() == 0 {
            Ok(Solution { level, name, solved, time, cost, area, parts, instructions })
        }
        else {
            Err(KaizenError::CorruptedFile)
        }
    }
}

trait SolutionReader: std::io::Read {
    fn read_bytes<const N: usize>(&mut self) -> Result<[u8; N], KaizenError> {
        let mut buf = [0u8; N];
        match self.read_exact(&mut buf) {
            Ok(()) => Ok(buf),
            Err(_) => Err(KaizenError::CorruptedFile),
        }
    }
    fn read_bool(&mut self) -> Result<bool, KaizenError> {
        Ok(self.read_bytes::<1>()?[0] == 1)
    }
    fn read_i32(&mut self) -> Result<i32, KaizenError> {
        let i = i32::from_le_bytes(self.read_bytes()?);
        if (-4096..4096).contains(&i) {
            Ok(i)
        } else {
            Err(KaizenError::NumberOutsideAllowedRange(i))
        }
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
        match self.read_exact(&mut buf) {
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
        Ok(Instruction { column, row, kind, arm, distance, grab })
    }
}
impl SolutionReader for &[u8] {}

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

pub enum Version {
    V1,
}

impl ParseEnum<i32> for Version {
    fn try_from(value: i32) -> Result<Self, KaizenError> {
        match value {
            10 => Ok(Version::V1),
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