use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};

use super::*;

#[derive(Debug)]
struct WrongNumberOfRucksacks;

impl fmt::Display for WrongNumberOfRucksacks {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Number of rucksacks was not a multiple of three!")
    }
}

impl error::Error for WrongNumberOfRucksacks {}

pub fn get_priority_of_team_badges() -> Result<usize> {
    let file = fs::File::open("inputs/day3.txt")?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut priority_sum: usize = 0;

    loop {
        let first_rucksack = if let Some(i) = lines.next() {
            HashSet::<u8>::from_iter(i?.bytes())
        } else {
            break;
        };

        let second_rucksack = if let Some(i) = lines.next() {
            Ok(HashSet::<u8>::from_iter(i?.bytes()))
        } else {
            Err(WrongNumberOfRucksacks)
        }?;

        let third_rucksack = if let Some(i) = lines.next() {
            Ok(HashSet::<u8>::from_iter(i?.bytes()))
        } else {
            Err(WrongNumberOfRucksacks)
        }?;

        let mut common_items = first_rucksack;
        common_items.retain(|item| second_rucksack.contains(item) && third_rucksack.contains(item));

        if common_items.len() != 1 {
            return Err(BadCommonItems(common_items.len()).into());
        }

        for item in common_items {
            priority_sum += get_priority_of_item(item)?;
        }
    }

    Ok(priority_sum)
}
