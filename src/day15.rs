use std::collections::HashMap;

use crate::input_const;

pub fn solution1() -> u64 {
    Grid::from_str(input_const!("16")).calc1()
}

struct Grid {
    size: usize,
    points: Vec<u64>,

    cache: HashMap<(isize, isize), u64>,
}

impl Grid {
    fn from_str(s: impl AsRef<str>) -> Self {
        let size = s.as_ref().lines().next().unwrap().len();
        let mut points = Vec::with_capacity(size * size);

        for line in s.as_ref().lines().filter(|s| !s.is_empty()) {
            for s in line.split("").filter(|s| !s.is_empty()) {
                points.push(s.parse().unwrap());
            }
        }

        assert_eq!(points.len(), size * size);
        assert!(points.iter().all(|i| *i > 0 && *i < 10));

        Self {
            size,
            points,
            cache: HashMap::new(),
        }
    }

    fn get(&self, x: usize, y: usize) -> u64 {
        self.points[x + y * self.size]
    }

    fn calc(&mut self, x: isize, y: isize) -> u64 {
        if self.cache.contains_key(&(x, y)) {
            *self.cache.get(&(x, y)).unwrap()
        } else {
            let result = if x == 0 && y == 0 {
                0
            } else if x == 0 {
                self.get(x as usize, y as usize) + self.calc(x, y - 1)
            } else if y == 0 {
                self.get(x as usize, y as usize) + self.calc(x - 1, y)
            } else {
                let above = self.calc(x - 1, y);
                let left = self.calc(x, y - 1);
                std::cmp::min(above, left) + self.get(x as usize, y as usize)
            };

            self.cache.insert((x, y), result);
            result
        }
    }

    fn calc1(mut self) -> u64 {
        self.calc((self.size - 1) as isize, (self.size - 1) as isize)
    }

    fn calculate1(mut self) -> u64 {
        self.points[0] = 0;
        let limit = self.size - 1;

        for y in (0..self.size).rev() {
            for x in (0..self.size).rev() {
                if x == limit && y < limit {
                    self.points[x + y * self.size] += self.get(x, y + 1);
                } else if x < limit && y == limit {
                    self.points[x + y * self.size] += self.get(x + 1, y);
                } else if x < limit && y < limit {
                    let bottom = self.get(x, y + 1);
                    let right = self.get(x + 1, y);
                    self.points[x + y * self.size] += std::cmp::min(bottom, right);
                }
            }
        }

        self.points[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = r#"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"#;

    #[test]
    fn given_input() {
        let grid = Grid::from_str(GIVEN_INPUT);
        assert_eq!(grid.size, 10);
        assert_eq!(grid.get(0, 0), 1);
        assert_eq!(grid.get(1, 1), 3);
        assert_eq!(grid.get(9, 9), 1);

        assert_eq!(grid.calc1(), 40);
    }

    #[test]
    fn parse_real_input() {
        let grid = Grid::from_str(input_const!("15"));
        assert_eq!(grid.size, 100);
        assert_eq!(grid.points.len(), 10000);
        assert_eq!(grid.get(0, 0), 2);
        assert_eq!(grid.get(1, 1), 1);
        assert_eq!(grid.get(99, 99), 9);
    }
}
