mod fast;

use crate::input_const;

fn inverse(s: char) -> char {
    match s {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        s => unreachable!(s),
    }
}

fn is_opening(s: char) -> bool {
    match s {
        '{' | '[' | '(' | '<' => true,
        '}' | ']' | ')' | '>' => false,
        s => unreachable!(s),
    }
}

fn score(s: char) -> u64 {
    match s {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        s => unreachable!(s),
    }
}

fn score_for_line(s: &str) -> Option<u64> {
    let mut v = vec![];

    for c in s.chars() {
        if is_opening(c) {
            v.push(c);
        } else if inverse(v.pop().unwrap()) != c {
            return Some(score(c));
        }
    }

    None
}

fn calculate1(s: &'static str) -> u64 {
    s.lines().filter_map(score_for_line).sum()
}

pub fn solution1() -> u64 {
    calculate1(input_const!("10"))
}

fn fix_line(s: &str) -> impl Iterator<Item = char> {
    let mut v = vec![];
    for c in s.chars() {
        if is_opening(c) {
            v.push(c);
        } else {
            v.pop().unwrap();
        }
    }

    v.into_iter().map(inverse).rev()
}

pub fn solution2() -> u64 {
    calculate2(input_const!("10"))
}

fn calculate2(s: &'static str) -> u64 {
    let mut scores: Vec<_> = s
        .lines()
        .filter(|s| !s.is_empty())
        .filter(|s| is_incomplete(s))
        .map(score_for_line2)
        .collect();

    scores.sort();
    let middle = (scores.len() - 1) / 2;
    scores[middle]
    
}

fn is_incomplete(s: &str) -> bool {
    score_for_line(s).is_none()
}

fn score_for_line2(s: &str) -> u64 {
    let mut i = 0;
    for c in fix_line(s) {
        i *= 5;
        i += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            s => unreachable!(s),
        };
    }
    i
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = r#"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]"#;

    #[test]
    fn test_given_input() {
        assert_eq!(calculate1(GIVEN_INPUT), 26397);
    }

    #[test]
    fn check_fix_line() {
        assert_eq!(
            fix_line("<{([{{}}[<[[[<>{}]]]>[]]").collect::<Vec<_>>(),
            vec![']', ')', '}', '>']
        );
    }

    #[test]
    fn test_given_input2() {
        assert_eq!(calculate2(GIVEN_INPUT), 288957);
    }
}
