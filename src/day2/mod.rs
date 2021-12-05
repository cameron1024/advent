use crate::input_lines;
use Instruction::*;

fn input() -> impl Iterator<Item = &'static str> {
    input_lines!("2")
}

pub fn solution() -> i64 {
   let mut p = Position::default();
   apply_all(&mut p, input().into_iter().map(map_line));
   p.depth * p.horizontal
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Instruction {
    Forward(i64),
    Up(i64),
    Down(i64),
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Default)]
struct Position {
    depth: i64,
    horizontal: i64,
    aim: i64,
}

fn apply_instruction(p: &mut Position, instruction: Instruction) {
    match instruction {
        Forward(i) => {
            p.horizontal += i;
            p.depth += i * p.aim;
        }
        Up(i) => p.aim -= i,
        Down(i) => p.aim += i,
    }
}

fn apply_all(p: &mut Position, i: impl IntoIterator<Item = Instruction>) {
    for instruction in i {
        apply_instruction(p, instruction)
    }
}

fn map_line(s: impl AsRef<str>) -> Instruction {
    let mut parts = s.as_ref().split_whitespace();
    let instruction = parts.next().unwrap();
    let i: i64 = parts.next().unwrap().parse().unwrap();
    match instruction {
        "forward" => Forward(i),
        "up" => Up(i),
        "down" => Down(i),
        _ => panic!("wtf?"),
    }
}



#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};

    use super::*;

    fn test_input() -> Vec<&'static str> {
        vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = input().map(map_line).collect::<Vec<_>>();
        assert_eq!(input.first(), Some(&Forward(2)));
        assert_eq!(input.last(), Some(&Forward(6)));
    }

    #[test]
    fn test_given_input() {
        let mut p = Position::default();
        apply_all(&mut p, test_input().iter().map(map_line));
        assert_eq!(p.horizontal, 15);
        assert_eq!(p.depth, 60);
    }

    #[test]
    fn test_map_line() {
        let parsed: Vec<_> = test_input().into_iter().map(map_line).collect();
        assert_eq!(
            parsed,
            vec![Forward(5), Down(5), Forward(8), Up(3), Down(8), Forward(2),]
        );
    }

    #[test]
    fn test_apply_instruction() {
        let mut pos = Position::default();

        apply_instruction(&mut pos, Forward(1));
        assert_eq!(pos, Position {
            depth: 0,
            horizontal: 1,
            aim: 0,
        });
        
        apply_instruction(&mut pos, Up(1));
        assert_eq!(pos, Position {
            depth: 0,
            horizontal: 1,
            aim: -1,
        });
        
        apply_instruction(&mut pos, Forward(10));
        assert_eq!(pos, Position {
            depth: -10,
            horizontal: 11,
            aim: -1,
        });
        
        apply_instruction(&mut pos, Down(10));
        assert_eq!(pos, Position {
            depth: -10,
            horizontal: 11,
            aim: 9,
        });
    }

    #[bench]
    fn bench_solution(b: &mut Bencher) {
        let input: Vec<_> = input().collect();
        b.iter(|| {
            let mut p = Position::default();
            let instructions = black_box(input.iter()).map(map_line);
            apply_all(&mut p, instructions);
            black_box(p)
        });
    }

}
