use std::collections::HashMap;

use crate::input_const;

#[derive(Debug, Clone)]
struct Entry {
    uniques: Vec<String>,
    output: Vec<String>,
}

impl Entry {
    fn from_str(s: impl AsRef<str>) -> Self {
        let mut parts = s.as_ref().split("|").map(str::trim);
        let uniques = parts.next().unwrap();
        let output = parts.next().unwrap();

        let uniques: Vec<String> = uniques.split(" ").map(ToOwned::to_owned).collect();
        let output: Vec<String> = output.split(" ").map(ToOwned::to_owned).collect();
        Self { uniques, output }
    }

    fn get_output(&self) -> u32 {
        let mapping = Mapping::from_uniques(self.uniques.clone());
        let mut output = 0;
        output += mapping.apply_to_digit(self.output[0].clone()) as u32 * 1000;
        output += mapping.apply_to_digit(self.output[1].clone()) as u32 * 100;
        output += mapping.apply_to_digit(self.output[2].clone()) as u32 * 10;
        output += mapping.apply_to_digit(self.output[3].clone()) as u32 * 1;
        output
    }
}

fn is_unique_len(s: impl AsRef<str>) -> bool {
    match s.as_ref().len() {
        2 | 3 | 4 | 7 => true,
        _ => false,
    }
}

pub fn solution1() -> usize {
    calculate1(input_const!("8"))
}

fn calculate1(s: impl AsRef<str>) -> usize {
    let lines = s.as_ref().lines();
    lines
        .flat_map(|line| Entry::from_str(line).output)
        .filter(|s| is_unique_len(s))
        .count()
}

pub fn solution2() -> u64 {
    calculate2(input_const!("8"))
}

fn calculate2(s: impl AsRef<str>) -> u64 {
    s.as_ref()
        .lines()
        .map(|s| {
            let entry = Entry::from_str(s);
            entry.get_output()
        })
        .sum::<u32>()
        .into()
}

const CHARS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];

// transpose:
// a: 8
// b: 6
// c: 8
// d: 7
// e: 4
// f: 9
// g: 7

#[derive(Debug)]
struct Mapping {
    // inner.insert('a', 'b')  means that the letter at real position 'a' is 'b'
    inner: HashMap<char, char>,
}

impl Mapping {
    fn from_uniques(uniques: impl IntoIterator<Item = String>) -> Self {
        let uniques: Vec<String> = uniques.into_iter().collect();
        let mut inner = HashMap::with_capacity(7);

        let one = uniques.iter().find(|s| s.len() == 2).unwrap().clone();
        let seven = uniques.iter().find(|s| s.len() == 3).unwrap().clone();
        let four = uniques.iter().find(|s| s.len() == 4).unwrap().clone();

        assert_eq!(one.len(), 2);
        assert_eq!(seven.len(), 3);
        assert_eq!(four.len(), 4);

        // 7 = 1 + a
        // a = 7 - 1

        let a = CHARS
            .iter()
            .filter(|c| !one.contains(&c.to_string()) && seven.contains(&c.to_string()))
            .next()
            .unwrap();

        inner.insert('a', *a);

        // 0, 6 and 9 are the only ones with 6 sections
        // only 9 is a superset of 4

        let zero_six_nine = uniques.iter().filter(|s| s.len() == 6).collect::<Vec<_>>();

        assert_eq!(zero_six_nine.len(), 3);

        let nine = zero_six_nine
            .iter()
            .find(|number| is_superset(number, &four))
            .unwrap();

        // 9 - 4 = a + g

        let g = nine
            .chars()
            .filter(|c| !four.contains(&c.to_string()))
            .find(|c| c != a)
            .unwrap();

        inner.insert('g', g);

        // now consider transpose

        let mut transpose = HashMap::new();

        for c in CHARS {
            transpose.insert(
                c,
                uniques
                    .iter()
                    .filter(|s| s.contains(&c.to_string()))
                    .count(),
            );
        }

        assert_eq!(
            {
                let mut vec = transpose.values().copied().collect::<Vec<_>>();
                vec.sort();
                vec
            },
            vec![4, 6, 7, 7, 8, 8, 9]
        );

        let b = *transpose.iter().find(|(_, v)| **v == 6).unwrap().0;
        let e = *transpose.iter().find(|(_, v)| **v == 4).unwrap().0;
        let f = *transpose.iter().find(|(_, v)| **v == 9).unwrap().0;

        inner.insert('b', b);
        inner.insert('e', e);
        inner.insert('f', f);

        let c = transpose
            .iter()
            .find(|(k, v)| **v == 8 && *k != a)
            .unwrap()
            .0;

        let d = transpose
            .iter()
            .find(|(k, v)| **v == 7 && **k != g)
            .unwrap()
            .0;

        inner.insert('c', *c);
        inner.insert('d', *d);

        Mapping { inner }
    }

    fn apply_to_digit(&self, digit: String) -> u8 {
        let mapped = digit
            .chars()
            .map(|c| self.inner.iter().find(|(_, v)| **v == c).unwrap());
        let normalized = {
            let mut v: Vec<String> = mapped.map(|c| c.0.to_string()).collect();
            v.sort();
            v.join("")
        };

        match normalized.as_str() {
            "abcefg" => 0,
            "cf" => 1,
            "acdeg" => 2,
            "acdfg" => 3,
            "bcdf" => 4,
            "abdfg" => 5,
            "abdefg" => 6,
            "acf" => 7,
            "abcdefg" => 8,
            "abcdfg" => 9,
            s => unreachable!(s),
        }
    }
}

fn is_superset(bigger: &str, smaller: &str) -> bool {
    let bigger_chars = bigger.chars().collect::<Vec<_>>();
    let mut smaller_chars = smaller.chars();
    smaller_chars.all(|c| bigger_chars.contains(&c))
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = r#"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"#;

    #[test]
    fn test_given_input() {
        assert_eq!(calculate1(GIVEN_INPUT), 26)
    }

    #[test]
    fn correctly_creates_mapping() {
        let entry = Entry::from_str(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        );
        let mapping = Mapping::from_uniques(entry.uniques);
        assert_eq!(
            mapping.inner,
            HashMap::from_iter([
                ('a', 'd'),
                ('b', 'e'),
                ('c', 'a'),
                ('d', 'f'),
                ('e', 'g'),
                ('f', 'b'),
                ('g', 'c'),
            ])
        );
    }

    #[test]
    fn given_input_2() {
        let input =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(calculate2(input), 5353);
    }
}
