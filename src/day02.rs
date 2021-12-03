use Direction::*;

pub struct Day02 {
    input: Vec<Direction>,
}

impl Day02 {
    pub fn new() -> Self {
        Day02 {
            input: super::parse_input(2)
                .into_iter()
                .map(|x| x.into())
                .collect(),
        }
    }
}

impl super::Day for Day02 {
    fn p1(&self) -> String {
        let (hpos, depth) =
            self.input
                .iter()
                .fold((0, 0), |(hpos, depth), direction| match direction {
                    Forward(x) => (hpos + x, depth),
                    Up(x) => (hpos, depth - x),
                    Down(x) => (hpos, depth + x),
                });

        format!("Part 1: {}", hpos * depth)
    }

    fn p2(&self) -> String {
        let (_, hpos, depth) =
            self.input
                .iter()
                .fold((0, 0, 0), |(aim, hpos, depth), direction| match direction {
                    Forward(x) => (aim, hpos + x, depth + aim * x),
                    Up(x) => (aim - x, hpos, depth),
                    Down(x) => (aim + x, hpos, depth),
                });

        format!("Part 2: {}", hpos * depth)
    }
}

enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl From<String> for Direction {
    fn from(item: String) -> Self {
        let parts = item.split(" ").collect::<Vec<_>>();

        match parts[0] {
            "forward" => Forward(parts[1].parse::<i32>().unwrap()),
            "up" => Up(parts[1].parse::<i32>().unwrap()),
            _ => Down(parts[1].parse::<i32>().unwrap()),
        }
    }
}
