use super::*;

pub fn find_most_scenic_path_length() -> crate::TimedResult<usize> {
    let start_time = Instant::now();

    let file = File::open("inputs/day12.txt")?;
    let map = Map::try_from(file)?;

    Ok((map.find_shortest_path(), start_time.elapsed()))
}
