use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

struct CPU {
    register_x: isize,
    clock_cycle: usize,
    signal_strength: isize,
}

impl CPU {
    fn new() -> Self {
        Self {
            register_x: 1,
            clock_cycle: 0,
            signal_strength: 0,
        }
    }

    fn increment_clock(&mut self) {
        self.clock_cycle += 1;
        if (self.clock_cycle + 20) % 40 == 0 {
            self.signal_strength += self.register_x * self.clock_cycle as isize;
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::AddX(amount) => {
                self.increment_clock();
                self.increment_clock();
                self.register_x += amount;
            }
            Instruction::NoOp => self.increment_clock(),
        }
    }
}

#[derive(Debug)]
enum ParseInstructionError {
    MissingCommand,
    InvalidCommand(String),
    MissingArgument(String, usize),
}

impl fmt::Display for ParseInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingCommand => write!(f, "Missing command!"),
            Self::InvalidCommand(command) => write!(f, "Invalid command: {}", command),
            Self::MissingArgument(command, index) => {
                write!(f, "Missing argument {} for '{}'", index, command)
            }
        }
    }
}

impl Error for ParseInstructionError {}

enum Instruction {
    AddX(isize),
    NoOp,
}

impl TryFrom<String> for Instruction {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut tokens = value.split_ascii_whitespace();
        let command = tokens.next().ok_or(ParseInstructionError::MissingCommand)?;

        match command {
            "addx" => {
                let amount = tokens
                    .next()
                    .ok_or(ParseInstructionError::MissingArgument(
                        String::from(command),
                        0,
                    ))?
                    .parse::<isize>()?;

                Ok(Instruction::AddX(amount))
            }
            "noop" => Ok(Instruction::NoOp),
            _ => Err(ParseInstructionError::InvalidCommand(String::from(command)).into()),
        }
    }
}

pub fn calculate_sum_of_signal_strengths() -> crate::TimedResult<isize> {
    let start_time = Instant::now();

    let file = File::open("inputs/day10.txt")?;
    let reader = BufReader::new(file);
    let mut cpu = CPU::new();

    for line in reader.lines() {
        let instruction = Instruction::try_from(line?)?;
        cpu.execute_instruction(instruction);
    }

    Ok((cpu.signal_strength, start_time.elapsed()))
}
