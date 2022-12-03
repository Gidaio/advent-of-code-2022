use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
#[derive(Debug)]
struct PriorityError(u8);

impl fmt::Display for PriorityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unrecognized character {}", self.0)
    }
}

impl Error for PriorityError {}

const LITTLE_A: u8 = 'a' as u8;
const LITTLE_Z: u8 = 'z' as u8;
const BIG_A: u8 = 'A' as u8;
const BIG_Z: u8 = 'Z' as u8;

pub fn get_priority_of_team_badges() -> Result<usize> {
    let file = fs::File::open("inputs/day3.txt")?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();

    let mut priority_sum: usize = 0;

    loop {
        let first_rucksack: HashSet<u8> = if let Some(i) = lines.next() {
            HashSet::from_iter(i?.bytes())
        } else {
            break;
        };

        let second_rucksack: HashSet<u8> = if let Some(i) = lines.next() {
            HashSet::from_iter(i?.bytes())
        } else {
            panic!("Number of rucksacks was not a multiple of three!");
        };

        let third_rucksack: HashSet<u8> = if let Some(i) = lines.next() {
            HashSet::from_iter(i?.bytes())
        } else {
            panic!("Number of rucksacks was not a multiple of three!");
        };

        let mut common_items = first_rucksack;
        common_items.retain(|item| second_rucksack.contains(item) && third_rucksack.contains(item));

        if common_items.len() != 1 {
            panic!("Not one common item found! {:?}", common_items);
        }

        for item in common_items {
            priority_sum += get_priority_of_item(item)?;
        }
    }

    Ok(priority_sum)
}

fn get_priority_of_item(item: u8) -> Result<usize> {
    if item >= LITTLE_A && item <= LITTLE_Z {
        Ok((item - LITTLE_A + 1) as usize)
    } else if item >= BIG_A && item <= BIG_Z {
        Ok((item - BIG_A + 27) as usize)
    } else {
        Err(PriorityError(item).into())
    }
}
