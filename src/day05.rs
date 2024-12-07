use crate::cli::Part;
use std::fmt;
use std::str::FromStr;
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

// The data is in two parts this time: updates rules, blank line, updates data.
fn parse_input_parts(data: &str) -> (&str, &str) {
    let mut iter = data.split("\n\n");
    let rules = iter.next().unwrap();
    let updates = iter.next().unwrap();
    assert_eq!(iter.next(), None);
    (rules, updates)
}

#[derive(Clone, Debug, PartialEq)]
enum Error {
    FromStrError(String),
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FromStrError(e) => writeln!(f, "Couldn't get rule from str, {}", e),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Rule {
    before: u32,
    after: u32,
}

impl std::str::FromStr for Rule {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('|');
        let before = iter
            .next()
            .expect("Should have a before part")
            .parse::<u32>()
            .expect("Puzzle input should have rules in form of u32|u32");
        let after = iter
            .next()
            .expect("Should have an after part")
            .parse::<u32>()
            .expect("Puzzle input should have rules in form of u32|u32");
        Ok(Self { before, after })
    }
}

fn parse_rules(rules: &str) -> Vec<Rule> {
    rules
        .trim()
        .lines()
        .map(|l| Rule::from_str(l).unwrap())
        .collect()
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    updates
        .lines()
        .map(|line| {
            line.chars()
                .filter(|c| *c != ',')
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

pub fn part01(data: &str) -> u32 {
    let (rules, updates) = parse_input_parts(data);
    let rules = parse_rules(rules);
    let updates = parse_updates(updates);
    println!("Rules len: {}, updates len: {}", rules.len(), updates.len());
    42
}

pub fn part02(data: &str) -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let answer = part01(data);
    }

    #[test]
    fn test_part02() {}
}
