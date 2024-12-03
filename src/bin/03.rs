advent_of_code::solution!(3);

use thiserror::Error;
use regex::{Match, Regex};

#[derive(Error, Debug)]
enum MyError {
    #[error("Unknown error")]
    GenericError,
    #[error("Error parsing")]
    ParsingError
}

#[derive(Error, Debug)]
enum Operation {
    Mul(i32, i32),
    Do,
    Dont
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Mul(a, b) => write!(f, "mul({}, {})", a, b),
            Operation::Do => write!(f, "do()"),
            Operation::Dont => write!(f, "don't()"),
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let expression = Regex::new(r"mul\(([1-9][0-9][0-9]|[1-9][0-9]|[0-9]),([1-9][0-9][0-9]|[1-9][0-9]|[0-9])\)").unwrap();
    let mut valid_multiplies: Vec<(i32, i32)> = Vec::new();

    for (_, arg1, arg2) in expression.captures_iter(input).map(|c| (c.get(0), c.get(1), c.get(2))) {
        let result = match (arg1, arg2) {
            (Some(num1), Some(num2)) => {
                let parsed_num1 = num1.as_str().parse::<i32>();
                let parsed_num2 = num2.as_str().parse::<i32>();
                match (parsed_num1, parsed_num2) {
                    (Ok(n1), Ok(n2)) => Ok((n1, n2)),
                    (Err(_e), _) | (_, Err(_e)) => Err(MyError::ParsingError),
                }
            }
            _ => Err(MyError::GenericError),
        };

        match result {
            Ok(pair) => valid_multiplies.push(pair),
            Err(e) => eprintln!("Error parsing input: {:?}", e),
        };
    }

    let result = valid_multiplies.iter().fold(0, |acc, pair| acc + pair.0 * pair.1);
    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let expression = Regex::new(r"(mul\(([1-9][0-9][0-9]|[1-9][0-9]|[0-9]),([1-9][0-9][0-9]|[1-9][0-9]|[0-9])\))|(do\(\))|(don't\(\))").unwrap();
    let mut valid_operations: Vec<Operation> = Vec::new();

    for (group, arg1, arg2) in expression.captures_iter(input).map(|c| (c.get(0), c.get(2), c.get(3))) {
        // println!("{:?} {:?} {:?}", group, arg1, arg2);

        let temporary_operations: Result<Operation, MyError> = match (group, arg1, arg2) {
            (Some(group), _, _) => {
                match group.as_str() {
                    "do()" => Ok(Operation::Do),
                    "don't()" => Ok(Operation::Dont),
                    _ => match (group, arg1, arg2) {
                        (_, Some(arg1), Some(arg2)) => {
                            let parsed_arg1 = arg1.as_str().parse::<i32>();
                            let parsed_arg2 = arg2.as_str().parse::<i32>();
                            match (parsed_arg1, parsed_arg2) {
                                (Ok(n1), Ok(n2)) => Ok(Operation::Mul(n1, n2)),
                                (Err(_e), _) | (_, Err(_e)) => Err(MyError::ParsingError),
                            }
                        },
                        _ => Err(MyError::ParsingError),
                    }
                }
            },
            _ => Err(MyError::ParsingError),
        };

        match temporary_operations {
            Ok(op) => valid_operations.push(op),
            Err(e) => eprintln!("Error parsing input: {:?}", e),
        };
    }

    let mut valid = true;
    let mut result = 0;
    for operation in valid_operations {
        match operation {
            Operation::Do => valid = true,
            Operation::Dont => valid = false,
            Operation::Mul(a, b) => {
                if valid {
                    result += a * b;
                }
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
