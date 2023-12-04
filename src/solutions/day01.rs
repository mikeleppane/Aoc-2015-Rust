use aoc_2015_rust::read_to_chars;
use aoc_2015_rust::{output, Runner};

const INPUT: &str = "input/day01.txt";

pub struct Day01 {
    doors: Vec<char>,
}

impl Day01 {
    pub fn new() -> Self {
        Self { doors: Vec::new() }
    }
}

impl Runner for Day01 {
    fn name(&self) -> (usize, usize) {
        (2015, 1)
    }

    fn parse(&mut self, input: Option<&str>) {
        self.doors = read_to_chars(input.unwrap_or(INPUT));
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            self.doors
                .iter()
                .fold(0, |acc, c| if *c == '(' { acc + 1 } else { acc - 1 }),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        let mut pos = 0;
        self.doors.iter().fold(0, |acc, c| {
            if acc == -1 {
                return -1;
            }
            pos += 1;
            if *c == '(' {
                acc + 1
            } else {
                acc - 1
            }
        });
        output(pos)
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day01::Day01;

    #[test]
    fn part1_works() {
        let mut day01 = Day01::new();
        day01.parse(None);
        let output = day01.part1();
        assert_eq!(output[0], "74")
    }

    #[test]
    fn part2_works() {
        let mut day01 = Day01::new();
        day01.parse(None);
        let output = day01.part2();
        assert_eq!(output[0], "1795")
    }
}
