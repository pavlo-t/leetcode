#![allow(dead_code)]

struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        flowerbed
            .iter()
            .fold((0, false), |acc, &current| match acc {
                (planted, _) if planted > n => acc,
                (planted, false) if current == 0 => (planted + 1, true),
                (planted, false) => (planted, true),
                (planted, _) if current == 1 => (planted - 1, true),
                (planted, _) => (planted, false),
            }).0 >= n
    }

    pub fn can_place_flowers_fold1(flowerbed: Vec<i32>, n: i32) -> bool {
        flowerbed
            .iter()
            .fold((0, 0), |(planted, previous), &current| {
                if planted > n {
                    (planted, previous)
                } else if previous == 0 {
                    (if current == 0 { planted + 1 } else { planted }, 1)
                } else if current == 1 {
                    (planted - 1, 1)
                } else {
                    (planted, 0)
                }
            }).0 >= n
    }

    pub fn can_place_flowers_iterative(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut planted = 0;
        let mut i = 0;

        while i < flowerbed.len() {
            if flowerbed[i] == 0 {
                if i == 0 || flowerbed[i - 1] == 0 {
                    if i == flowerbed.len() - 1 || flowerbed[i + 1] == 0 {
                        i += 1;
                        planted += 1;
                        if planted >= n {
                            return true;
                        }
                    }
                }
            };
            i += 1;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_10001_1_is_t() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        assert!(Solution::can_place_flowers(flowerbed, 1));
    }

    #[test]
    fn example2_10001_2_is_f() {
        let flowerbed = vec![1, 0, 0, 0, 1];
        assert!(!Solution::can_place_flowers(flowerbed, 2));
    }

    #[test]
    fn test_00000_3_is_t() {
        let flowerbed = vec![0, 0, 0, 0, 0];
        assert!(Solution::can_place_flowers(flowerbed, 3));
    }

    #[test]
    fn test_100001_2_is_f() {
        let flowerbed = vec![1, 0, 0, 0, 0, 1];
        assert!(!Solution::can_place_flowers(flowerbed, 2));
    }

    #[test]
    fn test_00_1_is_t() {
        let flowerbed = vec![0, 0];
        assert!(Solution::can_place_flowers(flowerbed, 1));
    }

    #[test]
    fn test_00_2_is_f() {
        let flowerbed = vec![0, 0];
        assert!(!Solution::can_place_flowers(flowerbed, 2));
    }

    #[test]
    fn test_01_1_is_f() {
        let flowerbed = vec![0, 1];
        assert!(!Solution::can_place_flowers(flowerbed, 1));
    }

    #[test]
    fn test_10_1_is_f() {
        let flowerbed = vec![1, 0];
        assert!(!Solution::can_place_flowers(flowerbed, 1));
    }

    #[test]
    fn test_20k0s_1_is_t() {
        let flowerbed = vec![0; 20000];
        assert!(Solution::can_place_flowers(flowerbed, 1));
    }

    #[test]
    fn test_20k0s_10000_is_t() {
        let flowerbed = vec![0; 20000];
        assert!(Solution::can_place_flowers(flowerbed, 10000));
    }

    #[test]
    fn test_19999_0s_10000_is_t() {
        let flowerbed = vec![0; 19999];
        assert!(Solution::can_place_flowers(flowerbed, 10000));
    }

    #[test]
    fn test_20k0s_10001_is_f() {
        let flowerbed = vec![0; 20000];
        assert!(!Solution::can_place_flowers(flowerbed, 10001));
    }
}