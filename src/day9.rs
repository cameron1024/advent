use std::collections::HashSet;

use crate::input_const;

struct Grid {
    height: usize,
    width: usize,
    elements: Vec<u8>,
}

impl Grid {
    fn from_str(s: impl AsRef<str>) -> Self {
        let s = s.as_ref();
        let lines: Vec<_> = s.lines().filter(|s| !s.is_empty()).collect();
        let height = lines.len();
        let width = lines[0].len();
        let elements = s
            .replace("\n", "")
            .split("")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        Grid {
            height,
            width,
            elements,
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        assert!(x < self.width, "getting {},{}", x, y);
        assert!(y < self.height, "getting {},{}", x, y);
        self.elements[x + (y * self.width)]
    }

    fn neighbor_coords(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut v = vec![];

        if x > 0 {
            v.push((x - 1, y));
        }

        if x < self.width - 1 {
            v.push((x + 1, y));
        }

        if y > 0 {
            v.push((x, y - 1));
        }

        if y < self.height - 1 {
            v.push((x, y + 1));
        }

        v
    }

    fn neighbors(&self, x: usize, y: usize) -> Vec<u8> {
        self.neighbor_coords(x, y)
            .iter()
            .map(|(x, y)| self.get(*x, *y))
            .collect()
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        let i = self.get(x, y);
        self.neighbors(x, y).iter().all(|n| *n > i)
    }

    fn low_points(&self) -> impl IntoIterator<Item = (usize, usize)> + '_ {
        (0..self.width)
            .flat_map(|x| (0..self.height).map(move |y| (x, y)))
            .filter(|(x, y)| self.is_low_point(*x, *y))
    }

    fn risk_levels_for_low_points(&self) -> u64 {
        self.low_points()
            .into_iter()
            .map(|(x, y)| self.get(x, y) as u64 + 1)
            .sum()
    }

    fn basins(&self) -> Vec<HashSet<(usize, usize)>> {
        let low_points = self.low_points();
        let mut results = vec![];
        for (x, y) in low_points {
            let mut basin = HashSet::from_iter([(x, y)]);
            while self.expand_basin(&mut basin) {}

            results.push(basin)
        }

        results
    }

    fn expand_basin(&self, basin: &mut HashSet<(usize, usize)>) -> bool {
        let possibles: HashSet<(usize, usize)> = basin
            .iter()
            .flat_map(|(x, y)| self.neighbor_coords(*x, *y))
            .filter(|(x, y)| self.get(*x, *y) != 9)
            .collect();
        let len = basin.len();
        basin.extend(possibles);
        basin.len() != len
    }
}

pub fn solution1() -> u64 {
    Grid::from_str(input_const!("9")).risk_levels_for_low_points()
}

pub fn solution2() -> u64 {
    calculate2(input_const!("9"))
}


pub fn calculate2(s: impl AsRef<str>) -> u64 {
    let grid = Grid::from_str(s);
    let mut basins: Vec<_> = grid
        .basins()
        .into_iter()
        .map(|basin| basin.len() as u64)
        .collect();
    basins.sort();
    basins.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = r#"2199943210
3987894921
9856789892
8767896789
9899965678"#;

    #[test]
    fn parse_grid() {
        let grid = Grid::from_str(GIVEN_INPUT);
        assert_eq!(grid.height, 5);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.get(0, 0), 2);
        assert_eq!(grid.get(1, 1), 9);
        assert_eq!(grid.get(9, 4), 8);
    }

    #[test]
    fn get_neighbors() {
        let grid = Grid::from_str(GIVEN_INPUT);
        assert_eq!(grid.neighbors(0, 0), vec![1, 3]);
        assert_eq!(grid.neighbors(1, 1), vec![3, 8, 1, 8]);
    }

    #[test]
    fn is_low_point() {
        let grid = Grid::from_str(GIVEN_INPUT);
        assert_eq!(grid.is_low_point(0, 0), false);
        assert_eq!(grid.is_low_point(1, 1), false);
        assert_eq!(grid.is_low_point(1, 0), true);
        assert_eq!(grid.is_low_point(9, 0), true);
        assert_eq!(grid.is_low_point(2, 2), true);
        assert_eq!(grid.is_low_point(6, 4), true);
    }

    #[test]
    fn given_input() {
        let grid = Grid::from_str(GIVEN_INPUT);
        assert_eq!(grid.risk_levels_for_low_points(), 15);

        assert_eq!(calculate2(GIVEN_INPUT), 1134)
    }
}
