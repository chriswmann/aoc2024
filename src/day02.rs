use crate::cli::Part;
pub fn run(data: &str, part: Option<Part>) {
    if let Some(part) = part {
        match part {
            Part::One => part01(data),
            Part::Two => part02(data),
        }
        return;
    }
    part01(data);
    part02(data);
}

pub fn part01(data: &str) {
    let answer = "Unimplemented";
    println!("Day 1, Part 1 answer is: {}", answer);
}

pub fn part02(data: &str) {
    let answer = "Unimplemented";
    println!("Day 1, Part 2 answer is: {}", answer);
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
