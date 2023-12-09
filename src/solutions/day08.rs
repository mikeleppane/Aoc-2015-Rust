use aoc_2015_rust::{output, read_lines, Runner};
const INPUT: &str = "input/day08.txt";

#[derive(Default)]
pub struct Day08 {
    literals: Vec<String>,
}

impl Day08 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day08 {
    fn name(&self) -> (usize, usize) {
        (2015, 8)
    }

    fn parse(&mut self, input: Option<&str>) {
        self.literals = read_lines(input.unwrap_or(INPUT));
    }

    fn part1(&mut self) -> Vec<String> {
        let mut total = 0;
        for line in &self.literals {
            let chars = line
                .chars()
                .skip(1)
                .take(line.len() - 2)
                .collect::<Vec<char>>();

            let mut i = 0;
            let mut in_memory = 0;
            while i < chars.len() {
                if chars[i] == '\\' {
                    if chars[i + 1] == 'x' {
                        i += 4;
                    } else {
                        i += 2;
                    }
                } else {
                    i += 1;
                }
                in_memory += 1;
            }
            total += line.len() - in_memory;
        }
        output(total)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut total = 0;
        for literal in &self.literals {
            let encoded = encode_literal(literal);
            let chars = encoded
                .chars()
                .skip(1)
                .take(encoded.len() - 2)
                .collect::<Vec<char>>();

            let mut i = 0;
            let mut in_memory = 0;
            while i < chars.len() {
                if chars[i] == '\\' {
                    if chars[i + 1] == 'x' {
                        i += 4;
                    } else {
                        i += 2;
                    }
                } else {
                    i += 1;
                }
                in_memory += 1;
            }
            total += encoded.len() - in_memory;
        }
        output(total)
    }
}

// ---------------------------------------------------

fn encode_literal(literal: &str) -> String {
    let mut encoded = String::from("\"");
    for c in literal.chars() {
        if c == '\\' || c == '"' {
            encoded.push('\\');
        }
        encoded.push(c);
    }
    encoded.push('"');
    encoded
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day08::Day08;

    const TEST_INPUT: &str = "input/day08-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day08::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "12")
    }

    #[test]
    fn part1_works() {
        let mut day = Day08::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "1342")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day08::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "19")
    }

    #[test]
    fn part2_works() {
        let mut day = Day08::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "2074")
    }
}
