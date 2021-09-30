#![allow(dead_code)]
/// Valid Triangle Number
/// =====================
///
/// Given an integer array `nums`, return _the number of triplets chosen from the array that can make triangles
/// if we take them as side lengths of a triangle_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 1000`
/// - `0 <= nums[i] <= 1000`
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/610/week-3-july-15th-july-21st/3815/
struct Solution;
impl Solution {
    /// https://leetcode.com/problems/valid-triangle-number/solution/
    pub fn triangle_number(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            0
        } else {
            nums.sort_unstable();
            let mut result = 0;
            for i in 0..nums.len() - 2 {
                let a = nums[i];
                let mut k = i + 2;
                for j in i + 1..nums.len() - 1 {
                    let b = nums[j];
                    let ab = a + b;
                    while k < nums.len() && ab > nums[k] {
                        k += 1;
                    }
                    result += k - j - 1;
                }
            }
            result as i32
        }
    }

    /// ~5 seconds for tests; about 3 times faster than brute force; passed in leetcode
    pub fn triangle_number_optimized_my_bad_still_passes(mut nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            0
        } else {
            nums.sort_unstable();
            let mut result = 0;
            for i in 0..nums.len() - 2 {
                let a = nums[i];
                for j in i + 1..nums.len() - 1 {
                    let b = nums[j];
                    let ab = a + b;
                    let mut k = j + 1;
                    while k < nums.len() && ab > nums[k] {
                        k += 1;
                    }
                    result += k - j - 1;
                }
            }
            result as i32
        }
    }

    /// ~15 seconds for tests; TLE exception in leetcode
    pub fn triangle_number_brute_force(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            0
        } else {
            let mut result = 0;
            for i in 0..nums.len() - 2 {
                for j in i + 1..nums.len() - 1 {
                    for k in j + 1..nums.len() {
                        let (a, b, c) = (nums[i], nums[j], nums[k]);
                        if a < b + c && b < a + c && c < a + b {
                            result += 1;
                        }
                    }
                }
            }
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_2_2_3_4_produces_3() {
        let nums = vec![2, 2, 3, 4];
        assert_eq!(Solution::triangle_number(nums), 3);
        // Explanation: Valid combinations are:
        // 2,3,4 (using the first 2)
        // 2,3,4 (using the second 2)
        // 2,2,3
    }
    #[test]
    fn n_4_2_3_4_produces_4() {
        let nums = vec![4, 2, 3, 4];
        assert_eq!(Solution::triangle_number(nums), 4);
        // 4 2 3; 4 2 4; 4 3 4; 2 3 4
    }

    #[test]
    fn n_1_produces_0() {
        assert_eq!(Solution::triangle_number(vec![1]), 0);
    }
    #[test]
    fn n_1x2_produces_0() {
        assert_eq!(Solution::triangle_number(vec![1, 1]), 0);
    }

    #[test]
    fn n_1x3_produces_1() {
        assert_eq!(Solution::triangle_number(vec![1; 3]), 1);
    }
    #[test]
    fn n_1x4_produces_4() {
        assert_eq!(Solution::triangle_number(vec![1; 4]), 4);
    }
    #[test]
    fn n_1x5_produces_10() {
        assert_eq!(Solution::triangle_number(vec![1; 5]), 10);
    }
    #[test]
    fn n_1x6_produces_20() {
        assert_eq!(Solution::triangle_number(vec![1; 6]), 20);
    }
    #[test]
    fn n_1x7_produces_35() {
        assert_eq!(Solution::triangle_number(vec![1; 7]), 35);
    }
    #[test]
    fn n_1x8_produces_56() {
        assert_eq!(Solution::triangle_number(vec![1; 8]), 56);
    }
    #[test]
    fn n_1x9_produces_84() {
        assert_eq!(Solution::triangle_number(vec![1; 9]), 84);
    }
    #[test]
    fn n_1x10_produces_120() {
        assert_eq!(Solution::triangle_number(vec![1; 10]), 120);
    }

    mod performance {
        use super::*;

        #[test]
        fn n_1x100_produces_161_700() {
            assert_eq!(Solution::triangle_number(vec![1; 100]), 161_700);
        }
        #[test]
        fn n_1x1000_produces_166_167_000() {
            assert_eq!(Solution::triangle_number(vec![1; 1000]), 166_167_000);
        }
    }
}
