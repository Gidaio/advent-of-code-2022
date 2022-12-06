use std::error;
use std::fmt;
use std::fs;
use std::time;

use super::*;

#[derive(Debug)]
struct PacketStartNotFound {}

impl fmt::Display for PacketStartNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Start of packet not found.")
    }
}

impl error::Error for PacketStartNotFound {}

pub fn find_start_of_packet() -> TimedResult<usize> {
    let file = fs::read_to_string("inputs/day6.txt")?;

    let start_time = time::Instant::now();
    let bytes = file.as_bytes();

    let mut start_index: usize = 0;

    for end_index in 1..bytes.len() {
        if start_index + 4 == end_index {
            return Ok((end_index, start_time.elapsed()));
        }

        for i in start_index..end_index {
            if bytes[i] == bytes[end_index] {
                start_index = i + 1;
                break;
            }
        }
    }

    Err(PacketStartNotFound {}.into())
}
