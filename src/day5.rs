use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::input_const;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn all_points(&self) -> Vec<Point> {
        if self.start.x == self.end.x {
            let x = self.start.x;
            let y1 = self.start.y;
            let y2 = self.end.y;

            (y1..=y2).map(|y| Point { x, y }).collect()
        } else if self.start.y == self.end.y {
            let y = self.start.y;
            let x1 = self.start.x;
            let x2 = self.end.x;

            (x1..=x2).map(|x| Point { x, y }).collect()
        } else {
            vec![]
        }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    points: Vec<AtomicUsize>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            points: {
                let mut points = Vec::with_capacity(width * height);
                for _ in 0..(width * height) {
                    points.push(AtomicUsize::new(0))
                }
                points
            },
        }
    }

    fn get(&self, x: usize, y: usize) -> &AtomicUsize {
        &self.points[x + (y * self.width)]
    }

    fn apply_point(&self, Point { x, y }: Point) {
        let atomic = self.get(x, y);
        atomic.fetch_add(1, Ordering::Relaxed);
    }

    fn apply_lines(&self, lines: impl IntoParallelIterator<Item = Line>) {
        let points = lines.into_par_iter().flat_map(|line| line.all_points());
        points.for_each(|point| self.apply_point(point));
    }

    fn count_greater_than_one(&self) -> usize {
        self.points
            .par_iter()
            .filter(|i| i.load(Ordering::Relaxed) > 1)
            .count()
    }
}

pub fn solution1() -> usize {
    calculate(input_const!("5"))
}

fn calculate(s: impl AsRef<str>) -> usize {
    let (lines, width, height) = get_lines_and_max_dimensions(s.as_ref());
    let grid = Grid::new(width, height);
    grid.apply_lines(lines);
    println!("{:#?}", grid);
    grid.count_greater_than_one()
        
}

fn get_lines_and_max_dimensions(s: impl AsRef<str>) -> (Vec<Line>, usize, usize) {
    let lines = s.as_ref().lines().map(parse_line).collect::<Vec<_>>();

    let points = lines.iter().flat_map(|line| [line.start, line.end]);
    let max_width = points.clone().map(|p| p.x).max().unwrap();
    let max_height = points.map(|p| p.y).max().unwrap();

    (lines, max_width, max_height)
}

fn parse_line(s: impl AsRef<str>) -> Line {
    let mut parts = s.as_ref().split(" ");
    let start = parts.next().unwrap();
    parts.next().unwrap();
    let end = parts.next().unwrap();

    let start = parse_point(start);
    let end = parse_point(end);

    Line { start, end }
}

#[inline(always)]
fn parse_point(s: impl AsRef<str>) -> Point {
    let mut parts = s.as_ref().split(",");
    let x = parts.next().unwrap().parse().unwrap();
    let y = parts.next().unwrap().parse().unwrap();
    Point { x, y }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse() {
        let line = parse_line("3,4 -> 5,6");
        assert_eq!(
            line,
            Line {
                start: Point { x: 3, y: 4 },
                end: Point { x: 5, y: 6 },
            }
        )
    }

    #[test]
    fn grid_new() {
        let grid = Grid::new(5, 5);
        for x in 0..5 {
            for y in 0..5 {
                assert_eq!(grid.get(x, y).load(Ordering::Relaxed), 0);
            }
        }
    }

    #[test]
    fn test_apply_line_and_count() {
        let grid = Grid::new(5, 5);
        let lines = (0..5).map(|x| Line {
            start: Point { x, y: 0 },
            end: Point { x, y: 4 },
        });

        grid.apply_lines(lines.collect::<Vec<_>>());

        for x in 0..5 {
            for y in 0..5 {
                assert_eq!(grid.get(x, y).load(Ordering::Relaxed), 1);
            }
        }

        let second_line = Line {
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 4 },
        };

        grid.apply_lines([second_line]);

        for y in 0..5 {
            assert_eq!(grid.get(0, y).load(Ordering::Relaxed), 2)
        }
    }

    fn test_points_from_line() {
        let bl = Point { x: 0, y: 0 };
        let br = Point { x: 2, y: 0 };
        let tl = Point { x: 0, y: 2 };
        let tr = Point { x: 2, y: 2 };

        assert_eq!(
            Line { start: br, end: bl }.all_points(),
            vec![br, Point { x: 1, y: 0 }, bl,]
        );

        assert_eq!(
            Line { start: br, end: tr }.all_points(),
            vec![br, Point { x: 0, y: 1 }, tr,]
        );

        assert_eq!(Line { start: br, end: tl }.all_points(), vec![]);
    }

    fn check_given_input() {
        let input = r#"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"#;

        let answer = calculate(input);
        assert_eq!(answer, 5);
        panic!()
    }
}
