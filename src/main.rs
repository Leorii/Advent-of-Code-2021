use aoc_2021::*;

fn main() {
    let days = vec![Day01::new()];

    println!("ADVENT OF CODE");

    for (i, day) in days.iter().enumerate() {
        print_day(i as u8 + 1, day);
    }
}

fn print_day<D: Day>(n: u8, day: &D) {
    println!();
    println!("Day {:02}:", n);
    println!("  {}", day.p1());
    println!("  {}", day.p2());
}
