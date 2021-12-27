#![allow(dead_code)]
use std::{io::{BufReader, BufRead, self}, fs::File, path::Path, num::ParseIntError};

mod util;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

fn main() -> crate::Result<()> {
    let solvers = [
        day1::solve,
        day2::solve,
        day3::solve,
        day4::solve,
        day5::solve,
        day6::solve,
        day7::solve,
        day8::solve,
        day9::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
        day17::solve,
        day18::solve,
        day19::solve,
        day20::solve,
        day21::solve,
        day22::solve,
        day23::solve,
        day24::solve,
        day25::solve,
    ];

    for (i, solver) in solvers.into_iter().enumerate() {
        let day = i + 1;
        if day != 1 {
            continue;
        }

        println!("Day {}", day);

        let now = std::time::Instant::now();
        let _ = solver()?;
        let took = now.elapsed();

        println!("Took {} ms ({} ns)", took.as_millis(), took.as_nanos());
        println!()
    }

    Ok(())
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