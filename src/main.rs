#![allow(dead_code)]
use std::{io::{BufReader, BufRead, self}, fs::File, path::Path, num::ParseIntError};

mod day1;
mod day2;
mod day3;

fn main() {
    let solver = day3::solve;

    let now = std::time::Instant::now();
    let res = solver();
    let took = now.elapsed();

    if let Err(err) = res {
        println!("Solver failed: {}", err);
    }

    println!("Took {} ms ({} ns)", took.as_millis(), took.as_nanos());
}

pub fn get_input<P: AsRef<Path>>(file: P) -> Result<BufReader<File>> {
    let f = File::open(file)?;

    let reader = BufReader::new(f);
    Ok(reader)
}

pub fn get_input_lines<P: AsRef<Path>>(file: P) -> Result<impl Iterator<Item = String>> {
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
    InvalidInput,
}

type Result<T> = std::result::Result<T, Error>;

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