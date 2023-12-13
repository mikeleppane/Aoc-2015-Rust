use aoc_2015_rust::{output, Runner};

#[derive(Debug, Default, Clone)]
pub struct Day20 {}

impl Day20 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day20 {
    fn name(&self) -> (usize, usize) {
        (2020, 20)
    }

    fn parse(&mut self, _: Option<&str>) {}

    fn part1(&mut self) -> Vec<String> {
        output(find_lowest_house_number(34000000))
    }

    fn part2(&mut self) -> Vec<String> {
        output(find_lowest_house_number_with_50_stops(34000000))
    }
}

// ---------------------------------------------------

fn find_lowest_house_number(target: usize) -> usize {
    let mut houses = vec![0; target / 10];
    for elf in 1..target / 10 {
        for house in (elf..target / 10).step_by(elf) {
            houses[house] += elf * 10;
        }
    }
    houses.iter().position(|&p| p >= target).unwrap()
}

fn find_lowest_house_number_with_50_stops(target: usize) -> usize {
    let mut houses = vec![0; target / 10];
    for elf in 1..target / 10 {
        for house in (elf..target / 10).step_by(elf).take(50) {
            houses[house] += elf * 11;
        }
    }
    houses.iter().position(|&p| p >= target).unwrap()
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day20::Day20;

    #[test]
    fn part1_works() {
        let mut day = Day20::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "786240")
    }

    #[test]
    fn part2_works() {
        let mut day = Day20::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "831600")
    }
}
