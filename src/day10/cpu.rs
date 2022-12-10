use std::error::Error;
use std::fmt;

pub trait Peripheral {
    fn new() -> Self;
    fn update(&mut self, register_x: isize, clock_cycle: usize);
}

pub struct CPU<T: Peripheral> {
    register_x: isize,
    clock_cycle: usize,
    pub peripheral: T,
}

impl<T: Peripheral> CPU<T> {
    pub fn new() -> Self {
        Self {
            register_x: 1,
            clock_cycle: 0,
            peripheral: T::new(),
        }
    }

    fn increment_clock(&mut self) {
        self.clock_cycle += 1;
        self.peripheral.update(self.register_x, self.clock_cycle);
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
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

pub enum Instruction {
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
