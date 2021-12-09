use aoc_2021::*;

fn main() {
    let days: Vec<Box<dyn Day>> = vec![
        Box::new(Day01::new()),
        Box::new(Day02::new()),
        Box::new(Day03::new()),
        Box::new(Day04::new()),
        Box::new(Day05::new()),
        Box::new(Day06::new()),
    ];

    println!("ADVENT OF CODE");

    for (i, day) in days.iter().enumerate() {
        print_day(i as u8 + 1, day);
    }
}

fn print_day(n: u8, day: &Box<dyn Day>) {
    println!();
    println!("Day {:02}:", n);
    println!("  {}", day.p1());
    println!("  {}", day.p2());
}
