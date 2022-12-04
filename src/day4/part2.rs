use std::fs;
use std::io::{self, BufRead};

use super::*;

impl Pair {
    fn overlaps(&self) -> bool {
        (self.0.start >= self.1.start && self.0.start <= self.1.end)
            || (self.0.end >= self.1.start && self.0.end <= self.1.end)
            || (self.1.start >= self.0.start && self.1.start <= self.0.end)
            || (self.1.end >= self.0.start && self.1.end <= self.0.end)
    }
}

pub fn find_overlaps() -> Result<usize> {
    let file = fs::File::open("inputs/day4.txt")?;
    let reader = io::BufReader::new(file);

    let mut overlaps: usize = 0;

    for line in reader.lines() {
        if Pair::new(&line?)?.overlaps() {
            overlaps += 1;
        }
    }

    Ok(overlaps)
}
