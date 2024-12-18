use anyhow;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use nom_derive::*;

use santas_little_helpers::data::{get_day_number, load_data};

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

#[derive(Debug, Nom)]
struct Row {
    answer: u64,
    numbers: Vec<u64>,
}

fn parse_row(input: &str) -> IResult<&str, Row> {
    let (input, (answer, numbers)) = separated_pair(
        complete::u64,
        tag(": "),
        separated_list1(space1, complete::u64),
    )(input)?;
    Ok((input, Row { answer, numbers }))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(line_ending, parse_row)(input)
}

pub fn part01(data: &str) -> u32 {
    let (_, rows) = parse_input(data).expect("Input is consistently parsable");
    println!("{:?}", rows);
    42
}

pub fn part02(_data: &str) -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    use rstest_reuse::{self, *};

    const DATA: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[rstest]
    fn test_part01() {}

    #[rstest]
    fn test_part02() {}
}
