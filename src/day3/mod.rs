use crate::input_lines;

fn input() -> impl Iterator<Item = String> {
    input_lines!()
}

pub fn solution1() -> i64 {
    let length = input().into_iter().next().unwrap().len();
    let (gamma, epsilon) = gamma_and_epsilon(length, input());

    let gamma = i64::from_str_radix(&gamma, 2).unwrap();
    let epsilon = i64::from_str_radix(&epsilon, 2).unwrap();

    gamma * epsilon
}

fn gamma_and_epsilon(
    length: usize,
    nums: impl IntoIterator<Item = impl AsRef<str>>,
) -> (String, String) {
    let mcbs = most_common_bits(length, nums);
    let lcbs = invert(&mcbs);
    (mcbs, lcbs)
}

fn most_common_bits(length: usize, nums: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    let mut zeroes = vec![0usize; length];
    let mut ones = vec![0usize; length];

    for num in nums {
        for (i, chr) in num.as_ref().chars().enumerate() {
            match chr {
                '0' => zeroes[i] += 1,
                '1' => ones[i] += 1,
                _ => unreachable!(),
            }
        }
    }

    let mut s = String::with_capacity(length);
    for i in 0..length {
        let more_zeroes = zeroes[i] > ones[i];
        s.push(if more_zeroes { '0' } else { '1' });
    }
    s
}

fn invert(s: impl AsRef<str>) -> String {
    let s = s.as_ref();
    let s = s.replace("0", "t");
    let s = s.replace("1", "0");
    s.replace("t", "1")
}

pub fn solution2() -> i64 {

    let (oxy, co2) = ratings(input());

    let oxy = i64::from_str_radix(&oxy, 2).unwrap();
    let co2 = i64::from_str_radix(&co2, 2).unwrap();

    oxy * co2
}

fn ratings(input: impl IntoIterator<Item = impl AsRef<str> + Clone>) -> (String, String) {
    let mut oxy_readings: Vec<_> = input.into_iter().collect();
    let mut co2_readings = oxy_readings.clone();

    let length = oxy_readings[0].as_ref().len();

    let mut current_index = 0;
    let nth = |s: &str, n: usize| s.chars().collect::<Vec<_>>()[n];

    while oxy_readings.len() != 1 || co2_readings.len() != 1 {
        if oxy_readings.len() > 1 {
            let mcbs = most_common_bits(length, &oxy_readings);
            let bit = nth(&mcbs, current_index);
            oxy_readings.retain(|s| nth(s.as_ref(), current_index) == bit);
        }

        if co2_readings.len() > 1 {
            let lcbs = invert(most_common_bits(length, &co2_readings));
            let bit = nth(&lcbs, current_index);
            co2_readings.retain(|s| nth(s.as_ref(), current_index) == bit);
        }

        current_index += 1;
    }

    (
        oxy_readings[0].as_ref().to_owned(),
        co2_readings[0].as_ref().to_owned(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_invert() {
        assert_eq!(invert(""), "");
        assert_eq!(invert("1"), "0");
        assert_eq!(invert("0"), "1");
        assert_eq!(invert("101010"), "010101");
        assert_eq!(invert("110110110"), "001001001");
    }

    #[test]
    fn given_input() {
        let mcbs = most_common_bits(5, test_input().lines());
        assert_eq!(mcbs, "10110");
    }

    #[test]
    fn test_ratings() {
        let (oxy, co2) = ratings(test_input().lines());

        assert_eq!(oxy, "10111");
        assert_eq!(co2, "01010");
    }
}
