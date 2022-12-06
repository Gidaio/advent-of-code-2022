use std::fs;
use std::io::{self, BufRead};
use std::time;

use super::*;

impl UnloadSpace {
    fn do_move(&mut self, move_to_do: &Move) -> BoxedResult<()> {
        if move_to_do.from >= STACK_COUNT {
            return Err(MoveError::BadFromStack(move_to_do.from).into());
        }

        if move_to_do.to >= STACK_COUNT {
            return Err(MoveError::BadToStack(move_to_do.to).into());
        }

        if self.stacks[move_to_do.from].len() < move_to_do.quantity {
            return Err(MoveError::NotEnoughCrates(move_to_do.from).into());
        }

        for _ in 0..move_to_do.quantity {
            // Unwrap is fine here because I've already verified that it exists.
            let value = self.stacks[move_to_do.from].pop().unwrap();
            self.stacks[move_to_do.to].push(value);
        }

        Ok(())
    }
}

pub fn get_tops_of_stacks() -> TimedResult<String> {
    let start = time::Instant::now();

    let mut unload_space = UnloadSpace::new();
    let file = fs::File::open("inputs/day5.txt")?;
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    _ = lines.nth(9); // Skip the specification in the file.

    for line in lines {
        let line = line?;
        let move_to_do = Move::from_str(&line)?;
        unload_space.do_move(&move_to_do)?;
    }

    let mut result = String::with_capacity(STACK_COUNT);

    for stack in unload_space.stacks {
        if let Some(i) = stack.last() {
            result.push(*i);
        }
    }

    Ok((result, start.elapsed()))
}
