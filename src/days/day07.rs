use anyhow::{Context, Ok, Result};

// Most credit must be attributed to https://gist.github.com/marektamaskovic/3421acf52c7e8da882df503fb5495125
// This was a great learning experience though

pub fn solve(input_file: &str) {
    let input = std::fs::read_to_string(input_file).unwrap();

    let equations: Vec<Equation> = input
        .lines()
        .map(|line| Equation::from(line).unwrap())
        .collect();

    let result: u64 = equations
        .iter()
        .map(|eq| Equation::validate(eq).unwrap())
        .sum();

    println!("Total calibration result: {result}");
}

struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

static OPERATORS: [fn(u64, u64) -> u64; 3] = [
    |a: u64, b: u64| -> u64 { a + b },
    |a: u64, b: u64| -> u64 { a * b },
    // For part one result:
    // Modify signature 3 -> 2
    // Comment out this line
    |a: u64, b: u64| -> u64 { format!("{}{}", a, b).parse().unwrap() },
];

impl Equation {
    fn new() -> Equation {
        Equation {
            result: 0,
            numbers: Vec::new(),
        }
    }

    fn from(line: &str) -> Result<Equation> {
        let mut equation = Equation::new();

        if !line.contains(":") {
            return Err(anyhow::anyhow!("Invalid equation input"));
        }

        let (result, numbers) = match line.split_once(": ") {
            Some((res, nums)) => (
                res.parse::<u64>().unwrap(),
                nums.split(" ").map(|x| x.parse::<u64>().unwrap()).collect(),
            ),
            None => {
                return Err(anyhow::anyhow!("Couldn't split valid input"))
                    .context("Splitting to result and numbers failed")
            }
        };

        equation.result = result;
        equation.numbers = numbers;

        Ok(equation)
    }

    fn validate(&self) -> Result<u64> {
        match Equation::eval(self.result, self.numbers[0], self.numbers[1..].to_vec()).unwrap() {
            true => Ok(self.result),
            false => Ok(0),
        }
    }

    fn eval(expected_result: u64, acc: u64, numbers: Vec<u64>) -> Result<bool> {
        if numbers.is_empty() {
            return Ok(expected_result == acc);
        }

        Ok(OPERATORS.iter().any(|op| {
            Equation::eval(expected_result, op(acc, numbers[0]), numbers[1..].to_vec()).unwrap()
        }))
    }
}
