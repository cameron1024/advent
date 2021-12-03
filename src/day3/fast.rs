fn input() -> (usize, impl IntoIterator<Item = u16>) {
    let length = super::input().next().unwrap().len();
    (length, super::input().map(|s| u16::from_str_radix(&s, 2).unwrap()))
}

pub fn solution1() -> u64 {
    let (length, nums) = input();
    let (gamma, epsilon) = gamma_and_epsilon(length, nums);

    gamma as u64 * epsilon as u64
}

fn gamma_and_epsilon(length: usize, nums: impl IntoIterator<Item = u16>) -> (u16, u16) {
    let mcbs = most_common_bits(length, nums);
    let lcbs = invert(mcbs);
    (mcbs, lcbs)
}

fn most_common_bits(length: usize, nums: impl IntoIterator<Item = u16>) -> u16 {
    let mut ones = vec![0usize; length];
    let nums: Vec<_> = nums.into_iter().collect();
    let nums_len = nums.len();

    for num in nums {
        for n in 0..length {
            if nth_bit(num, n) {
                ones[n] += 1;
            }
        }
    }

    let mut zeroes = vec![0usize; length];
    for (index, count) in ones.iter().enumerate() {
        zeroes[index] = nums_len - count;
    }

    let mut output = 0u16;
    for i in 0..length {
        let more_zeroes = zeroes[i] > ones[i];
        if !more_zeroes {
            output |= 1 << i;
        }
    }
    output
}

fn nth_bit(i: u16, n: usize) -> bool {
    i & (1 << n) != 0
}

fn invert(i: u16) -> u16 {
    !i
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use super::*;

    #[test]
    fn test_invert() {
        assert_eq!(invert(0b0101010101010101), 0b1010101010101010);
    }

    fn test_input() -> &'static str {
        r#"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"#
    }

    fn test_given_input() {
        let input = test_input().lines().map(|s| s.parse().unwrap());
        let mcbs = most_common_bits(5, input);
        assert_eq!(mcbs, 0b10110);
    }

    #[test]
    fn test_nth_bit() {
        assert_eq!(nth_bit(0b1111, 0), true);
        assert_eq!(nth_bit(0b1111, 1), true);
        assert_eq!(nth_bit(0b1111, 2), true);
        assert_eq!(nth_bit(0b1111, 3), true);
        assert_eq!(nth_bit(0b1111, 4), false);
        assert_eq!(nth_bit(0b1111, 5), false);
        assert_eq!(nth_bit(0b1111, 6), false);
        assert_eq!(nth_bit(0b1111, 7), false);
    }

    #[bench]
    fn slow_most_common_bits(b: &mut Bencher) {
        let nums = super::super::input();
        let nums = nums.into_iter().collect::<Vec<_>>();
        b.iter(|| {
            let bits = crate::day3::most_common_bits(
                black_box(nums.len()),
                black_box(nums.iter().map(|i| i.to_string())),
            );
            black_box(bits)
        })
    }

    #[bench]
    fn fast_most_common_bits(b: &mut Bencher) {
        let (length, nums) = input();
        let nums = nums.into_iter().collect::<Vec<_>>();
        b.iter(|| {
            let bits = most_common_bits(black_box(length), black_box(nums.clone()));
            black_box(bits)
        })
    }
}
