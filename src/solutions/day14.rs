use aoc_2015_rust::{output, read_lines, Runner};

const INPUT: &str = "input/day14.txt";

#[derive(Debug, Default, Clone)]
pub struct Day14 {
    reindeers: Vec<Reindeer>,
}

impl Day14 {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Runner for Day14 {
    fn name(&self) -> (usize, usize) {
        (2015, 14)
    }

    fn parse(&mut self, input: Option<&str>) {
        let lines = read_lines(input.unwrap_or(INPUT));
        for line in lines {
            self.reindeers.push(Reindeer::from(line.as_str()));
        }
    }

    fn part1(&mut self) -> Vec<String> {
        let mut max_distance = 0;
        for reindeer in &self.reindeers {
            let distance = reindeer.distance_after(2503);
            max_distance = max_distance.max(distance);
        }
        output(max_distance)
    }

    fn part2(&mut self) -> Vec<String> {
        output(points_after(2503, &mut self.reindeers))
    }
}

// ---------------------------------------------------

fn points_after(seconds: u32, reindeers: &mut Vec<Reindeer>) -> u32 {
    for second in 1..=seconds {
        let mut max_distance = 0;
        for reindeer in reindeers.iter_mut() {
            let distance = reindeer.distance_after(second);
            max_distance = max_distance.max(distance);
        }
        for reindeer in reindeers.iter_mut() {
            if reindeer.distance_after(second) == max_distance {
                reindeer.points += 1;
            }
        }
    }
    let mut points = 0;
    for reindeer in reindeers {
        points = points.max(reindeer.points);
    }
    points
}

#[derive(Debug, Default, Clone)]
struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
    points: u32,
}

impl From<&str> for Reindeer {
    fn from(line: &str) -> Self {
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let speed = line[3].parse::<u32>().unwrap();
        let fly_time = line[6].parse::<u32>().unwrap();
        let rest_time = line[13].parse::<u32>().unwrap();
        Self {
            speed,
            fly_time,
            rest_time,
            ..Default::default()
        }
    }
}

impl Reindeer {
    fn distance_after(&self, seconds: u32) -> u32 {
        let cycle_time = self.fly_time + self.rest_time;
        let cycles = seconds / cycle_time;
        let remaining_seconds = seconds % cycle_time;
        let mut distance = cycles * self.speed * self.fly_time;
        if remaining_seconds > self.fly_time {
            distance += self.speed * self.fly_time;
        } else {
            distance += self.speed * remaining_seconds;
        }
        distance
    }
}

// --------------------------------------
#[cfg(test)]
mod tests {
    use aoc_2015_rust::Runner;

    use crate::solutions::day14::Day14;

    #[test]
    fn part1_works() {
        let mut day = Day14::new();
        day.parse(None);
        let output = day.part1();
        assert_eq!(output[0], "2640")
    }

    #[test]
    fn part2_works() {
        let mut day = Day14::new();
        day.parse(None);
        let output = day.part2();
        assert_eq!(output[0], "1102")
    }
}
