use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf, slice::Windows,
};

pub fn solution1() -> usize {
    count_increases(&input())
}

pub fn solution2() -> usize {
   let summed = summed_list(&input());
    count_increases(&summed)
}

fn count_increases(i: &[i32]) -> usize {
    i.windows(2).filter(|w| w[0] < w[1]).count()
}

fn summed_list(i: &[i32]) -> Vec<i32> {
    i.windows(3).map(|w| w[0] + w[1] + w[2]).collect()
}

fn input() -> Vec<i32> {
    let path = PathBuf::from(file!()).parent().unwrap().join("input");
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .map(|s| s.unwrap().parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn summed_list_test() {
        let x = summed_list(&[1, 2, 3]);
        assert_eq!(x, vec![6]);

        assert_eq!(summed_list(&[]), []);
        assert_eq!(summed_list(&[1]), []);
        assert_eq!(summed_list(&[1, 2]), []);

    }

    #[test]
    fn parses_input() {
        let i = input();
        assert_eq!(i.first(), Some(&173));
        assert_eq!(i.last(), Some(&9380));
    }

    #[test]
    fn given_example() {
        assert_eq!(
            count_increases(&[199, 200, 208, 210, 200, 207, 240, 269, 260, 263]),
            7
        );
    }

    #[test]
    fn edge_cases() {
        assert_eq!(count_increases(&[]), 0);
        assert_eq!(count_increases(&[1]), 0);
        assert_eq!(count_increases(&[1, 0]), 0);
    }
}
