use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use super::*;

#[derive(Debug)]
enum ListSetParseError {
    MissingRightSide,
}

impl fmt::Display for ListSetParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRightSide => write!(f, "Missing right side list."),
        }
    }
}

impl Error for ListSetParseError {}

#[derive(Debug)]
struct ListPair {
    left: List,
    right: List,
}

impl ListPair {
    fn is_ordered(&self) -> bool {
        self.left < self.right
    }
}

#[derive(Debug)]
struct ListSet {
    pairs: Vec<ListPair>,
}

impl TryFrom<File> for ListSet {
    type Error = Box<dyn Error>;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let reader = BufReader::new(value);
        let mut lines = reader.lines();

        let mut pairs = Vec::<ListPair>::new();

        while let Some(left) = lines.next() {
            let left = left?;
            let mut left_bytes = left.bytes().peekable();
            let right = lines.next().ok_or(ListSetParseError::MissingRightSide)??;
            let mut right_bytes = right.bytes().peekable();

            let left_list = List::parse(&mut left_bytes)?;
            let right_list = List::parse(&mut right_bytes)?;

            pairs.push(ListPair {
                left: left_list,
                right: right_list,
            });

            // Read out the blank line between.
            lines.next();
        }

        Ok(Self { pairs })
    }
}

pub fn find_pairs_in_correct_order() -> BoxedResult<usize> {
    let list_set: ListSet = File::open("inputs/day13.txt")?.try_into()?;

    let mut answer: usize = 0;
    for i in 0..list_set.pairs.len() {
        if list_set.pairs[i].is_ordered() {
            answer += i + 1;
        }
    }

    Ok(answer)
}
