use std::collections::BTreeMap;

use crate::Selector;
use aoc_2015_rust::{run_solution, run_solution_with_part, Runner};
use day01::Day01;
use day02::Day02;
use day03::Day03;
use day04::Day04;
use day05::Day05;
use day06::Day06;
use day07::Day07;
use day08::Day08;
use day09::Day09;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day18::Day18;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day18;

pub fn run(which: Selector) {
    let mut day01 = Day01::new();
    let mut day02 = Day02::new();
    let mut day03 = Day03::new();
    let mut day04 = Day04::new();
    let mut day05 = Day05::new();
    let mut day06 = Day06::new();
    let mut day07 = Day07::new();
    let mut day08 = Day08::new();
    let mut day09 = Day09::new();
    let mut day10 = Day10::new();
    let mut day11 = Day11::new();
    let mut day12 = Day12::new();
    let mut day13 = Day13::new();
    let mut day14 = Day14::new();
    let mut day15 = Day15::new();
    let mut day16 = Day16::new();
    let mut day18 = Day18::new();

    let mut days: BTreeMap<u8, &mut dyn Runner> = BTreeMap::new();
    days.insert(1, &mut day01);
    days.insert(2, &mut day02);
    days.insert(3, &mut day03);
    days.insert(4, &mut day04);
    days.insert(5, &mut day05);
    days.insert(6, &mut day06);
    days.insert(7, &mut day07);
    days.insert(8, &mut day08);
    days.insert(9, &mut day09);
    days.insert(10, &mut day10);
    days.insert(11, &mut day11);
    days.insert(12, &mut day12);
    days.insert(13, &mut day13);
    days.insert(14, &mut day14);
    days.insert(15, &mut day15);
    days.insert(16, &mut day16);
    days.insert(18, &mut day18);

    match which {
        Selector::Last => {
            let last = *days.keys().max().unwrap();
            if let Some(d) = days.get_mut(&last) {
                run_solution(*d);
            }
        }
        Selector::All => {
            for d in days.values_mut() {
                run_solution(*d);
            }
        }
        Selector::One(day) => {
            if day.len() == 1 {
                if let Some(d) = days.get_mut(&{ day[0] }) {
                    run_solution(*d);
                }
            } else if let Some(d) = days.get_mut(&{ day[0] }) {
                run_solution_with_part(*d, day[1]);
            }
        }
    }
}
