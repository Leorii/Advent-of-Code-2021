use std::fs;

mod day01;

pub use day01::Day01;

pub trait Day {
    fn new() -> Self;
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
