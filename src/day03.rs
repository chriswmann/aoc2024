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
    unimplemented!();
}

pub fn part02(data: &str) -> u32 {
    unimplemented!();
}

/// The data is two integers per line, separated by three spaces
fn parse_input(data: &str) -> Vec<&str> {
    data.lines().collect::<Vec<&str>>()
}

#[cfg(test)]
mod tests {
    use super::*;
}
