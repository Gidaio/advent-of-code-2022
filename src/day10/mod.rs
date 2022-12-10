mod cpu;
pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use crate::{BoxedResult, TimedResult};
use cpu::*;
