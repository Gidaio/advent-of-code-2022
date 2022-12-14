use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::Bytes;

use super::*;

const OPEN_BRACKET: u8 = '[' as u8;
const CLOSE_BRACKET: u8 = ']' as u8;
const COMMA: u8 = ',' as u8;
const DIGIT_START: u8 = '0' as u8;
const DIGIT_END: u8 = '9' as u8;

#[derive(Debug)]
enum ListParseError {
    ExpectedDigit(u8),
    ExpectedList(u8),
    ExpectedListItem(u8),
    ExpectedEndOfList(u8),
}

type ListParseResult<T> = Result<T, ListParseError>;

impl fmt::Display for ListParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedDigit(found) => {
                write!(f, "Expected a digit, found '{}'.", *found as char)
            }
            Self::ExpectedList(found) => write!(f, "Expected '[', found '{}'.", *found as char),
            Self::ExpectedListItem(found) => {
                write!(f, "Expected integer or list, found '{}'.", *found as char)
            }
            Self::ExpectedEndOfList(found) => {
                write!(f, "Expected ']', found '{}'.", *found as char)
            }
        }
    }
}

impl Error for ListParseError {}

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
enum ListItem {
    List(List),
    Integer(usize),
}

type List = Vec<ListItem>;

#[derive(Debug)]
struct ListPair {
    left: List,
    right: List,
}

impl ListPair {
    fn is_ordered(&self) -> bool {
        match Self::compare(&self.left, &self.right) {
            Ordering::Less => true,
            _ => false,
        }
    }

    fn compare(left: &List, right: &List) -> Ordering {
        let min_length = usize::min(left.len(), right.len());

        for i in 0..min_length {
            let left_item = &left[i];
            let right_item = &right[i];

            match left_item {
                ListItem::List(left_value) => match right_item {
                    ListItem::List(right_value) => match Self::compare(left_value, right_value) {
                        Ordering::Equal => (),
                        any => return any,
                    },
                    ListItem::Integer(right_value) => {
                        match Self::compare(left_value, &vec![ListItem::Integer(*right_value)]) {
                            Ordering::Equal => (),
                            any => return any,
                        }
                    }
                },
                ListItem::Integer(left_value) => match right_item {
                    ListItem::List(right_value) => {
                        match Self::compare(&vec![ListItem::Integer(*left_value)], right_value) {
                            Ordering::Equal => (),
                            any => return any,
                        }
                    }
                    ListItem::Integer(right_value) => match left_value.cmp(right_value) {
                        Ordering::Equal => (),
                        any => return any,
                    },
                },
            }
        }

        if left.len() < right.len() {
            Ordering::Less
        } else if left.len() > right.len() {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
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

            let left_list = Self::parse_list(&mut left_bytes)?;
            let right_list = Self::parse_list(&mut right_bytes)?;

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

impl ListSet {
    fn parse_list_item(bytes: &mut Peekable<Bytes>) -> ListParseResult<ListItem> {
        // println!("Parsing list item...");
        let next_byte = *(bytes.peek().ok_or(ListParseError::ExpectedListItem(0))?);
        // println!("Peeking at: {}", next_byte as char);
        if Self::is_digit(next_byte) {
            Ok(ListItem::Integer(Self::parse_integer(bytes)?))
        } else if next_byte == OPEN_BRACKET {
            Ok(ListItem::List(Self::parse_list(bytes)?))
        } else {
            Err(ListParseError::ExpectedListItem(next_byte))
        }
    }

    fn parse_list(bytes: &mut Peekable<Bytes>) -> ListParseResult<Vec<ListItem>> {
        // println!("Parsing list...");
        let start_of_list = bytes.next().ok_or(ListParseError::ExpectedList(0))?;
        // println!("Got start of list: {}", start_of_list as char);
        if start_of_list != OPEN_BRACKET {
            return Err(ListParseError::ExpectedList(start_of_list));
        }

        let mut items = Vec::<ListItem>::new();

        let next = *(bytes.peek().ok_or(ListParseError::ExpectedEndOfList(0))?);
        // println!("Peeking at: {}", next as char);
        match next {
            CLOSE_BRACKET => {
                bytes.next();
                return Ok(items);
            }
            OPEN_BRACKET => {
                items.push(Self::parse_list_item(bytes)?);
            }
            val => {
                if Self::is_digit(val) {
                    items.push(Self::parse_list_item(bytes)?);
                } else {
                    return Err(ListParseError::ExpectedListItem(val));
                }
            }
        }

        while let Some(next_byte) = bytes.peek() {
            // println!("Peeking at: {}", *next_byte as char);
            if *next_byte == COMMA {
                bytes.next();
                items.push(Self::parse_list_item(bytes)?);
            } else {
                break;
            }
        }

        let end_of_list = bytes.next().ok_or(ListParseError::ExpectedEndOfList(0))?;
        if end_of_list != CLOSE_BRACKET {
            return Err(ListParseError::ExpectedEndOfList(end_of_list));
        }

        Ok(items)
    }

    fn parse_integer(bytes: &mut Peekable<Bytes>) -> ListParseResult<usize> {
        // println!("Parsing integer...");
        let mut value = Self::parse_digit(bytes)?;
        // println!("Current value: {}", value);
        while let Some(next_byte) = bytes.peek() {
            // println!("Peeking at: {}", *next_byte as char);
            if Self::is_digit(*next_byte) {
                value *= 10;
                value += Self::parse_digit(bytes)?;
            } else {
                break;
            }
        }

        Ok(value)
    }

    fn parse_digit(bytes: &mut Peekable<Bytes>) -> Result<usize, ListParseError> {
        let next_byte = bytes.next().ok_or(ListParseError::ExpectedDigit(0))?;

        if Self::is_digit(next_byte) {
            Ok((next_byte - DIGIT_START) as usize)
        } else {
            Err(ListParseError::ExpectedDigit(next_byte))
        }
    }

    fn is_digit(byte: u8) -> bool {
        byte >= DIGIT_START && byte <= DIGIT_END
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
