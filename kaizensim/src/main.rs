use std::io::Read;
use kaizensim::KaizenError;

fn main() -> Result<(), KaizenError> {
    let mut args = std::env::args();
    let _ = args.next();
    let command = match args.next() {
        Some(command) => Command::try_from(command)?,
        None => Command::Score,
    };
    let path = args.next();
    Ok(match command {
        Command::Score => println!("{:?}", kaizensim::score(&read(&path)?)?),
    })
}

fn read(path: &Option<String>) -> Result<Vec<u8>, KaizenError> {
    match path {
        Some(path) => read_file(path),
        None => read_stdin(),
    }
}

fn read_file(path: &String) -> Result<Vec<u8>, KaizenError> {
    std::fs::read(path).or(Err(KaizenError::CouldNotReadFile))
}

fn read_stdin() -> Result<Vec<u8>, KaizenError> {
    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf).or(Err(KaizenError::CouldNotReadStdIn))?;
    Ok(buf)
}

enum Command {
    Score,
}

impl Command {
    fn try_from(value: String) -> Result<Self, KaizenError> {
        match &value[..] {
            "score" => Ok(Command::Score),
            _ => Err(KaizenError::UnknownCommand(value)),
        }
    }
}