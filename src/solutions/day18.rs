use std::collections::HashSet;

use aoc_2015_rust::{output, read_lines, Point, Runner};

const INPUT: &str = "input/day18.txt";

#[derive(Debug, Default, Clone)]
pub struct Day18 {
    grid: HashSet<Point<i32>>,
    rows: u32,
    cols: u32,
    steps: u32,
}

impl Day18 {
    pub fn new() -> Self {
        Self::default()
    }

    fn step(&mut self) {
        let mut new_grid = HashSet::new();
        for row in 0..self.rows as i32 {
            for col in 0..self.cols as i32 {
                let point = Point::new(col, row);
                let neighbors = point.neighbors();
                let neighbors = neighbors
                    .iter()
                    .filter(|p| self.grid.contains(p))
                    .collect::<Vec<_>>();
                if neighbors.len() == 3 || (neighbors.len() == 2 && self.grid.contains(&point)) {
                    new_grid.insert(point);
                }
            }
        }
        self.grid = new_grid;
    }

    fn step_with_corner_lights_on(&mut self) {
        let mut new_grid = HashSet::new();
        let corners = [
            Point::new(0, 0),
            Point::new(0, self.rows as i32 - 1),
            Point::new(self.cols as i32 - 1, 0),
            Point::new(self.cols as i32 - 1, self.rows as i32 - 1),
        ];
        self.grid.extend(corners.iter());
        new_grid.extend(corners.iter());
        for row in 0..self.rows as i32 {
            for col in 0..self.cols as i32 {
                let point = Point::new(col, row);
                let neighbors = point.neighbors();
                let neighbors = neighbors
                    .iter()
                    .filter(|p| self.grid.contains(p))
                    .collect::<Vec<_>>();
                if neighbors.len() == 3 || (neighbors.len() == 2 && self.grid.contains(&point)) {
                    new_grid.insert(point);
                }
            }
        }
        self.grid = new_grid;
    }
}

impl Runner for Day18 {
    fn name(&self) -> (usize, usize) {
        (2018, 18)
    }

    fn parse(&mut self, input: Option<&str>) {
        let lines = read_lines(input.unwrap_or(INPUT));
        self.cols = lines.len() as u32;
        self.rows = lines[0].len() as u32;
        for (col, line) in lines.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    self.grid.insert(Point::new(i as i32, col as i32));
                }
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        for _ in 0..self.steps {
            self.step();
        }
        output(self.grid.len())
    }

    fn part2(&mut self) -> Vec<String> {
        for _ in 0..self.steps {
            self.step_with_corner_lights_on()
        }
        output(self.grid.len())
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day18::Day18;

    const TEST_INPUT: &str = "input/day18-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day18::new();
        day.steps = 5;
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "4")
    }

    #[test]
    fn part1_works() {
        let mut day = Day18::new();
        day.steps = 100;
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "768")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day18::new();
        day.steps = 5;
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "17")
    }

    #[test]
    fn part2_works() {
        let mut day = Day18::new();
        day.steps = 100;
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "781")
    }
}
