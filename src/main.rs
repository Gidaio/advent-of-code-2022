mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

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
        day5::part1::get_tops_of_stacks().unwrap()
    );
    println!(
        "Day 5, part 2: {}",
        day5::part2::get_tops_of_stacks_for_9001().unwrap()
    );
}
