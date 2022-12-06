pub mod part1;
pub mod part2;

use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

const STACK_COUNT: usize = 9;

#[derive(Debug)]
enum MoveError {
    ParseError,
    BadFromStack(usize),
    BadToStack(usize),
    NotEnoughCrates(usize),
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::ParseError => write!(f, "Couldn't parse move."),
            Self::BadFromStack(stack_index) => write!(f, "Bad from stack: {}", stack_index),
            Self::BadToStack(stack_index) => write!(f, "Bad to stack: {}", stack_index),
            Self::NotEnoughCrates(stack_index) => {
                write!(f, "Not enough crates in stack {}", stack_index)
            }
        }
    }
}

impl error::Error for MoveError {}

struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

impl Move {
    fn from_str(line: &str) -> Result<Self> {
        let mut line_parts = line.split_whitespace();

        let mut quantity: usize = 0;
        let mut from = 0;
        let mut to = 0;

        while let Some(part) = line_parts.next() {
            if part == "move" {
                quantity = if let Some(i) = line_parts.next() {
                    Ok(i.parse::<usize>()?)
                } else {
                    Err(MoveError::ParseError)
                }?;
            } else if part == "from" {
                from = if let Some(i) = line_parts.next() {
                    Ok(i.parse::<usize>()? - 1)
                } else {
                    Err(MoveError::ParseError)
                }?;
            } else if part == "to" {
                to = if let Some(i) = line_parts.next() {
                    Ok(i.parse::<usize>()? - 1)
                } else {
                    Err(MoveError::ParseError)
                }?;
            } else {
                return Err(MoveError::ParseError.into());
            }
        }

        Ok(Self { quantity, from, to })
    }
}

type Stack = Vec<char>;
struct UnloadSpace {
    stacks: [Stack; STACK_COUNT],
}

impl UnloadSpace {
    fn new() -> Self {
        Self {
            stacks: [
                vec!['J', 'H', 'P', 'M', 'S', 'F', 'N', 'V'],
                vec!['S', 'R', 'L', 'M', 'J', 'D', 'Q'],
                vec!['N', 'Q', 'D', 'H', 'C', 'S', 'W', 'B'],
                vec!['R', 'S', 'C', 'L'],
                vec!['M', 'V', 'T', 'P', 'F', 'B'],
                vec!['T', 'R', 'Q', 'N', 'C'],
                vec!['G', 'V', 'R'],
                vec!['C', 'Z', 'S', 'P', 'D', 'L', 'R'],
                vec!['D', 'S', 'J', 'V', 'G', 'P', 'B', 'F'],
            ],
        }
    }
}
