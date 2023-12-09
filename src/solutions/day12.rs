use aoc_2015_rust::{output, read_lines, OutputStatus, Runner};

const INPUT: &str = "input/day12.txt";

#[derive(Debug, Default, Clone)]
pub struct Day12 {
    json_document: String,
}

impl Day12 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day12 {
    fn name(&self) -> (usize, usize) {
        (2015, 12)
    }

    fn parse(&mut self, input: Option<&str>) {
        let lines = read_lines(input.unwrap_or(INPUT));
        self.json_document = lines[0].clone();
    }

    fn part1(&mut self) -> Vec<String> {
        output(
            find_all_numbers_from_json_document(&self.json_document)
                .iter()
                .sum::<i64>(),
        )
    }

    fn part2(&mut self) -> Vec<String> {
        output(OutputStatus::Unsolved)
    }
}

// ---------------------------------------------------

fn find_all_numbers_from_json_document(document: &str) -> Vec<i64> {
    let mut numbers = Vec::new();
    let mut number = String::new();
    let mut in_number = false;
    for c in document.chars() {
        if c.is_ascii_digit() || c == '-' {
            number.push(c);
            in_number = true;
        } else if in_number {
            numbers.push(number.parse::<i64>().unwrap());
            number.clear();
            in_number = false;
        }
    }
    numbers
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day12::Day12;

    #[test]
    fn part1_works() {
        let mut day = Day12::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "119433")
    }

    #[test]
    fn part2_works() {
        let mut day = Day12::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "6")
    }
}
