use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

const LITTLE_A: u8 = 'a' as u8;
const LITTLE_Z: u8 = 'z' as u8;
const BIG_A: u8 = 'A' as u8;
const BIG_Z: u8 = 'Z' as u8;

pub fn get_priority_of_incorrect_items() -> io::Result<usize> {
    let file = fs::File::open("inputs/day3.txt")?;
    let reader = io::BufReader::new(file);

    let mut priority_sum: usize = 0;
    for line in reader.lines() {
        let line = line?;
        let compartment_size = line.len() / 2;
        let first_compartment = &line[0..compartment_size];
        
        let mut unique_items: HashSet<u8> = HashSet::new();
        for character in first_compartment.bytes() {
            unique_items.insert(character);
        }

        let second_compartment = &line[compartment_size..];
        for character in second_compartment.bytes() {
            if unique_items.contains(&character) {
                let item_priority = if character >= LITTLE_A && character <= LITTLE_Z {
                    (character - LITTLE_A + 1) as usize
                } else if character >= BIG_A && character <= BIG_Z {
                    (character - BIG_A + 27) as usize
                } else {
                    panic!("Unrecognized char {}!", character as char);
                };

                priority_sum += item_priority;
                break;
            }
        }
    }

    Ok(priority_sum)
}
