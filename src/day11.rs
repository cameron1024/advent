use std::collections::{HashMap, HashSet};

use crate::input_const;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid([u32; 100]);

const SIZE: usize = 10;
impl Grid {
    fn new(s: &'static str) -> Self {
        let mut grid = [0u32; SIZE * SIZE];
        let mut i = 0;
        for c in s
            .lines()
            .flat_map(str::chars)
            .filter(|s| !s.is_whitespace())
        {
            grid[i] = c as u32 - '0' as u32;
            i += 1;
        }

        Self(grid)
    }

    const fn get(&self, x: usize, y: usize) -> u32 {
        self.0[Self::index(x, y)]
    }

    const fn index(x: usize, y: usize) -> usize {
        x + (SIZE * y)
    }

    fn neighbors(x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as isize;
        let y = y as isize;
        let mut results = vec![];
        for x in (x - 1)..=(x + 1) {
            for y in (y - 1)..=(y + 1) {
                if x >= 0 && x <= 9 && y >= 0 && y <= 9 {
                    results.push((x as usize, y as usize));
                }
            }
        }
        results
            .into_iter()
            .filter(|coord| *coord != (x as usize, y as usize))
            .collect()
    }

    fn step(&mut self) -> usize {
        for i in self.0.iter_mut() {
            *i += 1;
        }

        let points = || (0..SIZE).flat_map(move |x| (0..SIZE).map(move |y| (x, y)));
        let mut flashed = HashSet::new();

        loop {
            let new_flashes: HashSet<_> = points()
                .filter(|(x, y)| self.get(*x, *y) > 9 && !flashed.contains(&(*x, *y)))
                .collect();
            for (x, y) in &new_flashes {
                for (x, y) in Self::neighbors(*x, *y) {
                    self.0[(Self::index(x, y))] += 1;
                }
            }
            if new_flashes.is_empty() {
                break;
            } else {
                flashed.extend(new_flashes);
            }
        }

        for (ref x, ref y) in &flashed {
            self.0[Self::index(*x, *y)] = 0;
        }

        flashed.len()
    }
}

pub fn solution1() -> usize {
    calculate1(input_const!("11"))
}

fn calculate1(s: &'static str) -> usize {
    let mut grid = Grid::new(s);
    (0..100).map(|_| grid.step()).sum()
}

pub fn solution2() -> usize {
    calculate2(input_const!("11"))
}

fn calculate2(s: &'static str) -> usize {
    let mut grid = Grid::new(s);
    for i in 1.. {
        if grid.step() == 100 {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = r#"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"#;

    #[test]
    fn test_new_grid_and_get() {
        let grid = Grid::new(GIVEN_INPUT);
        assert_eq!(grid.0[0], 5);
        assert_eq!(grid.0[99], 6);
        assert_eq!(grid.get(0, 0), 5);
        assert_eq!(grid.get(1, 0), 4);
        assert_eq!(grid.get(0, 1), 2);
        assert_eq!(grid.get(1, 1), 7);
    }

    #[test]
    fn test_step() {
        let mut grid = Grid::new(GIVEN_INPUT);
        let after = Grid::new(
            r#"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637"#,
        );
        grid.step();
        assert_eq!(grid, after)
    }

    #[test]
    fn test_given_input() {
        assert_eq!(calculate1(GIVEN_INPUT), 1656);
    }

    #[test]
    fn test_given_input2() {
        assert_eq!(calculate2(GIVEN_INPUT), 195);
    }
}
