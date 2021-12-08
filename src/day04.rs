pub struct Day04 {
    number_sequence: Vec<u32>,
    boards: Vec<BingoBoard>,
}

impl Day04 {
    pub fn new() -> Self {
        let input = super::parse_input(4);

        Day04 {
            number_sequence: str::split(&input[0], ",")
                .map(|n| str::parse::<u32>(n).unwrap())
                .collect(),

            boards: input.into_iter().skip(1).fold(vec![], |mut boards, row| {
                if row == "" {
                    boards.push(BingoBoard::new());
                    return boards;
                }

                let last_board_idx = boards.len() - 1;
                boards[last_board_idx].add_row(
                    &row.split_whitespace()
                        .map(|n| str::parse::<u32>(n).unwrap())
                        .collect(),
                );
                boards
            }),
        }
    }
}

impl super::Day for Day04 {
    fn p1(&self) -> String {
        let mut boards = self.boards.clone();

        for &n in self.number_sequence.iter() {
            for board in boards.iter_mut() {
                board.mark_number(n);
                if board.is_winning_board() {
                    return format!("Part 1: {}", board.calculate_score());
                }
            }
        }
        format!("Part 1: ERROR! COULD NOT FIND WINNING BOARD")
    }

    fn p2(&self) -> String {
        let mut boards = self.boards.clone();

        for &n in self.number_sequence.iter() {
            let mut unfinished_boards = boards
                .iter_mut()
                .filter(|x| !x.is_winning_board())
                .collect::<Vec<&mut BingoBoard>>();
            let is_last_board = unfinished_boards.len() == 1;

            for board in unfinished_boards.iter_mut() {
                board.mark_number(n);
                if is_last_board && board.is_winning_board() {
                    return format!("Part 1: {}", board.calculate_score());
                }
            }
        }
        format!("Part 2: ERROR! COULD NOT FIND WINNING BOARD")
    }
}

#[derive(Clone)]
enum BingoNumber {
    Selected(u32),
    UnSelected(u32),
}

impl BingoNumber {
    fn is_selected(&self) -> bool {
        match self {
            BingoNumber::Selected(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
struct BingoBoard {
    rows: Vec<Vec<BingoNumber>>,
    last_called_number: u32,
}

impl BingoBoard {
    fn new() -> Self {
        BingoBoard {
            rows: vec![],
            last_called_number: 0,
        }
    }

    fn add_row(&mut self, row: &Vec<u32>) {
        self.rows
            .push(row.iter().map(|&n| BingoNumber::UnSelected(n)).collect());
    }

    fn mark_number(&mut self, n: u32) {
        self.last_called_number = n;

        for row in 0..self.rows.len() {
            for col in 0..self.rows.len() {
                match self.rows[row][col] {
                    BingoNumber::UnSelected(x) if x == n => {
                        self.rows[row][col] = BingoNumber::Selected(n);
                    }
                    _ => (),
                }
            }
        }
    }

    fn is_winning_board(&self) -> bool {
        for i in 0..self.rows.len() {
            if self.is_winning_row(i) || self.is_winning_col(i) {
                return true;
            }
        }
        false
    }

    fn is_winning_row(&self, row_idx: usize) -> bool {
        self.rows[row_idx]
            .iter()
            .fold(true, |acc, n| acc && n.is_selected())
    }

    fn is_winning_col(&self, col_idx: usize) -> bool {
        self.rows
            .iter()
            .fold(true, |acc, row| acc && row[col_idx].is_selected())
    }

    fn calculate_score(&self) -> u32 {
        self.rows.iter().fold(0, |sum_rows, row| {
            sum_rows
                + row
                    .iter()
                    .filter_map(|n| match n {
                        BingoNumber::UnSelected(x) => Some(x),
                        _ => None,
                    })
                    .sum::<u32>()
        }) * self.last_called_number
    }
}
