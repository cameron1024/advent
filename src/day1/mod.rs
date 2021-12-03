
use crate::input_line_nums;

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
    input_line_nums!()
}

#[cfg(test)]
mod tests {

    use test::{black_box, Bencher};

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

    #[bench]
    fn bench_count_increases(b: &mut Bencher) {
        let data = input();
        b.iter(|| black_box(count_increases(black_box(&data))));
    }

    #[bench]
    fn bench_summed_list(b: &mut Bencher) {
        let data = input();
        b.iter(|| black_box(summed_list(black_box(&data))));
    }

    #[bench]
    fn bench_both(b: &mut Bencher) {
        let data = input();
        b.iter(|| black_box(count_increases(&summed_list(black_box(&data)))));
    }
}
