use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

use super::*;

struct SignalStrength {
    signal_strength: isize,
}

impl Peripheral for SignalStrength {
    fn new() -> Self {
        Self { signal_strength: 0 }
    }

    fn update(&mut self, register_x: isize, clock_cycle: usize) {
        if (clock_cycle + 20) % 40 == 0 {
            self.signal_strength += register_x * clock_cycle as isize;
        }
    }
}

pub fn calculate_sum_of_signal_strengths() -> crate::TimedResult<isize> {
    let start_time = Instant::now();

    let file = File::open("inputs/day10.txt")?;
    let reader = BufReader::new(file);
    let mut cpu = CPU::<SignalStrength>::new();

    for line in reader.lines() {
        let instruction = Instruction::try_from(line?)?;
        cpu.execute_instruction(instruction);
    }

    Ok((cpu.peripheral.signal_strength, start_time.elapsed()))
}
