//! Not the most streamlined or efficient route but it makes adding
//! plots of the travelled route or similar easier.
use santas_little_helpers::data::{get_day_number, load_data};
use santas_little_helpers::error::AocError;

use std::ops::{Add, Sub};
use std::collections::HashSet;

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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
enum GridPointType {
    Space,
    Blocker,
    Exit,
}

impl TryFrom<char> for GridPointType {
    type Error = AocError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(GridPointType::Space),
            '#' => Ok(GridPointType::Blocker),
            'X' => Ok(GridPointType::Exit),
            '^' => Ok(GridPointType::Space),
            c => Err(AocError::ParsePointTypeError(c)),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct GuardPoint {
    x: i32,
    y: i32,
    direction: Direction,
}

impl GuardPoint {
    fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    fn default() -> Self {
        Self::new(0, 0, Direction::North)
    }
}

impl Add for GuardPoint {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            direction: self.direction,
        }
    }
}

impl Sub for GuardPoint {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            direction: self.direction,
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Guard {
    visited_points: Vec<GuardPoint>,
    grid: Grid,
    on_grid: bool,
}

impl Guard {
    fn new(grid: Grid, guard_point: GuardPoint) -> Self {
        Self {
            grid,
            visited_points: vec![guard_point],
            on_grid: true,
        }
    }

    fn get_next_point(&self) -> GuardPoint {
        let current_point = self.current_location();
        match current_point.direction {
            dir @ Direction::North => GuardPoint::new(current_point.x, current_point.y - 1, dir),
            dir @ Direction::South => GuardPoint::new(current_point.x, current_point.y + 1, dir),
            dir @ Direction::East => GuardPoint::new(current_point.x + 1, current_point.y, dir),
            dir @ Direction::West => GuardPoint::new(current_point.x - 1, current_point.y, dir),
        }
    }

    fn step(&mut self) {
        let next_point = self.get_next_point();
        match self.grid.check_point(&next_point) {
            GridPointType::Space => {
                self.visited_points.push(next_point);
            }
            GridPointType::Blocker => {
                let guard_point = GuardPoint::new(
                    self.current_location().x,
                    self.current_location().y,
                    self.rotate(),
                );
                self.visited_points.push(guard_point);
            }
            GridPointType::Exit => {
                self.on_grid = false;
            }
        }
    }

    fn rotate(&self) -> Direction {
        match self.current_location().direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn current_location(&self) -> GuardPoint {
        self.visited_points.last().expect("We initialise the guard with a location so there must be at least one visited point").clone()
    }

    fn get_num_unique_visited_points(self) -> u32 {
        self.visited_points
            .iter()
            .map(|g| (g.x, g.y))
            .collect::<HashSet<_>>()
            .len()
            .try_into()
            .expect("Should be able to cast to u32")
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct Grid {
    grid: Vec<Vec<char>>,
}

impl Grid {
    fn check_point(&self, point: &GuardPoint) -> GridPointType {
        match self.grid.get(point.y as usize) {
            Some(row) => match row.get(point.x as usize) {
                Some(c) => GridPointType::try_from(*c).expect("AoC input is well defined"),
                None => GridPointType::Exit,
            },
            None => GridPointType::Exit,
        }
    }
}

fn parse_input(input: &str) -> (GuardPoint, Grid) {
    let mut grid = Vec::new();
    let mut guard_coords = GuardPoint::default();
    for (y, row) in input.lines().enumerate() {
        let mut row_vec = Vec::new();
        for (x, column) in row.chars().enumerate() {
            if column == '^' {
                guard_coords.x = x as i32;
                guard_coords.y = y as i32;
            }
            row_vec.push(column);
        }
        grid.push(row_vec);
    }
    (
        GuardPoint {
            x: guard_coords.x,
            y: guard_coords.y,
            direction: Direction::North,
        },
        Grid { grid },
    )
}

#[tracing::instrument]
pub fn part01(data: &str) -> u32 {
    let (guard_point, grid) = parse_input(data);
    let mut guard = Guard::new(grid, guard_point);
    loop {
        guard.step();
        if !guard.on_grid {
            break;
        }
    }
    guard.get_num_unique_visited_points()
}

pub fn part02(_data: &str) -> u32 {
    42
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let answer = part01(data);
        assert_eq!(answer, 41);
    }

    #[test]
    fn test_part02() {}
}
