use std::collections::HashMap;

use aoc_2015_rust::{output, read_lines, Runner};
use itertools::Itertools;

const INPUT: &str = "input/day13.txt";

#[derive(Debug, Default, Clone)]
pub struct Day13 {
    persons: HashMap<String, Person>,
}

impl Day13 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day13 {
    fn name(&self) -> (usize, usize) {
        (2015, 13)
    }

    fn parse(&mut self, input: Option<&str>) {
        let lines = read_lines(input.unwrap_or(INPUT));
        for line in lines {
            let line = line.split_whitespace().collect::<Vec<&str>>();
            let name = line[0];
            let happiness_sign = line[2];
            let next_to = line[10].trim_end_matches('.');
            let happiness = match happiness_sign {
                "gain" => line[3].parse::<Happiness>().unwrap(),
                "lose" => -line[3].parse::<Happiness>().unwrap(),
                _ => panic!("Unknown happiness sign: {}", happiness_sign),
            };
            if self.persons.contains_key(name) {
                self.persons
                    .get_mut(name)
                    .unwrap()
                    .happiness
                    .insert(next_to.to_string(), happiness);
            } else {
                let mut person = Person {
                    name: name.to_string(),
                    ..Default::default()
                };
                person.happiness.insert(next_to.to_string(), happiness);
                self.persons.insert(name.to_string(), person);
            }
        }
    }

    fn part1(&mut self) -> Vec<String> {
        output(find_optimal_seating_order(&self.persons))
    }

    fn part2(&mut self) -> Vec<String> {
        for person in self.persons.values_mut() {
            person.happiness.insert("Me".to_string(), 0);
        }
        let mut me = Person {
            name: "Me".to_string(),
            happiness: HashMap::new(),
        };
        for person in self.persons.values() {
            me.happiness.insert(person.name.clone(), 0);
        }
        self.persons.insert("Me".to_string(), me);
        output(find_optimal_seating_order(&self.persons))
    }
}

// ---------------------------------------------------

fn find_optimal_seating_order(persons: &HashMap<String, Person>) -> Happiness {
    let persons = persons.clone();
    let persons_names = persons.keys().cloned().collect::<Vec<String>>();
    let mut total_happiness = 0;
    for order in persons_names.iter().permutations(persons_names.len()) {
        let mut happiness = 0;
        for i in 0..order.len() {
            let person = persons.get(order[i]).unwrap();
            let left = if i == 0 {
                persons.get(order[order.len() - 1]).unwrap()
            } else {
                persons.get(order[i - 1]).unwrap()
            };
            let right = if i == order.len() - 1 {
                persons.get(order[0]).unwrap()
            } else {
                persons.get(order[i + 1]).unwrap()
            };
            happiness += person.happiness.get(&left.name).unwrap();
            happiness += person.happiness.get(&right.name).unwrap();
        }
        total_happiness = total_happiness.max(happiness);
    }
    total_happiness
}

type Happiness = i32;

#[derive(Debug, Default, Clone)]
struct Person {
    name: String,
    happiness: HashMap<String, Happiness>,
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day13::Day13;

    const TEST_INPUT: &str = "input/day13-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day13::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "330")
    }

    #[test]
    fn part1_works() {
        let mut day = Day13::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "664")
    }

    #[test]
    fn part2_works() {
        let mut day = Day13::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "640")
    }
}
