mod day1;
mod day2;
mod day3;

fn main() {
    println!(
        "Day 1, part 1: {}",
        day1::part1::find_elf_with_most_calories()
    );
    println!(
        "Day 1, part 2: {}",
        day1::part2::find_three_elves_with_most_calories()
    );

    println!(
        "Day 2, part 1: {}",
        day2::part1::calculate_score_of_strategy_guide().unwrap()
    );
    println!(
        "Day 2, part 2: {}",
        day2::part2::calculate_score_of_strategy_guide().unwrap()
    );

    println!(
        "Day 3, part 1: {}",
        day3::part1::get_priority_of_incorrect_items().unwrap()
    );
}
