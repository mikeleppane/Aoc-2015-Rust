use aoc_2015_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day04.txt";

pub struct Day04 {
    prefix: String,
}

impl Day04 {
    pub fn new() -> Self {
        Self {
            prefix: String::new(),
        }
    }
}

impl Runner for Day04 {
    fn name(&self) -> (usize, usize) {
        (2015, 4)
    }

    fn parse(&mut self, input: Option<&str>) {
        self.prefix = read_lines(input.unwrap_or(INPUT))
            .first()
            .unwrap()
            .to_string();
    }

    fn part1(&mut self) -> Vec<String> {
        let mut i = 0;
        let mut hash = String::new();
        while !hash.starts_with("00000") {
            i += 1;
            hash = format!("{:x}", md5::compute(format!("{}{}", self.prefix, i)));
        }
        output(i)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut i = 0;
        let mut hash = String::new();
        while !hash.starts_with("000000") {
            i += 1;
            hash = format!("{:x}", md5::compute(format!("{}{}", self.prefix, i)));
        }
        output(i)
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day04::Day04;

    const TEST_INPUT: &str = "input/day04-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day04::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "609043")
    }

    #[test]
    fn part1_works() {
        let mut day = Day04::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "282749")
    }

    #[test]
    fn part2_works() {
        let mut day = Day04::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "9962624")
    }
}
