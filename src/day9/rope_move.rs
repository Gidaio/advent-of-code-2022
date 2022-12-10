use std::error::Error;

use super::*;

#[derive(Debug)]
enum RopeMoveParseError {
    MissingDirection,
    MissingDistance,
    InvalidDirection,
}

impl fmt::Display for RopeMoveParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::MissingDirection => write!(f, "Missing direction."),
            Self::MissingDistance => write!(f, "Missing distance."),
            Self::InvalidDirection => write!(f, "Invalid direction."),
        }
    }
}

impl Error for RopeMoveParseError {}

pub struct RopeMove {
    pub direction: Vector2,
    pub distance: usize,
}

impl RopeMove {
    pub fn from_str(line: &str) -> crate::BoxedResult<Self> {
        let mut tokens = line.split_whitespace();
        let direction_character = tokens.next().ok_or(RopeMoveParseError::MissingDirection)?;
        let distance = tokens
            .next()
            .ok_or(RopeMoveParseError::MissingDistance)?
            .parse::<usize>()?;

        let direction = match direction_character {
            "R" => Ok(Vector2::new(1, 0)),
            "U" => Ok(Vector2::new(0, -1)),
            "L" => Ok(Vector2::new(-1, 0)),
            "D" => Ok(Vector2::new(0, 1)),
            _ => Err(RopeMoveParseError::InvalidDirection),
        }?;

        Ok(Self {
            direction,
            distance,
        })
    }
}
