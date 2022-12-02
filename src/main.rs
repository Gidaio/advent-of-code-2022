mod day1;

fn main() {
    println!(
        "Day 1, part 1: {}",
        day1::part1::find_elf_with_most_calories()
    );
    println!(
        "Day 1, part 2: {}",
        day1::part2::find_three_elves_with_most_calories()
    );
}
