use std::fs;
use std::io;
use std::io::BufRead;

// The basic idea of the algorithm is to just run through the file one line at
// a time, keeping track of the biggest value we've seen. At the end, return it!
pub fn find_elf_with_most_calories() -> usize {
    let file = fs::File::open("inputs/day1.txt").unwrap();
    let buf_reader = io::BufReader::new(file);

    let mut biggest: usize = 0;
    let mut sum: usize = 0;

    for line_result in buf_reader.lines() {
        let line = line_result.unwrap();

        if line.len() == 0 {
            if sum > biggest {
                biggest = sum;
            }

            sum = 0;
        } else {
            let value = line.parse::<usize>().unwrap();
            sum += value;
        }
    }

    if sum > biggest {
        sum
    } else {
        biggest
    }
}
