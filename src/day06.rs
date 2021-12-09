use std::collections::HashMap;

pub struct Day06 {
    input: Vec<usize>,
}

impl Day06 {
    pub fn new() -> Self {
        Day06 {
            input: str::split(&super::parse_input(6)[0], ",")
                .map(|n| str::parse::<usize>(n).unwrap())
                .collect(),
        }
    }
}

impl super::Day for Day06 {
    fn p1(&self) -> String {
        let count = predict_pupulation_size(self.input.clone(), 80);
        format!("Part 1: {}", count)
    }

    fn p2(&self) -> String {
        let count = predict_pupulation_size(self.input.clone(), 256);
        format!("Part 2: {}", count)
    }
}

fn predict_pupulation_size(fishies: Vec<usize>, days: usize) -> usize {
    let reproduction_map_base = HashMap::from([
        (0, 0),
        (1, 0),
        (2, 0),
        (3, 0),
        (4, 0),
        (5, 0),
        (6, 0),
        (7, 0),
        (8, 0),
    ]);
    let mut reproduction_map = reproduction_map_base.clone();
    for fishy in fishies.iter() {
        if let Some(count) = reproduction_map.get_mut(&fishy) {
            *count += 1;
        }
    }

    for _ in 0..days {
        let mut updated = reproduction_map_base.clone();

        for cycle in (0..=8).rev() {
            let count = reproduction_map[&cycle];

            if count == 0 {
                continue;
            }

            if cycle == 0 {
                if let Some(x) = updated.get_mut(&6) {
                    *x += count;
                }
                updated.insert(8, count);
                continue;
            }

            updated.insert(cycle - 1, count);
        }

        reproduction_map = updated;
    }

    reproduction_map.iter().map(|(_, count)| count).sum()
}
