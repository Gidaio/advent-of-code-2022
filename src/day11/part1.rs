use super::*;

pub fn calculate_monkey_business() -> TimedResult<usize> {
    let start_time = Instant::now();

    let file = File::open("inputs/day11_test.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut monkeys = Vec::<Monkey>::new();

    loop {
        let monkey = Monkey::parse_from_lines(&mut lines)?;
        monkeys.push(monkey);
        if let None = lines.next() {
            break;
        }
    }

    println!("Monkeys: {:?}", monkeys);

    Ok((0, start_time.elapsed()))
}
