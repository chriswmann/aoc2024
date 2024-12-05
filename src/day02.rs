use crate::cli::Part;
pub fn run(data: &str, part: Option<Part>) {
    if let Some(part) = part {
        let answer = match part {
            Part::One => part01(data),
            Part::Two => part02(data),
        };
        println!("Day 2, Part {} answer is: {:?}", part, answer);
        return;
    }
    let part01_answer = part01(data);

    println!("Day 2, Part 1 answer is: {:?}", part01_answer);
    let part02_answer = part02(data);
    println!("Day 2, Part 2 answer is: {:?}", part02_answer);
}

pub fn part01(data: &str) -> u32 {
    let lines = parse_input(data);
    let mut rows = Vec::new();
    lines
        .iter()
        .for_each(|line| rows.push(line_to_digits(line)));
    let mut answer = 0u32;
    for row in rows {
        if report_decreasing_slowly(&row) || report_increasing_slowly(&row) {
            answer += 1;
        }
    }
    answer
}

fn line_to_digits(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn report_decreasing_slowly(report: &[u32]) -> bool {
    let len = report.len();
    for level_idx in 0..len - 1 {
        let left = report[level_idx];
        let right = report[level_idx + 1];
        if !(1..4).contains(&(left.saturating_sub(right))) {
            return false;
        }
    }
    true
}

fn report_increasing_slowly(report: &[u32]) -> bool {
    let len = report.len();
    for level_idx in 0..len - 1 {
        let left = report[level_idx];
        let right = report[level_idx + 1];
        if !(1..4).contains(&(right.saturating_sub(left))) {
            return false;
        }
    }
    true
}

// Terribly inefficient brute force solution but it works.
pub fn part02(data: &str) -> u32 {
    let lines = parse_input(data);
    let mut reports = Vec::new();
    lines
        .iter()
        .for_each(|line| reports.push(line_to_digits(line)));
    let mut answer = 0u32;
    for report in reports {
        if report_decreasing_slowly(&report) || report_increasing_slowly(&report) {
            answer += 1;
            continue;
        }
        for level_idx in 0..report.len() {
            let mut new_report = report.clone();
            new_report.remove(level_idx);
            if report_decreasing_slowly(&new_report) || report_increasing_slowly(&new_report) {
                answer += 1;
                break;
            }
        }
    }
    answer
}

/// The data is two integers per line, separated by three spaces
fn parse_input(data: &str) -> Vec<&str> {
    data.lines().collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const DATA: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
    #[test]
    fn test_part01() {
        let answer = part01(DATA);
        assert_eq!(answer, 2);
    }

    #[test]
    fn test_part02_given_input() {
        let answer = part02(DATA);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_part02_edge_case_input() {
        let data = "48 46 47 49 51 54 56\n1 1 2 3 4 5\n1 2 3 4 5 5\n5 1 2 3 4 5\n1 4 3 2 1\n1 6 7 8 9\n1 2 3 4 3\n9 8 7 6 7\n7 10 8 10 11\n29 28 27 25 26 25 22 20";
        let ansewr = part02(data);
        assert_eq!(ansewr, 10);
    }
}
