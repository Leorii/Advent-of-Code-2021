pub struct Day01 {
    input: Vec<i32>,
}

impl super::Day for Day01 {
    fn new() -> Self {
        Day01 {
            input: super::parse_input_i32(1),
        }
    }

    fn p1(&self) -> String {
        let (_, count) = self
            .input
            .iter()
            .fold((self.input[0], 0), |(prev, count), &n| {
                if n > prev {
                    (n, count + 1)
                } else {
                    (n, count)
                }
            });

        format!("Part 1: {}", count)
    }

    fn p2(&self) -> String {
        let start = (self.input[0], self.input[1], self.input[2]);

        let (_, count) = self
            .input
            .iter()
            .skip(3)
            .fold((start, 0), |((x, y, z), count), &n| {
                let a = x + y + z;
                let b = y + z + n;
                let next = (y, z, n);

                if b > a {
                    (next, count + 1)
                } else {
                    (next, count)
                }
            });

        format!("Part 2: {}", count)
    }
}
