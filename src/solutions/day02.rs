use aoc_2015_rust::read_lines;
use aoc_2015_rust::{output, Runner};

const INPUT: &str = "input/day02.txt";

pub struct Day02 {
    presents: Vec<Present>,
}

impl Day02 {
    pub fn new() -> Self {
        Self {
            presents: Vec::new(),
        }
    }
}

impl Runner for Day02 {
    fn name(&self) -> (usize, usize) {
        (2015, 2)
    }

    fn parse(&mut self, input: Option<&str>) {
        for line in read_lines(input.unwrap_or(INPUT)) {
            self.presents.push(Present::new(&line));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(self.presents.iter().fold(0, |acc, p| acc + p.area()))
    }

    fn part2(&mut self) -> Vec<String> {
        output(self.presents.iter().fold(0, |acc, p| acc + p.ribbon()))
    }
}

// ---------------------------------------------------

struct Present([u32; 3]);

impl Present {
    fn new(s: &str) -> Self {
        let mut v = s.split('x').map(|x| x.parse::<u32>().unwrap());
        Self([v.next().unwrap(), v.next().unwrap(), v.next().unwrap()])
    }

    fn area(&self) -> u32 {
        let a = self.0[0] * self.0[1];
        let b = self.0[1] * self.0[2];
        let c = self.0[2] * self.0[0];

        2 * (a + b + c) + [a, b, c].iter().min().unwrap()
    }

    fn ribbon(&self) -> u32 {
        let a = self.0[0] + self.0[0] + self.0[1] + self.0[1];
        let b = self.0[1] + self.0[1] + self.0[2] + self.0[2];
        let c = self.0[2] + self.0[2] + self.0[0] + self.0[0];

        [a, b, c].iter().min().unwrap() + self.0[0] * self.0[1] * self.0[2]
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day02::Day02;

    const TEST_INPUT: &str = "input/day02-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day02::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "101")
    }

    #[test]
    fn part1_works() {
        let mut day = Day02::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "1606483")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day02::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "48")
    }

    #[test]
    fn part2_works() {
        let mut day = Day02::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "3842356")
    }
}
