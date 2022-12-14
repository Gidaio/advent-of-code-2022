use super::*;

const FIRST_DIVIDER: &'static str = "[[2]]";
const SECOND_DIVIDER: &'static str = "[[6]]";

#[derive(Debug)]
struct MissingLine {}

impl fmt::Display for MissingLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Missing a line!.")
    }
}

impl Error for MissingLine {}

pub fn locate_divider_packets() -> BoxedResult<usize> {
    let first_divider_list = List::parse(&mut FIRST_DIVIDER.bytes().peekable())?;
    let second_divider_list = List::parse(&mut SECOND_DIVIDER.bytes().peekable())?;

    let file = File::open("inputs/day13.txt")?;
    let reader = BufReader::new(file);

    let mut lists = Vec::<List>::from([first_divider_list, second_divider_list]);

    for line in reader.lines() {
        let line = line?;
        if !line.is_empty() {
            let mut bytes = line.bytes().peekable();
            lists.push(List::parse(&mut bytes)?);
        }
    }

    lists.sort();
    let first_divider_list = List::parse(&mut FIRST_DIVIDER.bytes().peekable())?;
    let second_divider_list = List::parse(&mut SECOND_DIVIDER.bytes().peekable())?;

    let mut result: usize = 1;
    for i in 0..lists.len() {
        if lists[i] == first_divider_list || lists[i] == second_divider_list {
            result *= i + 1;
        }
    }

    Ok(result)
}
