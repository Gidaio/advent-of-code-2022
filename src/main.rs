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
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;

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
        "Day 6, part 2: {}\n",
        format_timed_result(day6::part2::find_start_of_message_marker())
    );

    println!(
        "Day 7, part 1: {}",
        format_timed_result(day7::part1::find_directory_sizes())
    );
    println!(
        "Day 7, part 2: {}\n",
        format_timed_result(day7::part2::find_directory_to_delete())
    );

    println!(
        "Day 8, part 1: {}",
        format_timed_result(day8::part1::count_visible_trees())
    );
    println!(
        "Day 8, part 2: {}\n",
        format_timed_result(day8::part2::get_best_scenic_score())
    );

    println!(
        "Day 9, part 1: {}",
        format_timed_result(day9::part1::count_unique_tail_positions())
    );
    println!(
        "Day 9, part 2: {}\n",
        format_timed_result(day9::part2::count_unique_long_tail_positions())
    );

    println!(
        "Day 10, part 1: {}",
        format_timed_result(day10::part1::calculate_sum_of_signal_strengths())
    );
    println!("Day 10, part 2:");
    let day10_part2_result = day10::part2::print_crt();
    match day10_part2_result {
        Ok((_, duration)) => println!("Took {} ms\n", duration.as_millis()),
        Err(error) => println!("Errored: {}\n", error),
    }

    println!(
        "Day 11, part 1: {}",
        format_timed_result(day11::part1::calculate_calm_monkey_business())
    );
    println!(
        "Day 11, part 2: {}\n",
        format_timed_result(day11::part2::calculate_anxious_monkey_business())
    );

    println!(
        "Day 12, part 1: {}",
        format_timed_result(day12::part1::find_best_path_length()),
    );
}

fn format_timed_result<T: fmt::Display>(result: TimedResult<T>) -> String {
    match result {
        Ok((value, duration)) => format!("{} (in {} ms)", value, duration.as_millis()),
        Err(error) => format!("Errored: {}", error),
    }
}
