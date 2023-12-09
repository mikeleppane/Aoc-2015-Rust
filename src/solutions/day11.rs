use std::collections::HashSet;

use aoc_2015_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day11.txt";

#[derive(Debug, Default, Clone)]
pub struct Day11 {
    password: String,
}

impl Day11 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day11 {
    fn name(&self) -> (usize, usize) {
        (2015, 11)
    }

    fn parse(&mut self, input: Option<&str>) {
        let lines = read_lines(input.unwrap_or(INPUT));
        self.password = lines[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        output(find_next_password(&self.password))
    }

    fn part2(&mut self) -> Vec<String> {
        let password = find_next_password(&self.password);
        output(find_next_password(&password))
    }
}

// ---------------------------------------------------

fn find_next_password(password: &str) -> String {
    let mut password = password.to_string();
    loop {
        password = increment_password(&password);
        if is_valid_password(&password) {
            break;
        }
    }
    password
}

fn is_valid_password(password: &str) -> bool {
    let mut has_straight = false;
    let mut pairs = HashSet::new();
    let mut prev = ' ';
    let mut prev_prev = ' ';
    for c in password.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }
        if c == prev && c != prev_prev {
            pairs.insert(c);
        }
        if c as u8 == prev as u8 + 1 && prev as u8 == prev_prev as u8 + 1 {
            has_straight = true;
        }
        prev_prev = prev;
        prev = c;
    }
    has_straight && pairs.len() >= 2
}

fn increment_password(password: &str) -> String {
    let mut password = password.to_string();
    let mut i = password.len() - 1;
    loop {
        let c = password.chars().nth(i).unwrap();
        if c == 'z' {
            password.replace_range(i..=i, "a");
            i -= 1;
        } else {
            password.replace_range(i..=i, &((c as u8 + 1) as char).to_string());
            break;
        }
    }
    password
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day11::Day11;

    const TEST_INPUT: &str = "input/day11-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day11::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "abcdffaa")
    }

    #[test]
    fn part1_works() {
        let mut day = Day11::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "cqjxxyzz")
    }

    #[test]
    fn part2_works() {
        let mut day = Day11::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "cqkaabcc")
    }
}
