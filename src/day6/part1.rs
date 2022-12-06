use super::*;

pub fn find_start_of_packet_marker() -> TimedResult<usize> {
    find_start_of_marker(4)
}
