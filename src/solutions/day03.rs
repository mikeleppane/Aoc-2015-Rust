use std::collections::HashSet;

use aoc_2015_rust::read_to_chars;
use aoc_2015_rust::{output, Runner};

const INPUT: &str = "input/day03.txt";

pub struct Day03 {
    data: Vec<char>,
}

impl Day03 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Runner for Day03 {
    fn name(&self) -> (usize, usize) {
        (2015, 3)
    }

    fn parse(&mut self, input: Option<&str>) {
        self.data = read_to_chars(input.unwrap_or(INPUT));
    }

    fn part1(&mut self) -> Vec<String> {
        let mut houses = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        for d in &self.data {
            houses.insert((x, y));
            match d {
                '^' => y -= 1,
                'v' => y += 1,
                '>' => x += 1,
                '<' => x -= 1,
                _ => panic!("Invalid input: {}", d),
            }
        }
        houses.insert((x, y));
        output(houses.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut santa = HashSet::new();
        let mut robo = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        let mut rx = 0;
        let mut ry = 0;
        santa.insert((x, y));
        robo.insert((rx, ry));
        for (i, d) in self.data.iter().enumerate() {
            if i % 2 == 0 {
                match d {
                    '^' => y -= 1,
                    'v' => y += 1,
                    '>' => x += 1,
                    '<' => x -= 1,
                    _ => panic!("Invalid input: {}", d),
                }
                santa.insert((x, y));
            } else {
                match d {
                    '^' => ry -= 1,
                    'v' => ry += 1,
                    '>' => rx += 1,
                    '<' => rx -= 1,
                    _ => panic!("Invalid input: {}", d),
                }
                robo.insert((rx, ry));
            }
        }
        output(santa.union(&robo).count())
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day03::Day03;

    const TEST_INPUT: &str = "input/day03-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day03::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "2")
    }

    #[test]
    fn part1_works() {
        let mut day = Day03::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "2592")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day03::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "11")
    }

    #[test]
    fn part2_works() {
        let mut day = Day03::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "2360")
    }
}
