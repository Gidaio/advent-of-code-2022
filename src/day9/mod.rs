pub mod part1;
pub mod part2;
mod rope;
mod rope_move;
mod vector2;

use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use rope::Rope;
use rope_move::RopeMove;
use vector2::Vector2;

fn count_unique_tail_positions_of_length(len: usize) -> crate::TimedResult<usize> {
    let start_time = Instant::now();
    let file = File::open("inputs/day9.txt")?;
    let reader = BufReader::new(file);
    let mut rope = Rope::new(len);

    for line in reader.lines() {
        let rope_move = RopeMove::from_str(&line?)?;
        rope.move_times_in_direction(rope_move.direction, rope_move.distance);
    }

    Ok((rope.get_tail_positions().len(), start_time.elapsed()))
}
