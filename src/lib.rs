use std::fs;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
pub use day04::Day04;
pub use day05::Day05;
pub use day06::Day06;

pub trait Day {
    fn p1(&self) -> String;
    fn p2(&self) -> String;
}

pub fn parse_input(day: u8) -> Vec<String> {
    fs::read_to_string(format!("./challenge-input/day{:02}.txt", day))
        .unwrap()
        .lines()
        .map(|x| x.into())
        .collect()
}

pub fn parse_input_i32(day: u8) -> Vec<i32> {
    parse_input(day)
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}
