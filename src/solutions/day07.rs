use std::collections::HashMap;

use aoc_2015_rust::{output, read_lines, Runner};
const INPUT: &str = "input/day07.txt";

pub struct Day07 {
    commands: Vec<Command>,
}

impl Day07 {
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
}

impl Runner for Day07 {
    fn name(&self) -> (usize, usize) {
        (2015, 7)
    }

    fn parse(&mut self, input: Option<&str>) {
        for line in read_lines(input.unwrap_or(INPUT)) {
            self.commands.push(Command::parse(&line))
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let a_signal = find_a_signal(&self.commands);
        output(a_signal)
    }

    fn part2(&mut self) -> Vec<String> {
        let a_signal = find_a_signal(&self.commands);
        for command in &mut self.commands {
            if let Command::Signal(signal, wire) = command {
                if wire == "b" {
                    *signal = a_signal.to_string();
                }
            }
        }
        let a_signal = find_a_signal(&self.commands);
        output(a_signal)
    }
}

// ---------------------------------------------------

fn find_a_signal(commands: &Vec<Command>) -> u16 {
    let mut signals = HashMap::new();
    loop {
        for command in commands {
            match command {
                Command::Signal(signal, wire) => {
                    if let Some(signal) = signals.get(signal) {
                        signals.insert(wire, *signal);
                    } else if let Ok(signal) = signal.parse::<u16>() {
                        signals.insert(wire, signal);
                    }
                }
                Command::And(wire_1, wire_2, wire_3) => {
                    if let Ok(signal) = wire_1.parse::<u16>() {
                        if let Some(signal_2) = signals.get(&wire_2) {
                            signals.insert(wire_3, signal & *signal_2);
                        }
                    } else if let Ok(signal) = wire_2.parse::<u16>() {
                        if let Some(signal_1) = signals.get(&wire_1) {
                            signals.insert(wire_3, *signal_1 & signal);
                        }
                    } else if let Some(signal_1) = signals.get(&wire_1) {
                        if let Some(signal_2) = signals.get(&wire_2) {
                            signals.insert(wire_3, *signal_1 & *signal_2);
                        }
                    }
                }
                Command::Or(wire_1, wire_2, wire_3) => {
                    if let Some(signal_1) = signals.get(&wire_1) {
                        if let Some(signal_2) = signals.get(&wire_2) {
                            signals.insert(wire_3, *signal_1 | *signal_2);
                        }
                    }
                }
                Command::LShift(wire_1, num, wire_3) => {
                    if let Some(signal_1) = signals.get(&wire_1) {
                        signals.insert(wire_3, *signal_1 << *num);
                    }
                }
                Command::RShift(wire_1, num, wire_3) => {
                    if let Some(signal_1) = signals.get(&wire_1) {
                        signals.insert(wire_3, *signal_1 >> *num);
                    }
                }
                Command::Not(wire_1, wire_2) => {
                    if let Some(signal_1) = signals.get(&wire_1) {
                        signals.insert(wire_2, !*signal_1);
                    }
                }
            }
        }
        if signals.contains_key(&"a".to_string()) {
            break;
        }
    }
    *signals.get(&"a".to_string()).unwrap()
}

#[derive(Debug, Clone)]
enum Command {
    Signal(String, String),
    And(String, String, String),
    Or(String, String, String),
    LShift(String, u16, String),
    RShift(String, u16, String),
    Not(String, String),
}

impl Command {
    fn parse(s: &str) -> Self {
        let s = s.trim();
        if s.is_empty() {
            panic!("empty string")
        }
        if s.contains("AND") {
            let (wire_1, rest) = s.split_once(" AND ").unwrap();
            let (wire_2, wire_3) = rest.split_once(" -> ").unwrap();
            Self::And(wire_1.to_string(), wire_2.to_string(), wire_3.to_string())
        } else if s.contains("OR") {
            let (wire_1, rest) = s.split_once(" OR ").unwrap();
            let (wire_2, wire_3) = rest.split_once(" -> ").unwrap();
            return Self::Or(wire_1.to_string(), wire_2.to_string(), wire_3.to_string());
        } else if s.contains("LSHIFT") {
            let (wire_1, rest) = s.split_once(" LSHIFT ").unwrap();
            let (num, wire_3) = rest.split_once(" -> ").unwrap();
            return Self::LShift(
                wire_1.to_string(),
                num.parse::<u16>().unwrap(),
                wire_3.to_string(),
            );
        } else if s.contains("RSHIFT") {
            let (wire_1, rest) = s.split_once(" RSHIFT ").unwrap();
            let (num, wire_3) = rest.split_once(" -> ").unwrap();
            return Self::RShift(
                wire_1.to_string(),
                num.parse::<u16>().unwrap(),
                wire_3.to_string(),
            );
        } else if s.contains("NOT") {
            let (_, rest) = s.split_once("NOT ").unwrap();
            let (wire_1, wire_2) = rest.split_once(" -> ").unwrap();
            return Self::Not(wire_1.to_string(), wire_2.to_string());
        } else {
            let (signal, wire) = s.split_once(" -> ").unwrap();
            return Self::Signal(signal.to_string(), wire.to_string());
        }
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day07::Day07;

    const TEST_INPUT: &str = "input/day07-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day07::new();
        day.parse(Some(TEST_INPUT));
        let _ = day.part1();
    }

    #[test]
    fn part1_works() {
        let mut day = Day07::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "956")
    }

    #[test]
    fn part2_works() {
        let mut day = Day07::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "40149")
    }
}
