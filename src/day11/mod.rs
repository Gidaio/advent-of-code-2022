mod monkey;
pub mod part1;
pub mod part2;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;

use crate::{BoxedResult, TimedResult};

use monkey::Monkey;

pub fn calculate_monkey_business(rounds: usize, divisor: usize) -> TimedResult<usize> {
    let start_time = Instant::now();

    let file = File::open("inputs/day11.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut monkey_infos = Vec::<(Monkey, usize)>::new();

    loop {
        let monkey = Monkey::parse_from_lines(&mut lines)?;
        monkey_infos.push((monkey, 0));
        if let None = lines.next() {
            break;
        }
    }

    for _ in 0..rounds {
        for monkey_index in 0..monkey_infos.len() {
            let thrown_items = {
                let monkey_info = &mut monkey_infos[monkey_index];
                let inspected = monkey_info.0.inspect_own_items(divisor);
                monkey_info.1 += inspected.len();
                inspected
            };

            for item in thrown_items.into_iter() {
                monkey_infos[item.0].0.items.push(item.1);
            }
        }
    }

    monkey_infos.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let top_two_infos = &monkey_infos[monkey_infos.len() - 2..];

    Ok((
        top_two_infos[0].1 * top_two_infos[1].1,
        start_time.elapsed(),
    ))
}
