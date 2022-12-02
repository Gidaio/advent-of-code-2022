use std::fs;
use std::io;
use std::io::BufRead;

// This one is similar, but we use an array of the three biggest values instead
// of just the single biggest.
pub fn find_three_elves_with_most_calories() -> usize {
    let file = fs::File::open("inputs/day1.txt").unwrap();
    let buf_reader = io::BufReader::new(file);

    let mut biggest_three: [usize; 3] = [0, 0, 0];
    let mut sum: usize = 0;

    for line_result in buf_reader.lines() {
        let line = line_result.unwrap();

        if line.len() == 0 {
            // Since this for loop is an early exit loop, sorting makes sure
            // we're always replacing the lowest value.
            biggest_three.sort();
            for biggest in &mut biggest_three {
                if sum > *biggest {
                    *biggest = sum;
                    break;
                }
            }

            sum = 0;
        } else {
            let value = line.parse::<usize>().unwrap();
            sum += value;
        }
    }

    for biggest in &mut biggest_three {
        if sum > *biggest {
            *biggest = sum;
            break;
        }
    }

    biggest_three.iter().sum()
}
