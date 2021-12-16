use pathfinding::prelude::dijkstra;

use crate::input_const;

pub fn solution1() -> u64 {
    Grid::from_str(input_const!("15")).calc1()
}

pub fn solution2() -> u64 {
    Grid::from_str(input_const!("15")).expand().calc1()
}

struct Grid {
    size: usize,
    points: Vec<u64>,

}

fn wrap(i: u64) -> u64 {
    if i > 9 {
        i - 9
    } else {
        i
    }
}

impl Grid {

    fn expand(self) -> Self {
        let old_size = self.size;
        let new_size = old_size * 5;
        let mut grid = Grid {
            size: new_size,
            points: vec![0; new_size * new_size]
        };
        for a in 0..5 {
            for b in 0..5 {
               for x in 0..old_size {
                   for y in 0..old_size {
                       let shift = a + b;
                       let value = self.get(x, y) + shift;
                       *grid.get_mut(x + old_size * a as usize, y + old_size * b as usize) = wrap(value);
                   }
               } 
            }
        }

        grid
    }

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
        }
    }

    fn get(&self, x: usize, y: usize) -> u64 {
        self.points[x + y * self.size]
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u64 {
        self.points.get_mut(x + y * self.size).unwrap()
    }

    fn calc1(&self) -> u64 {
        let (_, n) = dijkstra(&(0usize, 0usize), |&(x, y)| {
            let mut v = vec![];
            if x > 0 {
                v.push((x - 1, y));
            }
            if y > 0 {
                v.push((x, y - 1));
            }
            if x < self.size - 1 {
                v.push((x + 1, y));
            }
            if y < self.size - 1 {
                v.push((x, y + 1));
            }
            v.into_iter().map(|(x, y)| ((x, y), self.get(x, y)))
        }, |coord| coord == &(self.size - 1, self.size - 1)).unwrap();

        n
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

    #[test]
    fn part_2_given() {
        let grid = Grid::from_str(GIVEN_INPUT);
        let grid = grid.expand();
        assert_eq!(grid.calc1(), 315);
    }
}
