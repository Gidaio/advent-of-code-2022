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

pub fn calculate_sum_of_signal_strengths() -> TimedResult<isize> {
    let start_time = Instant::now();

    let mut cpu = CPU::<SignalStrength>::new();
    cpu.execute_file("inputs/day10.txt")?;

    Ok((cpu.peripheral.signal_strength, start_time.elapsed()))
}
