mod fast;


use std::collections::{HashMap, HashSet};


use crate::input_const;

struct Question {
    template: String,
    pairs: HashMap<(char, char), char>,
}


pub fn solution2() -> usize {
    fast::calculate(input_const!("14"), 40)
}



pub fn solution1() -> usize {
    calculate1(input_const!("14"))
}

fn calculate1(s: impl AsRef<str>) -> usize {
    let mut q = Question::new(s);

    for _ in 0..10 {
        q.step();
    }

    let all_chars: HashSet<_> = q.template.chars().collect();
    let char_counts: HashMap<_, _> = all_chars
        .iter()
        .map(|c| (*c, q.template.chars().filter(|c1| c1 == c).count()))
        .collect();

    let max_char = *char_counts.values().max().unwrap();
    let min_char = *char_counts.values().min().unwrap();

    max_char - min_char
}

impl Question {
    fn new(s: impl AsRef<str>) -> Self {
        let mut lines = s.as_ref().lines();
        let template = lines.next().unwrap().to_string();

        lines.next().unwrap();

        let pairs: HashMap<_, _> = lines
            .map(|line| {
                let mut chars = line.chars().filter(char::is_ascii_uppercase);
                (
                    (chars.next().unwrap(), chars.next().unwrap()),
                    chars.next().unwrap(),
                )
            })
            .collect();

        Self { template, pairs }
    }

    fn step(&mut self) {
        let bytes = self.template.as_bytes();
        let insertions = bytes
            .windows(2)
            .map(|c| self.pairs.get(&(c[0] as char, c[1] as char)).unwrap());
        let first = bytes[0];

        let mut mapped_bytes: Vec<u8> = bytes
            .iter()
            .skip(1)
            .zip(insertions)
            .flat_map(|(a, b)| [*b as u8, *a])
            .collect();

        mapped_bytes.insert(0, first);

        self.template = String::from_utf8(mapped_bytes).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = r#"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"#;

    #[test]
    fn test_given_input() {
        let mut q = Question::new(GIVEN_INPUT);
        assert_eq!(q.template, "NNCB");
        assert_eq!(q.pairs.get(&('C', 'H')).unwrap(), &'B');
        assert_eq!(q.pairs.get(&('C', 'N')).unwrap(), &'C');

        q.step();
        assert_eq!(q.template, "NCNBCHB");
        q.step();
        assert_eq!(q.template, "NBCCNBBBCBHCB");
        q.step();
        assert_eq!(q.template, "NBBBCNCCNBBNBNBBCHBHHBCHB");
        q.step();
        assert_eq!(
            q.template,
            "NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB"
        );

        assert_eq!(calculate1(GIVEN_INPUT), 1588);
    }
}
