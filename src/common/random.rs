use std::ops::Range;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

// https://en.wikipedia.org/wiki/Linear_congruential_generator#Parameters_in_common_use
const A: u64 = 48271;
const C: u64 = 0;
const M: u64 = 2147483647;

pub struct Random {
    seed: u64,
}

impl Random {
    pub fn new() -> Self {
        Self {
            seed: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
                % M,
        }
    }

    pub fn from_seed(seed: u64) -> Self {
        Self { seed }
    }

    pub fn next(&mut self) -> u32 {
        self.seed = (A * self.seed + C) % M;
        self.seed as u32
    }

    pub fn next_range(&mut self, range: Range<u32>) -> u32 {
        range.start + self.next() % (range.end - range.start)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_generates_the_same_random_numbers_with_one_seed() {
        let mut random = Random::from_seed(7);
        assert_eq!(random.next(), 337897);
        assert_eq!(random.next(), 1278240558);
        assert_eq!(random.next(), 449829614);
        assert_eq!(random.next(), 518142577);
    }

    #[test]
    fn it_generates_the_same_random_numbers_with_another_seed() {
        let mut random = Random::from_seed(123456);
        assert_eq!(random.next(), 1664377282);
        assert_eq!(random.next(), 1645061505);
        assert_eq!(random.next(), 1261092736);
        assert_eq!(random.next(), 1636001594);
    }

    #[test]
    fn it_generates_the_same_random_numbers_with_seed_in_a_range() {
        let mut random = Random::from_seed(10);
        assert_eq!(random.next_range(2..4), 2);
        assert_eq!(random.next_range(2..4), 2);
        assert_eq!(random.next_range(2..4), 2);
        assert_eq!(random.next_range(2..4), 2);
        assert_eq!(random.next_range(2..4), 3);
        assert_eq!(random.next_range(2..4), 3);
        assert_eq!(random.next_range(2..4), 3);
        assert_eq!(random.next_range(2..4), 3);
        assert_eq!(random.next_range(2..4), 2);
    }
}
