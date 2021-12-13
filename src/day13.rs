use std::collections::HashSet;

use crate::input_const;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Paper {
    height: usize,
    width: usize,
    points: HashSet<Point>,
}

impl Paper {
    fn from_str(s: impl AsRef<str>) -> (Self, Vec<Fold>) {
        let mut points = HashSet::new();
        let mut folds = vec![];
        let mut parsing_points = true;

        for line in s.as_ref().lines() {
            if line.is_empty() {
                parsing_points = false;
            } else {
                if parsing_points {
                    let mut parts = line.split(",");
                    let point = Point {
                        x: parts.next().unwrap().parse().unwrap(),
                        y: parts.next().unwrap().parse().unwrap(),
                    };
                    points.insert(point);
                } else {
                    let words: Vec<_> = line.split(" ").collect();
                    let mut parts = words[2].split("=");
                    let xy = parts.next().unwrap();
                    let num = parts.next().unwrap().parse().unwrap();
                    let fold = match xy {
                        "x" => Fold::Vertical(num),
                        "y" => Fold::Horizontal(num),
                        s => unreachable!(s),
                    };
                    folds.push(fold);
                }
            }
        }

        let width = points.iter().map(|p| p.x).max().unwrap_or(0) + 1;
        let height = points.iter().map(|p| p.y).max().unwrap_or(0) + 1;
        (
            Self {
                height,
                width,
                points,
            },
            folds,
        )
    }

    fn apply_fold(&mut self, fold: Fold) {
        match fold {
            Fold::Horizontal(n) => {
                let mapped_points: HashSet<_> = self
                    .points
                    .iter()
                    .filter(|p| p.y > n)
                    .map(|Point { x, y }| {
                        let distance = y - n;
                        Point {
                            x: *x,
                            y: n - distance,
                        }
                    })
                    .collect();
                self.points.extend(mapped_points);
                self.points.retain(|p| p.y <= n);
                self.height = n;
            }
            Fold::Vertical(n) => {
                let mapped_points: HashSet<_> = self
                    .points
                    .iter()
                    .filter(|p| p.x > n)
                    .map(|Point { x, y }| {
                        let distance = x - n;
                        Point {
                            x: n - distance,
                            y: *y,
                        }
                    })
                    .collect();
                self.points.extend(mapped_points);
                self.points.retain(|p| p.x <= n);
                self.width = n;
            }
        }
    }

    fn dots(&self) -> usize {
        self.points.len()
    }

    fn pretty_print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!(
                    "{}",
                    if self.points.contains(&Point { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!("");
        }
    }
}

fn calculate1(s: impl AsRef<str>) -> usize {
    let (mut paper, mut fold) = Paper::from_str(s);
    paper.apply_fold(fold.remove(0));
    paper.dots()
}

pub fn solution1() -> usize {
    calculate1(input_const!("13"))
}

pub fn print_solution2() {
    let (mut paper, mut folds) = Paper::from_str(input_const!("13"));
    for fold in folds {
        paper.apply_fold(fold);
    }
    paper.pretty_print();
}



#[cfg(test)]
mod tests {

    use super::*;

    const GIVEN_INPUT: &str = r#"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"#;

    #[test]
    fn parse_paper() {
        let (paper, folds) = Paper::from_str(GIVEN_INPUT);
        assert_eq!(paper.points.len(), 18);
        assert!(paper.points.contains(&Point { x: 6, y: 10 }));
        assert!(paper.points.contains(&Point { x: 0, y: 14 }));
        assert_eq!(folds, vec![Fold::Horizontal(7), Fold::Vertical(5)]);
        assert_eq!(paper.height, 15);
        assert_eq!(paper.width, 11);
    }

    #[test]
    fn given_example() {
        let (mut paper, mut folds) = Paper::from_str(GIVEN_INPUT);
        let fold = folds.remove(0);
        paper.apply_fold(fold);
        assert_eq!(paper.dots(), 17);
        paper.apply_fold(folds.remove(0));
        paper.pretty_print();
        panic!();
    }
}
