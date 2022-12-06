use super::*;

pub fn find_start_of_message_marker() -> TimedResult<usize> {
    find_start_of_marker(14)
}
