use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::{BoxedResult, TimedResult};

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

struct Rope {
    head: Point,
    tail: Point,
    tail_points: HashSet<Point>,
}

impl Rope {
    fn new() -> Self {
        let mut rope = Self {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
            tail_points: HashSet::new(),
        };

        rope.tail_points.insert(Point { x: 0, y: 0 });

        rope
    }

    fn do_move(&mut self, move_to_do: Move) {
        match move_to_do {
            Move::Right(distance) => {
                for _ in 0..distance {
                    self.move_right()
                }
            }
            Move::Up(distance) => {
                for _ in 0..distance {
                    self.move_up()
                }
            }
            Move::Left(distance) => {
                for _ in 0..distance {
                    self.move_left()
                }
            }
            Move::Down(distance) => {
                for _ in 0..distance {
                    self.move_down()
                }
            }
        }
    }

    fn move_right(&mut self) {
        self.head.x += 1;

        if self.head.x > self.tail.x + 1 {
            self.tail = Point {
                x: self.head.x - 1,
                y: self.head.y,
            };
            self.tail_points.insert(self.tail.clone());
        }
    }

    fn move_up(&mut self) {
        self.head.y -= 1;

        if self.head.y < self.tail.y - 1 {
            self.tail = Point {
                x: self.head.x,
                y: self.head.y + 1,
            };
            self.tail_points.insert(self.tail.clone());
        }
    }

    fn move_left(&mut self) {
        self.head.x -= 1;

        if self.head.x < self.tail.x - 1 {
            self.tail = Point {
                x: self.head.x + 1,
                y: self.head.y,
            };
            self.tail_points.insert(self.tail.clone());
        }
    }

    fn move_down(&mut self) {
        self.head.y += 1;

        if self.head.y > self.tail.y + 1 {
            self.tail = Point {
                x: self.head.x,
                y: self.head.y - 1,
            };
            self.tail_points.insert(self.tail.clone());
        }
    }
}

#[derive(Debug)]
enum MoveParseError {
    MissingDirection,
    MissingDistance,
    InvalidDirection,
}

impl fmt::Display for MoveParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::MissingDirection => write!(f, "Missing direction."),
            Self::MissingDistance => write!(f, "Missing distance."),
            Self::InvalidDirection => write!(f, "Invalid direction."),
        }
    }
}

impl Error for MoveParseError {}

enum Move {
    Right(usize),
    Up(usize),
    Left(usize),
    Down(usize),
}

impl Move {
    fn from_str(line: &str) -> BoxedResult<Self> {
        let mut tokens = line.split_whitespace();
        let direction = tokens.next().ok_or(MoveParseError::MissingDirection)?;
        let distance = tokens
            .next()
            .ok_or(MoveParseError::MissingDistance)?
            .parse::<usize>()?;

        match direction {
            "R" => Ok(Self::Right(distance)),
            "U" => Ok(Self::Up(distance)),
            "L" => Ok(Self::Left(distance)),
            "D" => Ok(Self::Down(distance)),
            _ => Err(MoveParseError::InvalidDirection.into()),
        }
    }
}

pub fn count_unique_tail_positions() -> TimedResult<usize> {
    let start_time = Instant::now();
    let file = File::open("inputs/day9.txt")?;
    let reader = BufReader::new(file);
    let mut rope = Rope::new();

    for line in reader.lines() {
        let move_to_do = Move::from_str(&line?)?;
        rope.do_move(move_to_do);
    }

    Ok((rope.tail_points.len(), start_time.elapsed()))
}
