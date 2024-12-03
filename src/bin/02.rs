advent_of_code::solution!(2);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    let reports = parse_input(input);
    let safe_reports = reports.iter().filter(|&report| is_safe(report)).count() as i32;

    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<i32> {
    let reports = parse_input(input);

    let mut safe_report_count = 0;
    reports.iter().for_each(|report| {
        if is_safe(report) {
            safe_report_count += 1;
            return
        }

        let is_safe_when_removing = report
            .iter()
            .combinations(report.len() - 1).any(|c| {
                let casted_combination: Vec<i32> = c.iter().map(|&&x| x).collect();
                let is_local_safe = is_safe(&casted_combination);

                is_local_safe
            });

        if is_safe_when_removing {
            safe_report_count += 1;
        }
    });


    Some(safe_report_count)
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
     input
         .lines()
         .map(parse_line)
         .collect::<Vec<_>>()
}

fn parse_line(line: &str) -> Vec<i32> {
    line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>()
}

fn is_safe(report: &[i32]) -> bool {
    let differences: Vec<i32> = report.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let is_in_range = differences.iter().all(|diff| (1..=3).contains(diff) || (-3..=-1).contains(diff));
    let is_all_increasing = differences.iter().all(|diff| diff.is_positive());
    let is_all_decreasing = differences.iter().all(|diff| diff.is_negative());

    is_in_range && (is_all_increasing || is_all_decreasing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
