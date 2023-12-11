use std::collections::HashMap;

use aoc_2015_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day16.txt";

#[derive(Debug, Default, Clone)]
pub struct Day16 {
    mapping: HashMap<String, i32>,
    sue: Vec<HashMap<String, i32>>,
}

impl Day16 {
    pub fn new() -> Self {
        let mapping = HashMap::from([
            ("children".to_string(), 3),
            ("cats".to_string(), 7),
            ("samoyeds".to_string(), 2),
            ("pomeranians".to_string(), 3),
            ("akitas".to_string(), 0),
            ("vizslas".to_string(), 0),
            ("goldfish".to_string(), 5),
            ("trees".to_string(), 3),
            ("cars".to_string(), 2),
            ("perfumes".to_string(), 1),
        ]);
        Self {
            mapping,
            sue: Vec::new(),
        }
    }
}

impl Runner for Day16 {
    fn name(&self) -> (usize, usize) {
        (2016, 16)
    }

    fn parse(&mut self, input: Option<&str>) {
        let lines = read_lines(input.unwrap_or(INPUT));
        for line in lines {
            let (_, data) = line.split_once(": ").unwrap();
            let mut sue = HashMap::new();
            for item in data.split(", ") {
                let (key, value) = item.split_once(": ").unwrap();
                sue.insert(key.to_string(), value.parse::<i32>().unwrap());
            }
            self.sue.push(sue);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        for (i, sue) in self.sue.iter().enumerate() {
            let mut found = true;
            for (key, value) in sue {
                if self.mapping.get(key).unwrap() != value {
                    found = false;
                    break;
                }
            }
            if found {
                return output((i + 1).to_string());
            }
        }
        output(-1)
    }

    fn part2(&mut self) -> Vec<String> {
        for (i, sue) in self.sue.iter().enumerate() {
            let mut found = true;
            for (key, value) in sue {
                match key.as_str() {
                    "cats" | "trees" => {
                        if self.mapping.get(key).unwrap() >= value {
                            found = false;
                            break;
                        }
                    }
                    "pomeranians" | "goldfish" => {
                        if self.mapping.get(key).unwrap() <= value {
                            found = false;
                            break;
                        }
                    }
                    _ => {
                        if self.mapping.get(key).unwrap() != value {
                            found = false;
                            break;
                        }
                    }
                }
            }
            if found {
                return output((i + 1).to_string());
            }
        }
        output(-1)
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day16::Day16;

    #[test]
    fn part1_works() {
        let mut day = Day16::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "103")
    }

    #[test]
    fn part2_works() {
        let mut day = Day16::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "405")
    }
}
