#![allow(dead_code)]
/// Minimum Swaps to Group All 1's Together
/// =======================================
///
/// Given a binary array `data`, return the minimum number of swaps required to group all `1`’s
/// present in the array together in __any place__ in the array.
///
/// __Constraints:__
///
/// - `1 <= data.length <= 100_000`
/// - `data[i]` is `0` or `1`.
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/595/week-3-april-15th-april-21st/3708/
struct Solution;
impl Solution {
    pub fn min_swaps(data: Vec<i32>) -> i32 {
        let ones = data
            .iter()
            .fold(0, |rsf, &i| if i == 1 { rsf + 1 } else { rsf });
        let zeros = (0..ones).fold(0, |rsf, i| if data[i] == 0 { rsf + 1 } else { rsf });
        (ones..data.len())
            .fold((zeros, zeros), |(rsf, zeros), i| {
                match (data[i - ones], data[i]) {
                    (0, 1) => (rsf.min(zeros - 1), zeros - 1),
                    (1, 0) => (rsf, zeros + 1),
                    _ => (rsf, zeros),
                }
            })
            .0
    }

    pub fn min_swaps_v1(data: Vec<i32>) -> i32 {
        let ones = data
            .iter()
            .fold(0, |rsf, &i| if i == 1 { rsf + 1 } else { rsf });
        let mut result = (0..ones).fold(0, |rsf, i| if data[i] == 0 { rsf + 1 } else { rsf });
        let mut zeros = result;
        for r in ones..data.len() {
            let l = r - ones + 1;
            if data[r] != data[l - 1] {
                if data[r] == 0 {
                    zeros += 1;
                } else {
                    zeros -= 1;
                    result = result.min(zeros);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_data_10101_produces_1() {
        let data = vec![1, 0, 1, 0, 1];
        assert_eq!(Solution::min_swaps(data), 1);
        // Explanation:
        // There are 3 ways to group all 1's together:
        // [1,1,1,0,0] using 1 swap.
        // [0,1,1,1,0] using 2 swaps.
        // [0,0,1,1,1] using 1 swap.
        // The minimum is 1.
    }
    #[test]
    fn example2_data_00010_produces_0() {
        let data = vec![0, 0, 0, 1, 0];
        assert_eq!(Solution::min_swaps(data), 0);
        // Explanation: Since there is only one 1 in the array, no swaps needed.
    }
    #[test]
    fn example3_data10101001101_produces_3() {
        let data = vec![1, 0, 1, 0, 1, 0, 0, 1, 1, 0, 1];
        assert_eq!(Solution::min_swaps(data), 3);
        // Explanation:
        // One possible solution that uses 3 swaps is [0,0,0,0,0,1,1,1,1,1,1].
    }
    #[test]
    fn example4_data_produces_8() {
        let data = vec![
            1, 0, 1, 0, 1, 0, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0,
            1, 1, 1, 1, 0, 0, 1,
        ];
        assert_eq!(Solution::min_swaps(data), 8);
    }

    mod performance {
        use super::*;

        #[test]
        fn data_99_999_zeros_1_one_produces_0() {
            let mut data = vec![0; 99999];
            data.push(1);
            assert_eq!(Solution::min_swaps(data), 0);
        }
        #[test]
        fn data_99_998_zeros_2_ones_produces_1() {
            let mut data = vec![0; 100_000];
            data[0] = 1;
            data[99_999] = 1;
            assert_eq!(Solution::min_swaps(data), 1);
        }
        #[test]
        fn data_100_000_ones_produces_0() {
            let data = vec![1; 100_000];
            assert_eq!(Solution::min_swaps(data), 0);
        }
    }
}
