use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

use super::*;

pub fn get_priority_of_incorrect_items() -> Result<usize> {
    let file = fs::File::open("inputs/day3.txt")?;
    let reader = io::BufReader::new(file);

    let mut priority_sum: usize = 0;
    for line in reader.lines() {
        let line = line?;
        let compartment_size = line.len() / 2;

        let first_compartment = HashSet::<u8>::from_iter((&line[0..compartment_size]).bytes());
        let second_compartment = HashSet::<u8>::from_iter((&line[compartment_size..]).bytes());

        let mut unique_items = first_compartment;
        unique_items.retain(|item| second_compartment.contains(item));

        if unique_items.len() != 1 {
            return Err(BadCommonItems(unique_items.len()).into());
        }

        for item in unique_items {
            priority_sum += get_priority_of_item(item)?;
        }
    }

    Ok(priority_sum)
}
