use aoc_2015_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day10.txt";

#[derive(Debug, Default, Clone)]
pub struct Day10 {
    sequence: Vec<u8>,
}

impl Day10 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day10 {
    fn name(&self) -> (usize, usize) {
        (2015, 10)
    }

    fn parse(&mut self, input: Option<&str>) {
        let lines = read_lines(input.unwrap_or(INPUT));
        self.sequence = lines[0]
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut sequence = self.sequence.clone();
        for _ in 0..40 {
            sequence = look_and_say(&sequence);
        }
        output(sequence.len())
    }

    fn part2(&mut self) -> Vec<String> {
        let mut sequence = self.sequence.clone();
        for _ in 0..50 {
            sequence = look_and_say(&sequence);
        }
        output(sequence.len())
    }
}

// ---------------------------------------------------

fn look_and_say(sequence: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    let mut count = 0;
    let mut current = sequence[0];
    for &num in sequence {
        if num == current {
            count += 1;
        } else {
            result.push(count);
            result.push(current);
            count = 1;
            current = num;
        }
    }
    result.push(count);
    result.push(current);
    result
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day10::Day10;

    const TEST_INPUT: &str = "input/day10-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day10::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "237746")
    }

    #[test]
    fn part1_works() {
        let mut day = Day10::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "252594")
    }

    #[test]
    fn part2_works() {
        let mut day = Day10::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "3579328")
    }
}
