mod monkey;
pub mod part1;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;

use crate::{BoxedResult, TimedResult};

use monkey::Monkey;
