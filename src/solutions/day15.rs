use aoc_2015_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day15.txt";

#[derive(Debug, Default, Clone)]
pub struct Day15 {}

impl Day15 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day15 {
    fn name(&self) -> (usize, usize) {
        (2015, 15)
    }

    fn parse(&mut self, input: Option<&str>) {
        let _ = read_lines(input.unwrap_or(INPUT));
    }

    fn part1(&mut self) -> Vec<String> {
        let mut best: i64 = 0;
        for sprinkles in 0..100 {
            for candy in 0..(100 - sprinkles) {
                for butterscotch in 0..(100 - sprinkles - candy) {
                    let chocolate = 100 - sprinkles - candy - butterscotch;
                    if chocolate < 0 {
                        continue;
                    }
                    let capacity = (2 * sprinkles).max(0);
                    let durability = (5 * butterscotch - candy).max(0);
                    let flavor = (5 * chocolate - 3 * butterscotch - 2 * sprinkles).max(0);
                    let texture = (5 * candy - chocolate).max(0);

                    best = best.max(capacity * durability * flavor * texture);
                }
            }
        }
        output(best)
    }

    fn part2(&mut self) -> Vec<String> {
        let mut best: i64 = 0;
        let mut best_total_score = 0;
        for sprinkles in 0..100 {
            for candy in 0..(100 - sprinkles) {
                for butterscotch in 0..(100 - sprinkles - candy) {
                    let chocolate = 100 - sprinkles - candy - butterscotch;
                    if chocolate < 0 {
                        continue;
                    }
                    let capacity = (2 * sprinkles).max(0);
                    let durability = (5 * butterscotch - candy).max(0);
                    let flavor = (5 * chocolate - 3 * butterscotch - 2 * sprinkles).max(0);
                    let texture = (5 * candy - chocolate).max(0);

                    let score = capacity * durability * flavor * texture;
                    best = best.max(score);

                    let calories = 3 * sprinkles + 3 * butterscotch + 8 * chocolate + 8 * candy;
                    if calories == 500 {
                        best_total_score = best_total_score.max(score);
                    }
                }
            }
        }
        output(best_total_score)
    }
}

// ---------------------------------------------------

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day15::Day15;

    #[test]
    fn part1_works() {
        let mut day = Day15::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "21367368")
    }

    #[test]
    fn part2_works() {
        let mut day = Day15::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "1766400")
    }
}
