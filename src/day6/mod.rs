mod fast;

use num_bigint::BigUint;

use crate::input_const;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct Fish(u8);

impl Fish {
    fn step(&mut self) -> Option<Fish> {
        match self.0 {
            0 => {
                self.0 = 6;
                Some(Fish(8))
            }
            _ => {
                self.0 -= 1;
                None
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(transparent)]
struct FishList(Vec<Fish>);

impl FishList {
    
    fn from_str(s: impl AsRef<str>) -> Self {
        let fishes = s.as_ref().split(",").map(|s| Fish(s.trim().parse().unwrap()));
        Self(fishes.collect())
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn step(&mut self) {
        let new_fish = self
            .0
            .iter_mut()
            .map(Fish::step)
            .filter_map(|s| s)
            .collect::<Vec<_>>();
        self.0.extend(new_fish);
    }
}

pub fn solution1() -> usize {
    calculate(input_const!("6"), 80)
}
pub fn solution2() -> BigUint {
    fast::calculate(input_const!("6"), 256)
}

fn calculate(s: impl AsRef<str>, days: usize) -> usize {
    let mut fishlist = FishList::from_str(s);
    for _ in 0..days {
        fishlist.step();
    }

    fishlist.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const GIVEN_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn parse_real_input() {
        let fishlist = FishList::from_str(input_const!("6"));
        assert_eq!(fishlist.0[0], Fish(1));
    }

    #[test]
    fn parse_given_input() {
        let fishlist = FishList::from_str(GIVEN_INPUT);
        assert_eq!(fishlist.0, vec![
            Fish(3),
            Fish(4),
            Fish(3),
            Fish(1),
            Fish(2),
        ]);
    }

    #[test]
    fn given_input() {
        assert_eq!(calculate(GIVEN_INPUT, 80), 5934);
    }


}
