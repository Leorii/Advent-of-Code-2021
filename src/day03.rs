use BitCriteria::*;

pub struct Day03 {
    input: Vec<String>,
}

impl Day03 {
    pub fn new() -> Self {
        Day03 {
            input: super::parse_input(3),
        }
    }
}

impl super::Day for Day03 {
    fn p1(&self) -> String {
        let (gamma, epsilon) = transpose_to_columns(&self.input)
            .iter()
            // count zeroes and ones by column
            .map(|bits| {
                bits.chars().fold(
                    (0, 0),
                    |(z, o), bit| if bit == '0' { (z + 1, o) } else { (z, o + 1) },
                )
            })
            // build gamma and epsilon values
            .fold((0, 0), |(g, e), (z, o)| {
                if z > o {
                    (g << 1, (e << 1) | 1)
                } else {
                    ((g << 1) | 1, e << 1)
                }
            });

        format!("Part 1: {}", gamma * epsilon)
    }

    fn p2(&self) -> String {
        let o2_gen_rating = select_by_bit_criteria(O2Rating, &self.input);
        let co2_scrub_rating = select_by_bit_criteria(CO2Rating, &self.input);

        format!("Part 2: {}", o2_gen_rating * co2_scrub_rating)
    }
}

fn transpose_to_columns(bit_list: &Vec<String>) -> Vec<String> {
    let start = bit_list[0].chars().map(|_| String::new()).collect();

    bit_list.iter().fold(start, |mut acc: Vec<String>, bits| {
        for (i, bit) in bits.chars().enumerate() {
            acc[i].push(bit);
        }
        acc
    })
}

#[derive(Eq, PartialEq)]
enum BitCriteria {
    O2Rating,
    CO2Rating,
}

fn select_by_bit_criteria(bit_criteria: BitCriteria, bit_list: &Vec<String>) -> i32 {
    let mut result = bit_list.clone();

    for col in 0..bit_list[0].len() {
        if result.len() == 1 {
            break;
        }

        let median_bit = {
            let (zeroes, ones) = result.iter().fold((0, 0), |(z, o), bits| {
                if bits.chars().nth(col).unwrap() == '0' {
                    (z + 1, o)
                } else {
                    (z, o + 1)
                }
            });

            if zeroes > ones {
                '0'
            } else {
                '1'
            }
        };

        result = result
            .into_iter()
            .filter(|x| {
                let is_among_most_common = x.chars().nth(col).unwrap() == median_bit;
                if bit_criteria == O2Rating {
                    is_among_most_common
                } else {
                    !is_among_most_common
                }
            })
            .collect();
    }
    i32::from_str_radix(&result[0], 2).unwrap()
}
