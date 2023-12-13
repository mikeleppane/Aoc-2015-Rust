use aoc_2015_rust::{output, read_lines, Runner};
use itertools::Itertools;

const INPUT: &str = "input/day23.txt";

#[derive(Debug, Default, Clone)]
pub struct Day23 {
    computer: Computer,
}

impl Day23 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day23 {
    fn name(&self) -> (usize, usize) {
        (2023, 23)
    }

    fn parse(&mut self, input: Option<&str>) {
        let lines = read_lines(input.unwrap_or(INPUT));
        self.computer.instructions = lines
            .iter()
            .map(|line| Instruction::from(line.as_str()))
            .collect_vec();

        dbg!(&self.computer);
    }

    fn part1(&mut self) -> Vec<String> {
        self.computer.run();
        output(self.computer.b.to_string())
    }

    fn part2(&mut self) -> Vec<String> {
        self.computer.a = 1;
        self.computer.b = 0;
        self.computer.pc = 0;
        self.computer.run();
        output(self.computer.b.to_string())
    }
}

// ---------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Op {
    Hlf,
    Tpl,
    Inc,
    Jmp,
    Jie,
    Jio,
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Op,
    reg: String,
    offset: Option<i32>,
}

impl From<&str> for Instruction {
    fn from(s: &str) -> Self {
        let parts = s.split_whitespace().collect_vec();
        let op = match parts[0] {
            "hlf" => Op::Hlf,
            "tpl" => Op::Tpl,
            "inc" => Op::Inc,
            "jmp" => Op::Jmp,
            "jie" => Op::Jie,
            "jio" => Op::Jio,
            _ => panic!("Unknown instruction"),
        };

        if op == Op::Jmp {
            return Self::new(op, "".to_string(), Some(parts[1].parse::<i32>().unwrap()));
        }

        let reg = parts[1].trim_end_matches(',').to_string();
        let offset = if parts.len() == 3 {
            Some(parts[2].parse::<i32>().unwrap())
        } else {
            None
        };
        Self::new(op, reg, offset)
    }
}
impl Instruction {
    fn new(op: Op, reg: String, offset: Option<i32>) -> Self {
        Self { op, reg, offset }
    }
}

#[derive(Debug, Default, Clone)]
struct Computer {
    a: i32,
    b: i32,
    pc: usize,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn run(&mut self) {
        while self.pc < self.instructions.len() {
            let instruction = &self.instructions[self.pc];
            match instruction.op {
                Op::Hlf => {
                    if instruction.reg == "a" {
                        self.a /= 2;
                    } else {
                        self.b /= 2;
                    }
                    self.pc += 1;
                }
                Op::Tpl => {
                    if instruction.reg == "a" {
                        self.a *= 3;
                    } else {
                        self.b *= 3;
                    }
                    self.pc += 1;
                }
                Op::Inc => {
                    if instruction.reg == "a" {
                        self.a += 1;
                    } else {
                        self.b += 1;
                    }
                    self.pc += 1;
                }
                Op::Jmp => {
                    self.pc = (self.pc as i32 + instruction.offset.unwrap()) as usize;
                }
                Op::Jie => {
                    let reg = if instruction.reg == "a" {
                        self.a
                    } else {
                        self.b
                    };
                    if reg % 2 == 0 {
                        self.pc = (self.pc as i32 + instruction.offset.unwrap()) as usize;
                    } else {
                        self.pc += 1;
                    }
                }
                Op::Jio => {
                    let reg = if instruction.reg == "a" {
                        self.a
                    } else {
                        self.b
                    };
                    if reg == 1 {
                        self.pc = (self.pc as i32 + instruction.offset.unwrap()) as usize;
                    } else {
                        self.pc += 1;
                    }
                }
            }
        }
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day23::Day23;

    #[test]
    fn part1_works() {
        let mut day = Day23::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "307")
    }

    #[test]
    fn part2_works() {
        let mut day = Day23::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "160")
    }
}
