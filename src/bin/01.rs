use anyhow::{Context, Result};
use thiserror::Error;

advent_of_code::solution!(1);

#[derive(Error, Debug)]
enum MyError {
    #[error("this is my error message")]
    GenericError,
}

pub fn part_one(input: &str) -> Option<i32> {
    let columns = parse_input(input).expect("Parsing Input failed");
    let mut col1 = columns.iter().map(|line| line.0).collect::<Vec<_>>();
    let mut col2 = columns.iter().map(|line| line.1).collect::<Vec<_>>();
    col1.sort();
    col2.sort();

    Some(col1.into_iter().zip(col2).map(|(i, j)| (i - j).abs()).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let columns = parse_input(input).expect("Parsing Input failed");
    let col1 = columns.iter().map(|line| line.0).collect::<Vec<_>>();
    let col2 = columns.iter().map(|line| line.1).collect::<Vec<_>>();

    let mut sum = 0;
    for value in col1 {
        let amount = col2.iter().filter(|&&num| num == value).count() as i32;
        sum += value * amount;
    }

    Some(sum)
}

fn parse_input(input: &str) -> Result<Vec<(i32, i32)>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_line)
        .collect::<Result<Vec<(i32, i32)>>>()
        .context("Error while parsing input")
}

fn parse_line(line: &str) -> Result<(i32, i32)> {
    let mut parsed_line = line
        .split_whitespace()
        .collect::<Vec<_>>()
        .into_iter();

    match (parsed_line.next(), parsed_line.next()) {
        (Some(i), Some(j)) => Ok((i.parse::<i32>()?, j.parse::<i32>()?)),
        _ => Err(MyError::GenericError).context("no numbers")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.expect("nope"), 11);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
