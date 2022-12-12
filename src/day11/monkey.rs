use super::*;

#[derive(Debug)]
pub struct Monkey {
    pub items: Vec<usize>,
    operation: MonkeyOperation,
    test: MonkeyTest,
}

impl Monkey {
    pub fn parse_from_lines(lines: &mut Lines<BufReader<File>>) -> BoxedResult<Self> {
        // Skip the "Monkey #:" line.
        lines.next();

        // Starting items: #, #...
        let starting_items_line = lines.next().ok_or(MonkeyParseError::MissingLine(
            MissingLineType::StartingItems,
        ))??;
        // Skip "  Starting items: ", then split the rest on commas.
        let item_strs = starting_items_line[18..].split(", ");
        let items = item_strs
            .map(|item| item.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        // Operation: new = ...
        let operation = MonkeyOperation::try_from(
            lines
                .next()
                .ok_or(MonkeyParseError::MissingLine(MissingLineType::Operation))??,
        )?;
        let test = MonkeyTest::parse_from_lines(lines)?;

        Ok(Self {
            items,
            operation,
            test,
        })
    }

    pub fn inspect_own_items(&mut self, divisor: usize) -> Vec<(usize, usize)> {
        let out_vec = self
            .items
            .iter()
            .map(|item| self.inspect_item(*item, divisor))
            .collect();
        self.items.clear();

        out_vec
    }

    fn inspect_item(&self, item: usize, divisor: usize) -> (usize, usize) {
        let mut sides: [usize; 2] = [0, 0];
        match self.operation.left {
            Value::Old => sides[0] = item,
            Value::Const(value) => sides[0] = value,
        }

        match self.operation.right {
            Value::Old => sides[1] = item,
            Value::Const(value) => sides[1] = value,
        }

        let result = match self.operation.operation {
            MathOperation::Add => sides[0] + sides[1],
            MathOperation::Multiply => sides[0] * sides[1],
        } / divisor;

        let target = if result % self.test.modulus == 0 {
            self.test.true_target
        } else {
            self.test.false_target
        };

        (target, result % 9699690)
    }
}

#[derive(Debug)]
struct MonkeyOperation {
    left: Value,
    right: Value,
    operation: MathOperation,
}

impl TryFrom<String> for MonkeyOperation {
    type Error = Box<dyn Error>;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // Skip "  Operation: new = ", then split the rest on spaces.
        let mut operation_strs = value[19..].split_ascii_whitespace();
        let left = Value::try_from(operation_strs.next().unwrap())?;
        let operation = MathOperation::try_from(operation_strs.next().unwrap())?;
        let right = Value::try_from(operation_strs.next().unwrap())?;

        Ok(Self {
            left,
            right,
            operation,
        })
    }
}

#[derive(Debug)]
enum Value {
    Old,
    Const(usize),
}

impl TryFrom<&str> for Value {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "old" {
            Ok(Self::Old)
        } else {
            Ok(Self::Const(value.parse::<usize>()?))
        }
    }
}

#[derive(Debug)]
enum MathOperation {
    Add,
    Multiply,
}

impl TryFrom<&str> for MathOperation {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Self::Add),
            "*" => Ok(Self::Multiply),
            _ => Err(MonkeyParseError::InvalidOperation(String::from(value)).into()),
        }
    }
}

#[derive(Debug)]
struct MonkeyTest {
    modulus: usize,
    true_target: usize,
    false_target: usize,
}

impl MonkeyTest {
    fn parse_from_lines(lines: &mut Lines<BufReader<File>>) -> BoxedResult<Self> {
        let modulus_line = lines
            .next()
            .ok_or(MonkeyParseError::MissingLine(MissingLineType::Test))??;
        let modulus = modulus_line[21..].parse::<usize>()?;

        let if_true_line = lines
            .next()
            .ok_or(MonkeyParseError::MissingLine(MissingLineType::TestIfTrue))??;
        let true_target = if_true_line[29..].parse::<usize>()?;

        let if_false_line = lines
            .next()
            .ok_or(MonkeyParseError::MissingLine(MissingLineType::TestIfFalse))??;
        let false_target = if_false_line[30..].parse::<usize>()?;

        Ok(Self {
            modulus,
            true_target,
            false_target,
        })
    }
}

#[derive(Debug)]
enum MonkeyParseError {
    MissingLine(MissingLineType),
    InvalidOperation(String),
}

#[derive(Debug)]
enum MissingLineType {
    StartingItems,
    Operation,
    Test,
    TestIfTrue,
    TestIfFalse,
}

impl fmt::Display for MonkeyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLine(line_type) => {
                write!(f, "Missing line '{}' for monkey definition.", line_type)
            }
            Self::InvalidOperation(op) => write!(f, "Invalid operation '{}'", op),
        }
    }
}

impl Error for MonkeyParseError {}

impl fmt::Display for MissingLineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StartingItems => write!(f, "Starting items"),
            Self::Operation => write!(f, "Operation"),
            Self::Test => write!(f, "Test"),
            Self::TestIfTrue => write!(f, "If true"),
            Self::TestIfFalse => write!(f, "If false"),
        }
    }
}
