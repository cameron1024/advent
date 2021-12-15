use std::collections::HashMap;

use super::Question;

struct State {
    map: HashMap<(char, char), char>,
    cache: HashMap<(char, char, usize), HashMap<char, usize>>,
}

impl State {
    fn score_for_pair(&mut self, c1: char, c2: char, depth: usize) -> HashMap<char, usize> {
        if let Some(map) = self.cache.get(&(c1, c2, depth)) {
            map.clone()
        } else {
            let middle = *self.map.get(&(c1, c2)).unwrap();
            let map = if depth == 1 {
                create_map(c1, c2, middle)
            } else {
                let first = self.score_for_pair(c1, middle, depth - 1);
                let second = self.score_for_pair(middle, c2, depth - 1);
                
                let mut map = merge(first, second);
                *map.get_mut(&middle).unwrap() -= 1;
                map
            };
            self.cache.insert((c1, c2, depth), map.clone());
            map
        }
    }
}

fn create_map(c1: char, c2: char, middle: char) -> HashMap<char, usize> {
    let mut map = HashMap::with_capacity(3);
    map.entry(c1).and_modify(|v| *v += 1).or_insert(1);
    map.entry(c2).and_modify(|v| *v += 1).or_insert(1);
    map.entry(middle).and_modify(|v| *v += 1).or_insert(1);
    map
}

fn merge(mut m1: HashMap<char, usize>, m2: HashMap<char, usize>) -> HashMap<char, usize> {
    for (c, count) in m2 {
        m1.entry(c)
            .and_modify(|value| *value += count)
            .or_insert(count);
    }

    m1
}

pub fn calculate(s: impl AsRef<str>, depth: usize) -> usize {
    let Question { pairs, template } = Question::new(s);
    let mut state = State {
        map: pairs,
        cache: HashMap::new(),
    };

    let mut chars: Vec<_> = template.chars().collect();
    let mut map = chars
        .windows(2)
        .map(|cs| state.score_for_pair(cs[0], cs[1], depth))
        .fold(HashMap::new(), merge);

    chars.remove(chars.len() - 1);
    chars.remove(0);

    for c in chars {
        *map.get_mut(&c).unwrap() -= 1;
    }

    let max = *map.values().max().unwrap();
    let min = *map.values().min().unwrap();

    max - min
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
    fn trivial() {
        assert_eq!(calculate(GIVEN_INPUT, 10), 1588);
    }

    #[test]
    #[ignore]
    fn fast_method_given_input() {
        dbg!(calculate(GIVEN_INPUT, 39));
        dbg!(calculate(GIVEN_INPUT, 41));
        assert_eq!(calculate(GIVEN_INPUT, 40), 2188189693529);
    }
}
