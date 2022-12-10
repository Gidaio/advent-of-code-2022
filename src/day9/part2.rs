use super::*;

pub fn count_unique_long_tail_positions() -> crate::TimedResult<usize> {
    count_unique_tail_positions_of_length(9)
}
