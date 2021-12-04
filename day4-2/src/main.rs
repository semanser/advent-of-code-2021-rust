use std::fmt;
use std::fs;

struct Cell {
    value: i32,
    marked: bool,
}

impl Cell {
    fn new(value: i32) -> Cell {
        Cell {
            value,
            marked: false,
        }
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "|{}, {}|", self.value, self.marked)
    }
}

struct Board {
    table: Vec<Vec<Cell>>,
    is_marked: bool,
}

impl Board {
    fn new() -> Board {
        Board {
            table: vec![],
            is_marked: false,
        }
    }

    fn find_and_mark(&mut self, value: i32) {
        let board_size = self.table.len();

        for i in 0..board_size {
            for j in 0..board_size {
                if self.table[i][j].value == value {
                    self.table[i][j].marked = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        let board_size = self.table.len();
        let mut col = 0;
        let mut row = 0;
        let mut winner = false;

        for i in 0..board_size {
            for j in 0..board_size {
                if self.table[i][j].marked {
                    row += 1;
                }
            }
            if row == board_size {
                winner = true;
            }
            row = 0;
        }

        for i in 0..board_size {
            for j in 0..board_size {
                if self.table[j][i].marked {
                    col += 1;
                }
            }
            if col == board_size {
                winner = true;
            }
            col = 0;
        }

        winner
    }

    fn get_sum_unmarked(&self) -> i32 {
        let mut sum = 0;
        let board_size = self.table.len();

        for i in 0..board_size {
            for j in 0..board_size {
                if !self.table[i][j].marked {
                    sum += self.table[i][j].value;
                }
            }
        }

        sum
    }
}

fn main() {
    let data = fs::read_to_string("../input.txt").expect("Unable to read file");
    let mut win_order: Vec<i32> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    let mut winner_board_id: i32 = -1;
    let mut win_number: i32 = -1;
    let mut number_of_winner_boards = 0;

    // Read all the input data
    let mut current_board = 0;
    let mut current_board_line = 0;
    data.lines().enumerate().for_each(|(i, line)| {
        if i == 0 {
            let mut order = line.split(',').map(|s| s.parse().unwrap()).collect();
            win_order.append(&mut order);
            return;
        }

        if line.trim().is_empty() {
            current_board_line = 0;
            boards.push(Board::new());
            if i > 1 {
                current_board += 1;
            }
            return;
        }

        let table_row: Vec<Cell> = line
            .split_whitespace()
            .map(|s| Cell::new(s.parse().unwrap()))
            .collect();

        boards[current_board].table.push(table_row);

        current_board_line += 1;
    });

    // Find the winner
    win_order.iter().for_each(|win_value| {
        if winner_board_id > -1 {
            return;
        }

        let boards_len = boards.len();
        boards.iter_mut().enumerate().for_each(|(i, board)| {
            if winner_board_id > -1 {
                return;
            }

            board.find_and_mark(*win_value);

            if board.is_winner() && !board.is_marked {
                board.is_marked = true;
                number_of_winner_boards += 1;
                println!("{}", i);
            }

            if number_of_winner_boards == boards_len {
                win_number = *win_value;
                winner_board_id = i as i32;
            }
        });
    });

    // Calculate the score of the winning board
    let sum_of_unmarked = boards[winner_board_id as usize].get_sum_unmarked();
    let result = sum_of_unmarked * win_number;

    println!(
        "The last winner board #{} \nSum of unmarked:{}\nWin number:{}\nResult:{}",
        winner_board_id + 1,
        sum_of_unmarked,
        win_number,
        result
    );
}
