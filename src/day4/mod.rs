pub mod part1;
pub mod part2;

use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
enum ParsingError {
    Pair,
    Assignment,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParsingError::Pair => write!(f, "Invalid pair."),
            ParsingError::Assignment => write!(f, "Invalid assignment."),
        }
    }
}

impl error::Error for ParsingError {}

struct Pair(Assignment, Assignment);

impl Pair {
    fn new(line: &str) -> Result<Self> {
        let raw_assignments = line.split(',').collect::<Vec<&str>>();

        if raw_assignments.len() != 2 {
            return Err(ParsingError::Pair.into());
        }

        Ok(Self(
            Assignment::new(raw_assignments[0])?,
            Assignment::new(raw_assignments[1])?,
        ))
    }
}

struct Assignment {
    start: usize,
    end: usize,
}

impl Assignment {
    fn new(part: &str) -> Result<Self> {
        // println!("Got assignment {}", part);

        let parts = part.split('-').collect::<Vec<&str>>();

        if parts.len() != 2 {
            return Err(ParsingError::Assignment.into());
        }

        Ok(Self {
            start: parts[0].parse::<usize>()?,
            end: parts[1].parse::<usize>()?,
        })
    }
}
