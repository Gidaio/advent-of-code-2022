use crate::TimedResult;

use super::*;

struct CRT {
    current_pixel: usize,
}

impl Peripheral for CRT {
    fn new() -> Self {
        Self { current_pixel: 0 }
    }

    fn update(&mut self, register_x: isize, _: usize) {
        if self.current_pixel as isize >= register_x - 1
            && self.current_pixel as isize <= register_x + 1
        {
            print!("#");
        } else {
            print!(".");
        }

        self.current_pixel += 1;

        if self.current_pixel % 40 == 0 {
            print!("\n");
            self.current_pixel = 0;
        }
    }
}

pub fn print_crt() -> TimedResult<()> {
    let start_time = Instant::now();

    let mut cpu = CPU::<CRT>::new();
    cpu.execute_file("inputs/day10.txt")?;

    Ok(((), start_time.elapsed()))
}
