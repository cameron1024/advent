mod fast;

use std::collections::HashMap;

use crate::input_lines;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    inputs: Vec<i32>,
    boards: Vec<Board>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Board {
    map: HashMap<(usize, usize), Option<i32>>,
}

fn input() -> impl IntoIterator<Item = &'static str> {
    input_lines!("4")
}

pub fn solution2() -> i64 {
    let game = parse_input(input());
    calculate_2(game)
}

fn calculate_2(mut game: Game) -> i64 {
    let (last_input, loser_index) = 'outer: loop {
        let mut input = step_game(&mut game);

        let mut loser_indices = vec![];

        for (i, board) in game.boards.iter().enumerate() {
            if !check_win(board) {
                loser_indices.push(i)
            }
        }

        if loser_indices.len() == 1 {
            let index = loser_indices[0];
            while !check_win(&game.boards[index]) {
                input = step_game(&mut game);
            }
            break 'outer (input, loser_indices[0]);
        }
    };

    let remaining_numbers: i32 = game.boards[loser_index]
        .map
        .values()
        .map(|i| i.unwrap_or(0))
        .sum();

    remaining_numbers as i64 * last_input as i64
}

pub fn solution1() -> i64 {
    let game = parse_input(input());
    calculate(game)
}

fn calculate(mut game: Game) -> i64 {
    let (last_input, winner_index) = 'outer: loop {
        let input = step_game(&mut game);

        for (index, board) in game.boards.iter().enumerate() {
            if check_win(board) {
                break 'outer (input, index);
            }
        }
    };

    let remaining_numbers: i32 = game.boards[winner_index]
        .map
        .values()
        .map(|i| i.unwrap_or(0))
        .sum();

    remaining_numbers as i64 * last_input as i64
}

fn parse_input(lines: impl IntoIterator<Item = &'static str>) -> Game {
    let mut lines = lines.into_iter();
    let inputs = lines.next().unwrap();
    let inputs: Vec<i32> = inputs.split(",").map(|s| s.parse().unwrap()).collect();

    lines.next().unwrap();
    let lines: Vec<_> = lines.map(ToOwned::to_owned).collect();

    let splits = lines.split(String::is_empty);

    let mut boards = vec![];

    for split in splits {
        let mut map = HashMap::with_capacity(25);
        for (i, line) in split.iter().enumerate() {
            for (j, s) in line.split_whitespace().enumerate() {
                map.insert((i, j), Some(s.parse().unwrap()));
            }
        }

        boards.push(Board { map });
    }

    Game { inputs, boards }
}

/// Draw the next number, remove it from the board and return the drawn number
fn step_game(game: &mut Game) -> i32 {
    let input = game.inputs.remove(0);

    for board in game.boards.iter_mut() {
        for (_, value) in board.map.iter_mut() {
            if Some(input) == *value {
                *value = None;
            }
        }
    }

    input
}

fn check_win(board: &Board) -> bool {
    (0..5).any(|i| {
        (0..5).all(|j| board.map.get(&(i, j)).unwrap().is_none())
            || (0..5).all(|j| board.map.get(&(j, i)).unwrap().is_none())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = r#"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"#;

    #[test]
    fn correctly_parses_inputs() {
        let game = parse_input(GIVEN_INPUT.lines());

        assert_eq!(game.inputs.first(), Some(&7));
        assert_eq!(game.inputs.last(), Some(&1));

        assert_eq!(game.boards.len(), 3);

        assert_eq!(game.boards[0].map.get(&(0, 0)).unwrap(), &Some(22));
        assert_eq!(game.boards[0].map.get(&(4, 4)).unwrap(), &Some(19));
        assert_eq!(game.boards[1].map.get(&(0, 0)).unwrap(), &Some(3));
        assert_eq!(game.boards[1].map.get(&(4, 4)).unwrap(), &Some(6));
        assert_eq!(game.boards[2].map.get(&(0, 0)).unwrap(), &Some(14));
        assert_eq!(game.boards[2].map.get(&(4, 4)).unwrap(), &Some(7));
    }

    #[test]
    fn steps_game() {
        let mut game = parse_input(GIVEN_INPUT.lines());

        let input = step_game(&mut game);

        assert_eq!(input, 7);
        assert_eq!(game.boards[0].map.get(&(2, 4)).unwrap(), &None);
        assert_eq!(game.boards[2].map.get(&(4, 4)).unwrap(), &None);
    }

    #[test]
    fn check_win_test() {
        let mut game = parse_input(GIVEN_INPUT.lines());

        assert!(!check_win(&game.boards[0]));

        for i in 0..5 {
            game.boards[0].map.insert((0, i), None);
        }

        assert!(check_win(&game.boards[0]));
    }

    #[test]
    fn check_given_input() {
        let game = parse_input(GIVEN_INPUT.lines());
        let answer = calculate(game);
        assert_eq!(answer, 4512);
    }

    #[test]
    fn check_given_input2() {
        let game = parse_input(GIVEN_INPUT.lines());
        let answer = calculate_2(game);
        assert_eq!(answer, 1924);
    }
}
