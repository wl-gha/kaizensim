use std::io::{Error, Read};
use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};
use kaizensim::KaizenError;

fn main() -> Result<(), Error> {
    let cli = Cli::parse();
    match cli.command {
        Command::Score(files) => {
            if files.path.is_empty() {
                println_json(kaizensim::score(&read_stdin()?))
            } else {
                for path in files.path {
                    println_json(kaizensim::score(&read_file(&path)?))
                }
            }
        },
    }
    Ok(())
}

fn println_json<T>(result: Result<T, KaizenError>)
where
    T: serde::Serialize,
{
    match result {
        Ok(v) => println!("{}", serde_json::to_string(&v).unwrap()),
        Err(e) => println!("{}", serde_json::to_string(&e).unwrap()),
    }
}

fn println<T>(result: Result<T, KaizenError>)
where
    T: std::fmt::Debug,
{
    match result {
        Ok(v) => println!("{v:?}"),
        Err(e) => println!("{e:?}"),
    }
}

fn read(path: &Option<PathBuf>) -> Result<Vec<u8>, Error> {
    match path {
        Some(path) => read_file(path),
        None => read_stdin(),
    }
}

fn read_file(path: &PathBuf) -> Result<Vec<u8>, Error> {
    std::fs::read(path)
}

fn read_stdin() -> Result<Vec<u8>, Error> {
    let mut buf = vec![];
    std::io::stdin().read_to_end(&mut buf)?;
    Ok(buf)
}

#[derive(Parser)]
#[command(version, about = "Tool for Kaizen: A Factory Story solutions")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Return the score of a solution")]
    Score(SolutionFiles),
}

#[derive(Args)]
struct SolutionFiles {
    #[arg(help = "Path(s) to solution(s), uses STDIN if omitted")]
    path: Vec<PathBuf>,
}

#[derive(Args)]
struct SolutionFile {
    #[arg(help = "Path to solution")]
    path: PathBuf,
}