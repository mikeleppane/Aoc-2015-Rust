use std::collections::{HashMap, HashSet};

use aoc_2015_rust::{output, read_lines, Point, Runner};
const INPUT: &str = "input/day06.txt";

pub struct Day06 {
    data: Vec<Command>,
}

impl Day06 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Runner for Day06 {
    fn name(&self) -> (usize, usize) {
        (2015, 5)
    }

    fn parse(&mut self, input: Option<&str>) {
        for line in read_lines(input.unwrap_or(INPUT)) {
            if line.starts_with("turn off") {
                let (_, rest) = line.split_once("turn off ").unwrap();
                let (start, end) = rest.split_once(" through ").unwrap();
                let start: Point<u32> = start.trim().parse().unwrap();
                let end: Point<u32> = end.parse().unwrap();
                self.data.push(Command::Off(start, end));
            }
            if line.starts_with("toggle") {
                let (_, rest) = line.split_once("toggle ").unwrap();
                let (start, end) = rest.split_once(" through ").unwrap();
                let start: Point<u32> = start.trim().parse().unwrap();
                let end: Point<u32> = end.trim().parse().unwrap();
                self.data.push(Command::Toggle(start, end));
            }
            if line.starts_with("turn on ") {
                let (_, rest) = line.split_once("turn on ").unwrap();
                let (start, end) = rest.split_once(" through ").unwrap();
                let start: Point<u32> = start.trim().parse().unwrap();
                let end: Point<u32> = end.trim().parse().unwrap();
                self.data.push(Command::On(start, end));
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut lights_on: HashSet<Point<u32>> = HashSet::new();
        for command in &self.data {
            match command {
                Command::On(start, end) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            lights_on.insert(Point::new(x, y));
                        }
                    }
                }
                Command::Off(start, end) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            if lights_on.contains(&Point::new(x, y)) {
                                lights_on.remove(&Point::new(x, y));
                            }
                        }
                    }
                }
                Command::Toggle(start, end) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            if lights_on.contains(&Point::new(x, y)) {
                                lights_on.remove(&Point::new(x, y));
                            } else {
                                lights_on.insert(Point::new(x, y));
                            }
                        }
                    }
                }
            }
        }
        output(lights_on.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut brightness: HashMap<Point<u32>, u32> = HashMap::new();
        for command in &self.data {
            match command {
                Command::On(start, end) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            brightness
                                .entry(Point::new(x, y))
                                .and_modify(|e| *e += 1)
                                .or_insert(1);
                        }
                    }
                }
                Command::Off(start, end) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            brightness
                                .entry(Point::new(x, y))
                                .and_modify(|e| *e = e.saturating_sub(1))
                                .or_insert(0);
                        }
                    }
                }
                Command::Toggle(start, end) => {
                    for x in start.x..=end.x {
                        for y in start.y..=end.y {
                            brightness
                                .entry(Point::new(x, y))
                                .and_modify(|e| *e += 2)
                                .or_insert(2);
                        }
                    }
                }
            }
        }
        let brightness: u32 = brightness.values().sum();
        output(brightness)
    }
}

// ---------------------------------------------------

#[derive(Debug)]
enum Command {
    On(Point<u32>, Point<u32>),
    Off(Point<u32>, Point<u32>),
    Toggle(Point<u32>, Point<u32>),
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day06::Day06;

    const TEST_INPUT: &str = "input/day06-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day06::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "1000000")
    }

    #[test]
    fn part1_works() {
        let mut day = Day06::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "543903")
    }

    #[test]
    fn part2_works() {
        let mut day = Day06::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "14687245")
    }
}
