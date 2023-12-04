use aoc_2015_rust::{output, read_lines, Runner};
use std::iter::zip;
const INPUT: &str = "input/day05.txt";

pub struct Day05 {
    data: Vec<String>,
}

impl Day05 {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Runner for Day05 {
    fn name(&self) -> (usize, usize) {
        (2015, 5)
    }

    fn parse(&mut self, input: Option<&str>) {
        self.data = read_lines(input.unwrap_or(INPUT));
    }

    fn part1(&mut self) -> Vec<String> {
        let mut nice = 0;
        for d in &self.data {
            let vowels = d.chars().filter(|c| "aeiou".contains(*c)).count();
            if vowels < 3 {
                continue;
            }
            let mut has_double = false;
            for pair in zip(d.chars(), d.chars().skip(1)) {
                if pair.0 == pair.1 {
                    has_double = true;
                    break;
                }
            }
            if !has_double {
                continue;
            }
            if d.contains("ab") || d.contains("cd") || d.contains("pq") || d.contains("xy") {
                continue;
            }
            nice += 1;
        }
        output(nice)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut nice = 0;
        for d in &self.data {
            let mut has_pair = false;
            for i in 0..d.len() - 1 {
                let pair = &d[i..i + 2];
                if d[i + 2..].contains(pair) {
                    has_pair = true;
                    break;
                }
            }
            if !has_pair {
                continue;
            }
            let mut has_repeat = false;
            for i in 0..d.len() - 2 {
                if d.chars().nth(i).unwrap() == d.chars().nth(i + 2).unwrap() {
                    has_repeat = true;
                    break;
                }
            }
            if !has_repeat {
                continue;
            }
            nice += 1;
        }

        output(nice)
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day05::Day05;

    const TEST_INPUT: &str = "input/day05-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day05::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "2")
    }

    #[test]
    fn part1_works() {
        let mut day = Day05::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "236")
    }

    #[test]
    fn part2_works() {
        let mut day = Day05::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "51")
    }
}
