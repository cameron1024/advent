use std::ops::RangeInclusive;

use crate::input_const;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Crabs(Vec<i64>);

impl Crabs {
    fn get_range(&self) -> RangeInclusive<i64> {
        let min = *self.0.iter().min().unwrap();
        let max = *self.0.iter().max().unwrap();
        min..=max
    }

    fn total_distance(&self, x: i64, q2: bool) -> i64 {
        self.0
            .iter()
            .map(|s| (s - x).abs())
            .map(distance_fn(q2))
            .sum()
    }
}

fn distance_fn(q2: bool) -> fn(i64) -> i64 {
    if q2 {
        triangle
    } else {
        id
    }
}
fn triangle(x: i64) -> i64 {
    x * (x + 1) / 2
}

fn id(x: i64) -> i64 {
    x
}

pub fn solution1() -> i64 {
    calculate(input_const!("7"), false)
}

pub fn solution2() -> i64 {
    calculate(input_const!("7"), true)
}

fn calculate(s: impl AsRef<str>, q2: bool) -> i64 {
    let crabs = parse_input(s);
    crabs
        .get_range()
        .map(|x| crabs.total_distance(x, q2))
        .min()
        .unwrap()
}

fn parse_input(s: impl AsRef<str>) -> Crabs {
    let nums = s.as_ref().split(",").map(|s| s.trim().parse().unwrap());
    Crabs(nums.collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn parses_input() {
        assert_eq!(
            parse_input(GIVEN_INPUT),
            Crabs(vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14])
        )
    }

    #[test]
    fn given_example() {
        assert_eq!(calculate(GIVEN_INPUT, false), 37);
        assert_eq!(calculate(GIVEN_INPUT, true), 168);
    }
}
