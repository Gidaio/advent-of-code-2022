use std::fmt;

use super::*;

#[derive(Debug)]
struct NoPathError {}

impl fmt::Display for NoPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Couldn't find a path!")
    }
}

impl Error for NoPathError {}

pub fn find_best_path_length() -> crate::TimedResult<usize> {
    let start_time = Instant::now();

    let file = File::open("inputs/day12.txt")?;
    let map = Map::try_from(file)?;

    Ok((
        map.find_shortest_path_from(map.start)
            .ok_or(NoPathError {})?,
        start_time.elapsed(),
    ))
}
