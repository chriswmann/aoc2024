use santas_little_helpers::{
    data::{get_day_number, load_data},
    error::AocError,
};
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

fn main() {
    let package_name = env!("CARGO_PKG_NAME");
    let day_number = get_day_number(package_name);
    let data = load_data(day_number);
    run(&data);
}

fn run(data: &str) {
    let part01_answer = part01(data);
    println!("Day 1, Part 1 answer is: {}", part01_answer);
    let part02_answer = part02(data);
    println!("Day 1, Part 2 answer is: {}", part02_answer);
}

#[derive(Clone, Debug, PartialEq)]
struct Rule {
    before: u32,
    after: u32,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}->{}", self.before, self.after)
    }
}

impl std::str::FromStr for Rule {
    type Err = AocError;
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

#[derive(Debug)]
struct PageOrderingRules {
    rules: Vec<Rule>,
}

impl fmt::Display for PageOrderingRules {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let head = self.rules.iter().take(6).collect::<Vec<_>>();
        writeln!(f, "{:#?}", head)
    }
}

type Update = Vec<u32>;

#[derive(Debug)]
struct Data {
    rules: PageOrderingRules,
    updates: Vec<Update>,
}

fn parse_rules_input(input: &str) -> PageOrderingRules {
    let rules = input
        .trim()
        .lines()
        .map(|l| Rule::from_str(l).unwrap())
        .collect();
    PageOrderingRules { rules }
}

fn parse_updates_input(input: &str) -> Vec<Update> {
    input
        .lines()
        .map(|l| l.split(',').map(|n| n.parse::<u32>().unwrap()).collect())
        .collect()
}

// The data is in two parts this time: updates rules, blank line, updates data.
fn parse_input(input: &str) -> Data {
    let mut parts = input.split("\n\n");
    let rules_input = parts.next().expect("There should always be some rules");
    let rules = parse_rules_input(rules_input);
    let updates_input = parts.next().expect("There should always be some updates");
    let updates = parse_updates_input(updates_input);
    Data { rules, updates }
}

fn create_page_update_map(update: &Update) -> HashMap<&u32, usize> {
    let mut page_to_index_map = HashMap::new();

    for (idx, page) in update.iter().enumerate() {
        page_to_index_map.insert(page, idx);
    }

    page_to_index_map
}

fn update_is_valid(rules: &PageOrderingRules, page_to_index_map: &HashMap<&u32, usize>) -> bool {
    for rule in &rules.rules {
        if !page_to_index_map.contains_key(&rule.before)
            || !page_to_index_map.contains_key(&rule.after)
        {
            continue;
        }
        let before_idx = page_to_index_map.get(&rule.before).unwrap();
        let after_idx = page_to_index_map.get(&rule.after).unwrap();

        if before_idx > after_idx {
            return false;
        }
    }

    true
}

pub fn part01(data: &str) -> u32 {
    let data = parse_input(data);
    data.updates
        .iter()
        .filter(|update| update_is_valid(&data.rules, &create_page_update_map(update)))
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn part02(_data: &str) -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47";
        let answer = part01(data);
        assert_eq!(answer, 143);
    }

    #[test]
    fn test_part02() {
        todo!();
    }
}
