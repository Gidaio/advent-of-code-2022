use std::error;
use std::fmt;
use std::result;
use std::time;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

type BoxedResult<T> = result::Result<T, Box<dyn error::Error>>;
type TimedResult<T> = BoxedResult<(T, time::Duration)>;

fn main() {
    println!(
        "Day 1, part 1: {}",
        day1::part1::find_elf_with_most_calories()
    );
    println!(
        "Day 1, part 2: {}\n",
        day1::part2::find_three_elves_with_most_calories()
    );

    println!(
        "Day 2, part 1: {}",
        day2::part1::calculate_score_of_strategy_guide().unwrap()
    );
    println!(
        "Day 2, part 2: {}\n",
        day2::part2::calculate_score_of_strategy_guide().unwrap()
    );

    println!(
        "Day 3, part 1: {}",
        day3::part1::get_priority_of_incorrect_items().unwrap()
    );
    println!(
        "Day 3, part 2: {}\n",
        day3::part2::get_priority_of_team_badges().unwrap()
    );

    println!("Day 4, part 1: {}", day4::part1::find_subsets().unwrap());
    println!("Day 4, part 2: {}\n", day4::part2::find_overlaps().unwrap());

    println!(
        "Day 5, part 1: {}",
        format_timed_result(day5::part1::get_tops_of_stacks())
    );
    println!(
        "Day 5, part 2: {}\n",
        format_timed_result(day5::part2::get_tops_of_stacks_for_9001())
    );

    println!(
        "Day 6, part 1: {}",
        format_timed_result(day6::part1::find_start_of_packet_marker())
    );
    println!(
        "Day 6, part 2: {}",
        format_timed_result(day6::part2::find_start_of_message_marker())
    );
}

fn format_timed_result<T: fmt::Display>(result: TimedResult<T>) -> String {
    match result {
        Ok((value, duration)) => format!("{} (in {} ms)", value, duration.as_millis()),
        Err(error) => format!("Errored: {}", error),
    }
}
