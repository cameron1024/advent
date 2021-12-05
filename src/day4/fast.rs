use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelRefMutIterator};

use crate::input_const;

const WIDTH: usize = 5;
const ELEMS: usize = WIDTH * WIDTH;

struct Board([Option<i32>; ELEMS]);

impl Board {
    fn new(i: impl IntoIterator<Item = i32>) -> Self {
        let mut inner = [None; ELEMS];
        let vec: Vec<i32> = i.into_iter().collect();
        assert!(vec.len() == ELEMS);

        for i in 0..ELEMS {
            inner[i] = Some(vec[i]);
        }

        Self(inner)
    }

    fn remaining_sum(&self) -> i32 {
        self.0.iter().filter_map(|i| *i).sum()
    }

    fn check_row(&self, i: usize) -> bool {
        let start = i * WIDTH;
        let end = (i + 1) * WIDTH;
        self.0[start..end].iter().all(Option::is_none)
    }

    fn check_col(&self, i: usize) -> bool {
        (0..WIDTH)
            .map(|x| &self.0[i + (WIDTH * x)])
            .all(Option::is_none)
    }

    fn check_win(&self) -> bool {
        (0..WIDTH).any(|i| self.check_row(i) | self.check_col(i))
    }

    fn remove_number(&mut self, number: i32) {
        for i in self.0.iter_mut() {
            if *i == Some(number) {
                *i = None
            }
        }
    } 
}

struct Game {
    inputs: Vec<i32>,
    boards: Vec<Board>,
}

impl Game {
    fn from_str(s: impl AsRef<str>) -> Self {
        let mut lines = s.as_ref().lines();
        let input_line = lines.next().unwrap();
        let inputs = input_line.split(",").map(|s| s.parse().unwrap()).collect();

        lines.next().unwrap();

        let remaining = lines.map(ToOwned::to_owned).collect::<Vec<_>>();
        let board_strings = remaining.split(String::is_empty);

        let mut boards = vec![];

        for board in board_strings {
            let values = board
                .iter()
                .flat_map(|s| s.split(" "))
                .map(|s| s.parse().unwrap());
            boards.push(Board::new(values));
        }

        Game { boards, inputs }
    }

    fn step(&mut self) -> i32 {
        let input = self.inputs.remove(0);
        self.boards.par_iter_mut().for_each(|b| b.remove_number(input));
        input
    }

    fn single_winner(&self) -> Option<&Board> {
        self.boards.par_iter().find_any(|b| b.check_win())
    }

}

pub fn calculate1(s: impl AsRef<str>) -> i32 {
    let mut game = Game::from_str(s.as_ref());

    let (board, last_input) = loop {
        let input = game.step();
        if let Some(board) = game.single_winner() {
            break (board, input)
        }
    };

    board.remaining_sum() * last_input
}
