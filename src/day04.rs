use crate::cli::Part;
use std::fmt;
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
    v: char,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}: {}", self.x, self.y, self.v)
    }
}

#[derive(Debug)]
struct Grid<'x> {
    points: Vec<Vec<Point>>,
    xmas: &'x str,
    matches: u32,
}

impl Grid<'_> {
    fn new(points: Vec<Vec<Point>>) -> Self {
        Self {
            points,
            xmas: "XMAS",
            matches: 0,
        }
    }

    fn get_point_by_coords(&self, x: i32, y: i32) -> Option<&Point> {
        if x < 0 || y < 0 {
            return None;
        }
        if let Some(row) = self.points.get(y as usize) {
            return row.get(x as usize);
        }
        None
    }

    fn search_eastwards(&mut self, point: &Point) {
        let x = point.x;
        let y = point.y;
        for (i, letter) in self.xmas.chars().enumerate() {
            if let Some(point) = self.get_point_by_coords(x + i as i32, y) {
                if point.v != letter {
                    return;
                }
            } else {
                return;
            }
        }
        self.matches += 1;
    }

    fn search_southeastwards(&mut self, point: &Point) {
        let x = point.x;
        let y = point.y;
        for (i, letter) in self.xmas.chars().enumerate() {
            if let Some(point) = self.get_point_by_coords(x + i as i32, y + i as i32) {
                if point.v != letter {
                    return;
                }
            } else {
                return;
            }
        }
        self.matches += 1;
    }

    fn search_southwards(&mut self, point: &Point) {
        let x = point.x;
        let y = point.y;
        for (i, letter) in self.xmas.chars().enumerate() {
            if let Some(point) = self.get_point_by_coords(x, y + i as i32) {
                if point.v != letter {
                    return;
                }
            } else {
                return;
            }
        }
        self.matches += 1;
    }

    fn search_southwestwards(&mut self, point: &Point) {
        let x = point.x;
        let y = point.y;
        for (i, letter) in self.xmas.chars().enumerate() {
            if let Some(point) = self.get_point_by_coords(x - i as i32, y + i as i32) {
                if point.v != letter {
                    return;
                }
            } else {
                return;
            }
        }
        self.matches += 1;
    }

    fn search_westwards(&mut self, point: &Point) {
        let x = point.x;
        let y = point.y;
        for (i, letter) in self.xmas.chars().enumerate() {
            if let Some(point) = self.get_point_by_coords(x - i as i32, y) {
                if point.v != letter {
                    return;
                }
            } else {
                return;
            }
        }
        self.matches += 1;
    }

    fn search_northwestwards(&mut self, point: &Point) {
        let x = point.x;
        let y = point.y;
        for (i, letter) in self.xmas.chars().enumerate() {
            if let Some(point) = self.get_point_by_coords(x - i as i32, y - i as i32) {
                if point.v != letter {
                    return;
                }
            } else {
                return;
            }
        }
        self.matches += 1;
    }

    fn search_northwards(&mut self, point: &Point) {
        let x = point.x;
        let y = point.y;
        for (i, letter) in self.xmas.chars().enumerate() {
            if let Some(point) = self.get_point_by_coords(x, y - i as i32) {
                if point.v != letter {
                    return;
                }
            } else {
                return;
            }
        }
        self.matches += 1;
    }

    fn search_northeastwards(&mut self, point: &Point) {
        let x = point.x;
        let y = point.y;
        for (i, letter) in self.xmas.chars().enumerate() {
            if let Some(point) = self.get_point_by_coords(x + i as i32, y - i as i32) {
                if point.v != letter {
                    return;
                }
            } else {
                return;
            }
        }
        self.matches += 1;
    }

    fn radial_search_from_point(&mut self, point: &Point) {
        self.search_eastwards(point);
        self.search_southeastwards(point);
        self.search_southwards(point);
        self.search_southwestwards(point);
        self.search_westwards(point);
        self.search_northwestwards(point);
        self.search_northwards(point);
        self.search_northeastwards(point);
    }

    fn x_shape_search_from_point(&mut self, point: &Point) {
        if let (Some(ne), Some(se), Some(sw), Some(nw)) = (
            self.get_point_by_coords(point.x + 1, point.y - 1),
            self.get_point_by_coords(point.x + 1, point.y + 1),
            self.get_point_by_coords(point.x - 1, point.y + 1),
            self.get_point_by_coords(point.x - 1, point.y - 1),
        ) {
            match (nw.v, se.v, sw.v, ne.v) {
                // M . S
                // . A .
                // M . S
                (nw, se, sw, ne) if nw == 'M' && se == 'S' && sw == 'M' && ne == 'S' => {
                    self.matches += 1;
                }
                // M . M
                // . A .
                // S . S
                (nw, se, sw, ne) if nw == 'M' && se == 'S' && sw == 'S' && ne == 'M' => {
                    self.matches += 1;
                }
                // S . S
                // . A .
                // M . M
                (nw, se, sw, ne) if nw == 'S' && se == 'M' && sw == 'M' && ne == 'S' => {
                    self.matches += 1;
                }
                // S . M
                // . A .
                // S . M
                (nw, se, sw, ne) if nw == 'S' && se == 'M' && sw == 'S' && ne == 'M' => {
                    self.matches += 1;
                }
                (_, _, _, _) => (),
            }
        }
    }

    fn grid_search_part01(&mut self) {
        let start = self.xmas.chars().next().unwrap();
        for row in self.points.clone().iter() {
            for point in row.iter() {
                if point.v == start {
                    self.radial_search_from_point(point);
                }
            }
        }
    }

    fn grid_search_part02(&mut self) {
        let start = 'A';
        for row in self.points.clone().iter() {
            for point in row.iter() {
                if point.v == start {
                    self.x_shape_search_from_point(point);
                }
            }
        }
    }
}

impl fmt::Display for Grid<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.points {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_input(data: &str) -> Grid {
    let mut points = Vec::with_capacity(data.len());
    for (y, line) in data.lines().enumerate() {
        let mut row = Vec::with_capacity(line.len());
        for (x, c) in line.chars().enumerate() {
            row.push(Point {
                x: x as i32,
                y: y as i32,
                v: c,
            });
        }
        points.push(row);
    }
    Grid::new(points)
}

pub fn part01(data: &str) -> u32 {
    let mut grid = parse_input(data);
    grid.grid_search_part01();
    grid.matches
}

pub fn part02(data: &str) -> u32 {
    let mut grid = parse_input(data);
    grid.grid_search_part02();
    grid.matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part01() {
        let data = "..X...\n.SAMX.\n.A..A.\nXMAS.S\n.X....";
        let answer = part01(data);
        assert_eq!(answer, 4);
    }

    #[test]
    fn test_part02() {
        let data = ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n..........";
        let answer = part02(data);
        assert_eq!(answer, 9);
    }
}
