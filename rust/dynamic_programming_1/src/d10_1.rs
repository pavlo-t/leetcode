#![allow(dead_code)]
/// 413. Arithmetic Slices
/// ======================
///
/// An integer array is called arithmetic if it consists of __at least three elements__
/// and if the difference between any two consecutive elements is the same.
///
/// For example, `[1,3,5,7,9]`, `[7,7,7,7]`, and `[3,-1,-5,-9]` are arithmetic sequences.
///
/// Given an integer array `nums`, return _the number of arithmetic __subarrays__ of `nums`_.
///
/// A __subarray__ is a contiguous subsequence of the array.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 5000`
/// - `-1000 <= nums[i] <= 1000`
///
/// https://leetcode.com/problems/arithmetic-slices/
struct Solution;
impl Solution {
    /// Approach 5: Constant Space Dynamic Programming
    /// https://leetcode.com/problems/arithmetic-slices/solution/
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut prev = 0;
        for i in 2..nums.len() {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                prev += 1;
                sum += prev;
            } else {
                prev = 0;
            }
        }
        sum
    }
    /// Approach 4: Dynamic Programming
    /// https://leetcode.com/problems/arithmetic-slices/solution/
    pub fn number_of_arithmetic_slices_dp_vec(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut dp = vec![0; nums.len()];
        for i in 2..nums.len() {
            if nums[i] - nums[i - 1] == nums[i - 1] - nums[i - 2] {
                dp[i] = 1 + dp[i - 1];
                sum += dp[i];
            }
        }
        sum
    }
    /// Approach 3: Using Recursion
    /// https://leetcode.com/problems/arithmetic-slices/solution/
    pub fn number_of_arithmetic_slices_recursion(nums: Vec<i32>) -> i32 {
        println!("number_of_arithmetic_slices({:?})", nums);
        fn slices(i: usize, ns: &[i32], sum: &mut i32) -> i32 {
            if i < 2 {
                0
            } else {
                let mut ap = 0;
                if ns[i] - ns[i - 1] == ns[i - 1] - ns[i - 2] {
                    ap = 1 + slices(i - 1, ns, sum);
                    *sum += ap;
                } else {
                    slices(i - 1, ns, sum);
                }
                ap
            }
        }
        let mut sum = 0;
        slices(nums.len() - 1, &nums, &mut sum);
        sum
    }

    pub fn number_of_arithmetic_slices_my_dp_vec(nums: Vec<i32>) -> i32 {
        println!("number_of_arithmetic_slices({:?})", nums);
        let n = nums.len();
        if n < 3 {
            0
        } else {
            let mut dp = vec![false; n - 2];
            let mut result = 0;
            for i in 0..n - 2 {
                dp[i] = nums[i] - nums[i + 1] == nums[i + 1] - nums[i + 2];
                result += dp[i] as i32;
            }
            for l in 4..=n {
                for i in 0..=n - l {
                    dp[i] = dp[i] && dp[i + 1];
                    result += dp[i] as i32;
                }
            }
            result
        }
    }
    pub fn number_of_arithmetic_slices_my_dp_vec_vec(nums: Vec<i32>) -> i32 {
        println!("number_of_arithmetic_slices({:?})", nums);
        let n = nums.len();
        if n < 3 {
            0
        } else {
            let mut dp = vec![vec![false; n - 2]; n + 1];
            let mut result = 0;
            for i in 0..n - 2 {
                dp[3][i] = nums[i] - nums[i + 1] == nums[i + 1] - nums[i + 2];
                result += dp[3][i] as i32;
            }
            for l in 4..=n {
                for i in 0..=n - l {
                    dp[l][i] = dp[l - 1][i] && dp[l - 1][i + 1];
                    result += dp[l][i] as i32;
                }
            }
            result
        }
    }
    pub fn number_of_arithmetic_slices_my_iter(nums: Vec<i32>) -> i32 {
        println!("number_of_arithmetic_slices({:?})", nums);
        let n = nums.len();
        let mut result = 0;
        for l in 3..=n {
            for i in 0..=n - l {
                let mut is_arithmetic = true;
                let d = nums[i + 1] - nums[i];
                for j in i + 2..i + l {
                    if nums[j] - nums[j - 1] != d {
                        is_arithmetic = false;
                        break;
                    }
                }
                result += is_arithmetic as i32;
            }
        }
        result
    }
    pub fn number_of_arithmetic_slices_my_rec(nums: Vec<i32>) -> i32 {
        println!("number_of_arithmetic_slices({:?})", nums);
        fn is_arithmetic(i: usize, l: usize, ns: &[i32]) -> bool {
            let d = ns[i + 1] - ns[i];
            for j in i + 2..i + l {
                if ns[j] - ns[j - 1] != d {
                    return false;
                }
            }
            true
        }
        fn rec(l: usize, ns: &[i32]) -> i32 {
            if l < 3 {
                0
            } else {
                let mut result = 0;
                for i in 0..=ns.len() - l {
                    result += is_arithmetic(i, l, ns) as i32;
                }
                result += rec(l - 1, ns);
                result
            }
        }
        rec(nums.len(), &nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ns_1_2_3_4() {
        let n = vec![1, 2, 3, 4];
        // [1,2,3][2,3,4]
        // [1,2,3,4]
        assert_eq!(Solution::number_of_arithmetic_slices(n), 3);
        // Explanation: We have 3 arithmetic slices in nums: [1,2,3], [2,3,4] and [1,2,3,4] itself.
    }
    #[test]
    fn ns_1_2_3_4_5() {
        let n = vec![1, 2, 3, 4, 5];
        // [1,2,3][2,3,4][3,4,5]
        // [1,2,3,4][2,3,4,5]
        // [1,2,3,4,5]
        assert_eq!(Solution::number_of_arithmetic_slices(n), 6);
    }
    #[test]
    fn ns_1_2_3_4_5_6() {
        let n = vec![1, 2, 3, 4, 5, 6];
        // 123 234 345 456 1234 2345 3456 12345 23456 123456
        assert_eq!(Solution::number_of_arithmetic_slices(n), 10);
    }
    #[test]
    fn ns_1_1_3_4_5_6() {
        let n = vec![1, 1, 3, 4, 5, 6];
        assert_eq!(Solution::number_of_arithmetic_slices(n), 3);
    }
    #[test]
    fn ns_1() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1]), 0);
    }
    #[test]
    fn ns_1_2() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2]), 0);
    }
    #[test]
    fn ns_1_2_3() {
        assert_eq!(Solution::number_of_arithmetic_slices(vec![1, 2, 3]), 1);
    }
    #[test]
    fn ns_1_2_3_4_5_9_9_9_9_9() {
        let n = vec![1, 2, 3, 4, 5, 9, 9, 9, 9, 9];
        assert_eq!(Solution::number_of_arithmetic_slices(n), 12);
    }

    //#[ignore]
    #[test]
    fn ns_5000x1() {
        let n = vec![1; 5000];
        assert_eq!(Solution::number_of_arithmetic_slices(n), 12_492_501);
    }
}
