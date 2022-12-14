use std::cmp::Ordering;
use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::Bytes;

const OPEN_BRACKET: u8 = '[' as u8;
const CLOSE_BRACKET: u8 = ']' as u8;
const COMMA: u8 = ',' as u8;
const DIGIT_START: u8 = '0' as u8;
const DIGIT_END: u8 = '9' as u8;

#[derive(Debug)]
pub enum ListParseError {
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
pub struct List {
    pub items: Vec<ListItem>,
}

#[derive(Debug)]
pub enum ListItem {
    List(List),
    Integer(usize),
}

impl From<&usize> for List {
    fn from(value: &usize) -> Self {
        Self {
            items: vec![ListItem::Integer(*value)],
        }
    }
}

impl PartialEq for List {
    fn eq(&self, other: &Self) -> bool {
        let min_length = usize::min(self.items.len(), other.items.len());

        for i in 0..min_length {
            let left_item = &self.items[i];
            let right_item = &other.items[i];

            match left_item {
                ListItem::List(left_value) => match right_item {
                    ListItem::List(right_value) => {
                        if !left_value.eq(right_value) {
                            return false;
                        }
                    }
                    ListItem::Integer(right_value) => {
                        if !left_value.eq(&right_value.into()) {
                            return false;
                        }
                    }
                },
                ListItem::Integer(left_value) => match right_item {
                    ListItem::List(right_value) => {
                        if !right_value.eq(&left_value.into()) {
                            return false;
                        }
                    }
                    ListItem::Integer(right_value) => {
                        if left_value != right_value {
                            return false;
                        }
                    }
                },
            }
        }

        if self.items.len() == other.items.len() {
            true
        } else {
            false
        }
    }
}

impl Eq for List {}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let min_length = usize::min(self.items.len(), other.items.len());

        for i in 0..min_length {
            let left_item = &self.items[i];
            let right_item = &other.items[i];

            match left_item {
                ListItem::List(left_value) => match right_item {
                    ListItem::List(right_value) => {
                        if left_value.ne(right_value) {
                            return left_value.partial_cmp(right_value);
                        }
                    }
                    ListItem::Integer(right_value) => {
                        let right_list = List::from(right_value);
                        if left_value.ne(&right_list) {
                            return left_value.partial_cmp(&right_list);
                        }
                    }
                },
                ListItem::Integer(left_value) => match right_item {
                    ListItem::List(right_value) => {
                        let left_list = List::from(left_value);
                        if left_list.ne(&right_value) {
                            return left_list.partial_cmp(&right_value);
                        }
                    }
                    ListItem::Integer(right_value) => {
                        if left_value != right_value {
                            return left_value.partial_cmp(right_value);
                        }
                    }
                },
            }
        }

        if self.items.len() < other.items.len() {
            Some(Ordering::Less)
        } else if self.items.len() > other.items.len() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl List {
    pub fn parse(bytes: &mut Peekable<Bytes>) -> ListParseResult<Self> {
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
                return Ok(Self { items });
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

        Ok(Self { items })
    }

    fn parse_list_item(bytes: &mut Peekable<Bytes>) -> ListParseResult<ListItem> {
        // println!("Parsing list item...");
        let next_byte = *(bytes.peek().ok_or(ListParseError::ExpectedListItem(0))?);
        // println!("Peeking at: {}", next_byte as char);
        if Self::is_digit(next_byte) {
            Ok(ListItem::Integer(Self::parse_integer(bytes)?))
        } else if next_byte == OPEN_BRACKET {
            Ok(ListItem::List(Self::parse(bytes)?))
        } else {
            Err(ListParseError::ExpectedListItem(next_byte))
        }
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
