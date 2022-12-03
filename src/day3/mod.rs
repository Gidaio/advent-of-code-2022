pub mod part1;
pub mod part2;

use std::error;
use std::fmt;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct PriorityError(u8);

impl fmt::Display for PriorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized character {}", self.0)
    }
}

impl error::Error for PriorityError {}

#[derive(Debug)]
struct BadCommonItems(usize);

impl fmt::Display for BadCommonItems {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad number of common items. Expected 1, got {}", self.0)
    }
}

impl error::Error for BadCommonItems {}

const LITTLE_A: u8 = 'a' as u8;
const LITTLE_Z: u8 = 'z' as u8;
const BIG_A: u8 = 'A' as u8;
const BIG_Z: u8 = 'Z' as u8;

fn get_priority_of_item(item: u8) -> Result<usize> {
    if item >= LITTLE_A && item <= LITTLE_Z {
        Ok((item - LITTLE_A + 1) as usize)
    } else if item >= BIG_A && item <= BIG_Z {
        Ok((item - BIG_A + 27) as usize)
    } else {
        Err(PriorityError(item).into())
    }
}
