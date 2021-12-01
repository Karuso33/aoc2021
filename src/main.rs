#![allow(dead_code)]
use std::{io::{BufReader, BufRead, self}, fs::File, path::Path, num::ParseIntError};

mod day1;

fn main() {
    // let path = Path::new("problems/test");
    let path = Path::new("problems/problem1");
    let solver = day1::solve_without_allocating;

    let res = solver(Path::new(path));

    if let Err(err) = res {
        println!("Solver failed: {}", err);
    }
}

pub fn get_input(file: &Path) -> Result<BufReader<File>, Error> {
    let f = File::open(file)?;

    let reader = BufReader::new(f);
    Ok(reader)
}

pub fn get_input_lines(file: &Path) -> Result<impl Iterator<Item = String>, Error> {
    Ok(get_input(file)?
        .lines()
        .map(|s| s.expect("Can these IO errors even happen?")))
}

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    ParseIntError(ParseIntError),
    NoInput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Self {
        Error::IOError(val)
    }
}

impl From<ParseIntError> for Error {
    fn from(inner: ParseIntError) -> Self {
        Error::ParseIntError(inner)
    }
}