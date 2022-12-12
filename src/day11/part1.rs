use super::*;

pub fn calculate_monkey_business() -> TimedResult<usize> {
    let start_time = Instant::now();

    let file = File::open("inputs/day11.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut monkey_infos = Vec::<(Monkey, usize)>::new();

    loop {
        let monkey = Monkey::parse_from_lines(&mut lines)?;
        monkey_infos.push((monkey, 0));
        if let None = lines.next() {
            break;
        }
    }

    for _ in 0..20 {
        for monkey_index in 0..monkey_infos.len() {
            let thrown_items = {
                let monkey_info = &mut monkey_infos[monkey_index];
                let inspected = monkey_info.0.inspect_own_items();
                monkey_info.1 += inspected.len();
                inspected
            };

            for item in thrown_items.into_iter() {
                monkey_infos[item.0].0.items.push(item.1);
            }
        }
    }

    monkey_infos.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    let top_two_infos = &monkey_infos[monkey_infos.len() - 2..];

    Ok((
        top_two_infos[0].1 * top_two_infos[1].1,
        start_time.elapsed(),
    ))
}
