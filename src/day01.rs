pub fn part01(data: &str) {
    let (mut first, mut second) = parse_input(data);
    first.sort_unstable();
    second.sort_unstable();
    let answer: u32 = first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| (*a as i32 - *b as i32).unsigned_abs())
        .sum();
    println!("Day 1, Part 1 answer is: {}", answer);
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
