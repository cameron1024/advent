use num_bigint::BigUint;

const AGE_LIMIT: usize = 9;

#[derive(Debug)]
struct FishList {
    ages: [BigUint; AGE_LIMIT],
}

impl FishList {
    fn from_str(s: impl AsRef<str>) -> Self {
        let fishes: Vec<BigUint> = s
            .as_ref()
            .split(",")
            .map(str::trim)
            .map(|s| s.parse().unwrap())
            .collect();
        let mut ages: [BigUint; AGE_LIMIT] = Default::default();
        for i in 0..AGE_LIMIT {
            ages[i] = fishes
                .iter()
                .filter(|n| **n == BigUint::from(i))
                .count()
                .try_into()
                .unwrap()
        }
        Self { ages }
    }

    fn step(&mut self) {
        let zeroes = self.ages[0].clone();
        for i in 1..AGE_LIMIT {
            self.ages[i - 1] = self.ages[i].clone();
        }
        self.ages[8] = zeroes.clone(); // zeroes make new children
        self.ages[6] += zeroes; // also become sixes
    }

    fn len(&self) -> BigUint {
        self.ages.iter().sum()
    }
}

pub fn calculate(s: impl AsRef<str>, days: usize) -> BigUint {
    let mut fishlist = FishList::from_str(s);
    for _ in 0..days {
        fishlist.step();
    }

    fishlist.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let mut list = FishList::from_str("3,4,3,1,2");
        assert_eq!(as_vec(&list), vec![0, 1, 1, 2, 1, 0, 0, 0, 0]);
        list.step();
        assert_eq!(as_vec(&list), vec![1, 1, 2, 1, 0, 0, 0, 0, 0]);
        list.step();
        assert_eq!(as_vec(&list), vec![1, 2, 1, 0, 0, 0, 1, 0, 1]);
        list.step();
        assert_eq!(as_vec(&list), vec![2, 1, 0, 0, 0, 1, 1, 1, 1]);
    }

    fn as_vec(f: &FishList) -> Vec<u128> {
        f.ages.iter().map(|i| i.try_into().unwrap()).collect()
    }

    #[test]
    fn test_given_example() {
        assert_eq!(calculate("3,4,3,1,2", 256), BigUint::from(26984457539u64));
    }
}
