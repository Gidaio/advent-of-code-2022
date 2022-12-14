use std::fmt::Display;
use std::time::Instant;

use crate::BoxedResult;

pub fn time_function<T: Display>(func: fn() -> BoxedResult<T>) -> String {
    let start_time = Instant::now();

    match func() {
        Ok(result) => format!("{} (in {} ms)", result, start_time.elapsed().as_millis()),
        Err(error) => format!("Errored: {}", error),
    }
}
