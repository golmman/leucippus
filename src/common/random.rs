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

    pub fn pick_element<'a, T>(&'a mut self, list: &'a Vec<T>) -> Option<&T> {
        if list.is_empty() {
            return None;
        }

        Some(&list[self.next() as usize % list.len()])
    }

    /// see https://en.wikipedia.org/wiki/Fisher%E2%80%93Yates_shuffle#The_modern_algorithm
    pub fn shuffle<T>(&mut self, list: &mut [T]) {
        let n = list.len() as i32; // prevents substraction with overflow error
        for i in 0..n - 1 {
            let start = i as u32;
            let end = n as u32;
            let j = self.next_range(start..end);
            list.swap(i as usize, j as usize);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_does_not_shuffle_a_list_with_no_numbers() {
        let mut random = Random::from_seed(7);
        let mut list: Vec<i32> = vec![];

        random.shuffle(&mut list);

        assert_eq!(list, vec![]);
    }

    #[test]
    fn it_does_not_shuffle_a_list_with_1_number() {
        let mut random = Random::from_seed(7);
        let mut list = vec![1];

        random.shuffle(&mut list);

        assert_eq!(list, vec![1]);
    }

    #[test]
    fn it_shuffles_a_list_of_2_numbers() {
        let mut random = Random::from_seed(7);
        let mut list = vec![1, 2];

        random.shuffle(&mut list);

        assert_eq!(list, vec![2, 1]);
    }

    #[test]
    fn it_shuffles_a_list_of_numbers() {
        let mut random = Random::from_seed(7);
        let mut list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        random.shuffle(&mut list);

        assert_eq!(list, vec![8, 1, 9, 5, 6, 10, 3, 2, 7, 4]);
    }

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

    #[test]
    fn it_picks_random_elements_from_a_list() {
        let mut random = Random::from_seed(10);
        let list = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(*random.pick_element(&list).unwrap(), 7);
        assert_eq!(*random.pick_element(&list).unwrap(), 5);
        assert_eq!(*random.pick_element(&list).unwrap(), 3);
        assert_eq!(*random.pick_element(&list).unwrap(), 3);
        assert_eq!(*random.pick_element(&list).unwrap(), 4);
        assert_eq!(*random.pick_element(&list).unwrap(), 8);
        assert_eq!(*random.pick_element(&list).unwrap(), 8);
        assert_eq!(*random.pick_element(&list).unwrap(), 6);
        assert_eq!(*random.pick_element(&list).unwrap(), 1);
    }
}
