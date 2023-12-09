use std::collections::HashSet;

use aoc_2015_rust::{output, read_lines, Runner};
use itertools::Itertools;
const INPUT: &str = "input/day09.txt";

#[derive(Debug, Default, Clone)]
pub struct Day09 {
    routes: Vec<Route>,
}

impl Day09 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day09 {
    fn name(&self) -> (usize, usize) {
        (2015, 9)
    }

    fn parse(&mut self, input: Option<&str>) {
        let lines = read_lines(input.unwrap_or(INPUT));
        for line in lines {
            let line = line.trim();
            let route = Route::from_str(line);
            self.routes.push(route);
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let total_distances = calculate_distances(&self.routes);
        output(total_distances.iter().min().unwrap())
    }

    fn part2(&mut self) -> Vec<String> {
        let total_distances = calculate_distances(&self.routes);
        output(total_distances.iter().max().unwrap())
    }
}

// ---------------------------------------------------

fn calculate_distances(routes: &[Route]) -> Vec<u32> {
    let locations = get_all_locations(routes);
    let mut total = Vec::new();
    for permutation in locations.iter().permutations(locations.len()) {
        let mut distance = 0;
        for i in 0..permutation.len() - 1 {
            let from = permutation[i];
            let to = permutation[i + 1];
            let route = routes
                .iter()
                .find(|&r| (&r.from == from && &r.to == to) || (&r.from == to && &r.to == from))
                .unwrap();
            distance += route.distance;
        }
        total.push(distance);
    }
    total
}

fn get_all_locations(routes: &[Route]) -> HashSet<String> {
    let mut locations = HashSet::new();
    routes.iter().for_each(|r| {
        locations.insert(r.from.clone());
        locations.insert(r.to.clone());
    });
    locations
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Route {
    from: String,
    to: String,
    distance: u32,
}

impl Route {
    fn new(from: String, to: String, distance: u32) -> Self {
        Self { from, to, distance }
    }

    fn from_str(s: &str) -> Self {
        let mut parts = s.split(" = ");
        let mut route = parts.next().unwrap().split(" to ");
        let from = route.next().unwrap().to_string();
        let to = route.next().unwrap().to_string();
        let distance = parts.next().unwrap().parse::<u32>().unwrap();
        Self::new(from, to, distance)
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day09::Day09;

    const TEST_INPUT: &str = "input/day09-test.txt";

    #[test]
    fn part1_test_works() {
        let mut day = Day09::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part1();
        assert_eq!(output[0], "605")
    }

    #[test]
    fn part1_works() {
        let mut day = Day09::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "207")
    }

    #[test]
    fn part2_test_works() {
        let mut day = Day09::new();
        day.parse(Some(TEST_INPUT));
        let output = day.part2();
        assert_eq!(output[0], "982")
    }

    #[test]
    fn part2_works() {
        let mut day = Day09::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "804")
    }
}
