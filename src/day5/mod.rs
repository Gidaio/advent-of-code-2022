pub mod part1;

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
