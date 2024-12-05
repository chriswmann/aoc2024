use crate::cli::Part;
use once_cell::sync::Lazy;
use regex::Regex;
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

struct Operation {
    multiplier: u32,
    multiplicand: u32,
}

/// Given a string with format 'mul(\d+,\d+)', parse the two digits and return an Instruction struct.
/// The left digit is the multiplier and the right digit is the multiplicand.
impl std::convert::From<&str> for Operation {
    fn from(s: &str) -> Self {
        let mut parts = s.split(',');
        let multiplier = parts
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();
        let multiplicand = parts
            .next()
            .unwrap()
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap();
        Self {
            multiplier,
            multiplicand,
        }
    }
}

/// Multiply the multiplier and multiplicand.
impl Operation {
    fn mul(&self) -> u32 {
        self.multiplier * self.multiplicand
    }
}

// Use regex to find the pattern 'mul(\d{1,3},\d{1,3})' in the haystack.
// Couldn't work out how to use nested capture groups to extract the digits directly.
/// Simple utility function to allow a regex haystack to be compiled once and reused.
fn regex_find_iter(haystack: &str) -> regex::Matches {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap());
    RE.find_iter(haystack)
}

// The basic strategy is to extract the 'mul(x,y)' instructions via regex, then parse the strings
// to extract the digits. The Instruction struct stores the multiplier and multiplicand.
// Instruction provides the From trait to convert from &str to Instruction as well as the mul
// method to perform the multiplication operation.
pub fn part01(data: &str) -> u32 {
    let regex_iter = regex_find_iter(data);
    let mut instructions = Vec::new();
    for m in regex_iter {
        instructions.push(Operation::from(m.as_str()).mul());
    }
    instructions.iter().sum::<u32>()
}

pub fn part02(data: &str) -> u32 {
    let separator = Regex::new(r"do\(\)|don't\(\)").unwrap();
    let mut enabled = true;
    let mut instructions = Vec::new();
    for split in data.split_inclusive(&separator) {
        if enabled {
            let regex_iter = regex_find_iter(split);
            for m in regex_iter {
                instructions.push(Operation::from(m.as_str()).mul());
            }
        }
        if split.ends_with("do()") {
            enabled = true;
        } else if split.ends_with("don't()") {
            enabled = false;
        }
    }
    instructions.iter().sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let answer = part01(data);
        assert_eq!(answer, 161);
    }

    #[test]
    fn test_part02() {
        let data = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let answer = part02(data);
        assert_eq!(answer, 48);
    }
}
