use santas_little_helpers::data::{get_day_number, load_data};

fn main() {
    let package_name = env!("CARGO_PKG_NAME");
    let day_number = get_day_number(package_name);
    let data = load_data(day_number);
    run(&data);
}

fn run(_data: &str) {
    let part01_answer = part01(data);
    println!("Day 1, Part 1 answer is: {}", part01_answer);
    let part02_answer = part02(data);
    println!("Day 1, Part 2 answer is: {}", part02_answer);
}

pub fn part01(_data: &str) -> u32 {
    42
}

pub fn part02(_data: &str) -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {}

    #[test]
    fn test_part02() {}
}
