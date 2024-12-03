use crate::cli::Part;
pub fn run(data: &str, part: Option<Part>) {
    if let Some(part) = part {
        let answer = match part {
            Part::One => part01(data),
            Part::Two => part02(data),
        };
        println!("Day 1, Part {} answer is: {}", part, answer);
        return;
    }
    let part01_answer = part01(data);

    println!("Day 1, Part 1 answer is: {}", part01_answer);
    let part02_answer = part02(data);
    println!("Day 1, Part 2 answer is: {}", part02_answer);
}

pub fn part01(data: &str) -> u32 {
    let (mut first, mut second) = parse_input(data);
    first.sort_unstable();
    second.sort_unstable();
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (*a as i32 - *b as i32).unsigned_abs())
        .sum()
}

pub fn part02(data: &str) -> u32 {
    let (first, second) = parse_input(data);
    first
        .iter()
        .map(|f| second.iter().filter(|s| f == *s).count() as u32 * f)
        .sum()
}

/// The data is two integers per line, separated by three spaces
fn parse_input(data: &str) -> (Vec<u32>, Vec<u32>) {
    let mut first = Vec::new();
    let mut second = Vec::new();
    for line in data.lines() {
        let mut split = line.split_whitespace();
        first.push(split.next().unwrap().parse::<u32>().unwrap());
        second.push(split.next().unwrap().parse::<u32>().unwrap());
    }
    (first, second)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";

    #[test]
    fn test_part01() {
        let answer = part01(DATA);
        assert_eq!(answer, 11);
    }

    #[test]
    fn test_part02() {
        let answer = part02(DATA);
        assert_eq!(answer, 31);
    }
}
