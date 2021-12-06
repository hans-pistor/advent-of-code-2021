use std::fmt;
use std::fmt::Formatter;

type BoardPosition = (bool, usize);

#[derive(Clone)]
pub struct BingoBoard {
    board: Vec<Vec<BoardPosition>>
}

impl BingoBoard {
    pub fn new(board: Vec<Vec<BoardPosition>>) -> BingoBoard {
        BingoBoard { board }
    }

    pub fn mark(&mut self, number: usize) {
        self.board = self.board
            .iter_mut()
            .map(|row| row.iter_mut()
                .map(|pos| match pos.1 == number {
                    true => (true, number),
                    false => *pos
                })
                .collect())
            .collect();
    }

    fn has_horizontal(&self) -> bool {
        self.board
            .iter()
            .any(|row| row.iter()
                .map(|pos| pos.0)
                .all(|state| state == true))
    }

    fn has_vertical(&self) -> bool {
        for i in 0..self.board.len() {
            if self.board
                .iter()
                .map(|row| row[i])
                .map(|pos| pos.0)
                .all(|state| state == true) {
                return true;
            }
        }

        return false;
    }

    pub fn is_valid(&self) -> bool {
        self.has_diagonal() || self.has_horizontal() || self.has_vertical()
    }

    fn has_diagonal(&self) -> bool {
        let mut ltr_diag = true;
        let mut rtl_diag = true;
        for i in 0..self.board.len() {
            if self.board[i][i].0 != true {
                ltr_diag = false;
            }

            if self.board[i][self.board.len() - i - 1].0 != true {
                rtl_diag = false;
            }
        }

        ltr_diag || rtl_diag
    }

    pub fn get_marked_numbers(&self) -> Vec<usize> {
        self.board
            .iter()
            .map(|row| row.iter()
                .filter(|pos| pos.0)
                .map(|pos| pos.1)
                .collect::<Vec<usize>>()
            )
            .flatten()
            .collect::<Vec<usize>>()
    }

    pub fn get_unmarked_numbers(&self) -> Vec<usize> {
        self.board
            .iter()
            .map(|row| row.iter()
                .filter(|pos| !pos.0)
                .map(|pos| pos.1)
                .collect::<Vec<usize>>()
            )
            .flatten()
            .collect::<Vec<usize>>()
    }
}

impl fmt::Display for BingoBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.board
            .iter()
            .for_each(|row| {
                row
                    .iter()
                    .for_each(|pos| {
                        let state = match pos.0 {
                            false => "f",
                            true => "t"
                        };
                        write!(f, "({}, {: >3}) ", state, pos.1).unwrap();
                    });
                write!(f, "\n");
            });

        write!(f, "is valid? {}", self.is_valid()).unwrap();
        Ok(())
    }
}

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<BingoBoard>) {
    let mut lines = input.lines();
    let numbers_to_call = lines.next().unwrap()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect::<Vec<usize>>();

    let _ = lines.next(); // blank line

    let boards_strings = lines
        .map(|str| str.trim())
        .collect::<Vec<&str>>();

    let mut index = 0;
    let mut bingo_boards: Vec<BingoBoard> = Vec::new();
    while index < boards_strings.len() {
        let mut board_string = String::new();
        while index < boards_strings.len() && !boards_strings[index].is_empty() {
            board_string += boards_strings[index];
            board_string += "\n";
            index += 1;
        }
        let current_board = board_string
            .split("\n")
            .filter(|row| !row.is_empty())
            .map(|row| row.trim().split_ascii_whitespace()
                .map(|num| (false, num.parse().unwrap()))
                .collect()
            )
            .collect();
        bingo_boards.push(BingoBoard::new(current_board));
        index += 1;
    }

    (numbers_to_call, bingo_boards)
}

#[aoc(day4, part1)]
pub fn solve_part_one(bingo: &(Vec<usize>, Vec<BingoBoard>)) -> usize {
    let (numbers_to_call, boards) = bingo;
    let mut my_boards = boards.clone();

    let mut winning_board: Option<&BingoBoard> = Option::None;

    let mut number_iter = numbers_to_call.iter();
    let mut num: &usize = &0;


    while winning_board.is_none() {
        num = number_iter.next().unwrap();

        let mut valid_boards = my_boards
            .iter_mut()
            .map(|board| {
                board.mark(*num);
                board
            })
            .filter(|board| board.is_valid());

        winning_board = match valid_boards.next() {
            Some(board) => Some(board),
            None => None
        };

    }

    let winning_board = winning_board.unwrap();

    winning_board.get_unmarked_numbers().iter().sum::<usize>() * num
}

#[aoc(day4, part2)]
pub fn solve_part_two(bingo: &(Vec<usize>, Vec<BingoBoard>)) -> usize {
    let (numbers_to_call, boards) = bingo;
    let mut my_boards = boards.clone();

    let mut number_iter = numbers_to_call.iter();
    let mut num: &usize = &0;

    while my_boards.len() > 1 {
        num = number_iter.next().unwrap();

        my_boards
            .iter_mut()
            .for_each(|board| board.mark(*num));

        my_boards = my_boards
            .iter()
            .filter(|board| !board.is_valid())
            .map(|board| board.clone())
            .collect();
    }

    let mut last_board = my_boards.first().unwrap().clone();

    let mut final_board: Option<&BingoBoard> = None;
    while final_board.is_none() {
        num = number_iter.next().unwrap();
        last_board.mark(*num);

        final_board = match last_board.is_valid() {
            true => Some(&last_board),
            false => None
        };
    }


    final_board.unwrap().get_unmarked_numbers().iter().sum::<usize>() * num
}
