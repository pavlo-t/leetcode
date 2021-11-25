#![allow(dead_code)]
/// 494. Target Sum
/// ===============
///
/// You are given an integer array `nums` and an integer `target`.
///
/// You want to build an __expression__ out of nums
/// by adding one of the symbols `'+'` and `'-'` before each integer in nums
/// and then concatenate all the integers.
///
/// For example, if `nums = [2, 1]`, you can add a `'+'` before `2` and a `'-'` before `1`
/// and concatenate them to build the expression `"+2-1"`.
///
/// Return the number of different __expressions__ that you can build, which evaluates to `target`.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 20`
/// - `0 <= nums[i] <= 1000`
/// - `0 <= sum(nums[i]) <= 1000`
/// - `-1000 <= target <= 1000`
///
/// https://leetcode.com/problems/target-sum/
struct Solution;
impl Solution {
    /// 22:42-22:47
    pub fn find_target_sum_ways_rec(nums: Vec<i32>, target: i32) -> i32 {
        println!("find_target_sum_ways({:?}, {})", nums, target);
        fn rec(i: usize, t: i32, ns: &[i32]) -> i32 {
            if i == ns.len() {
                (t == 0) as i32
            } else {
                let n = ns[i];
                rec(i + 1, t - n, ns) + rec(i + 1, t + n, ns)
            }
        }
        rec(0, target, &nums)
    }
    /// 22:47-23:01
    pub fn find_target_sum_ways_rec_with_memo(nums: Vec<i32>, target: i32) -> i32 {
        println!("find_target_sum_ways({:?}, {})", nums, target);
        fn rec(i: usize, t: i32, ns: &[i32], memo: &mut Vec<Vec<i32>>) -> i32 {
            let tk = (t + 1000) as usize;
            if memo[i][tk] != -1 {
                memo[i][tk]
            } else {
                memo[i][tk] = if i == ns.len() {
                    (t == 0) as i32
                } else {
                    let n = ns[i];
                    rec(i + 1, t - n, ns, memo) + rec(i + 1, t + n, ns, memo)
                };
                memo[i][tk]
            }
        }
        let n = nums.len();
        let mut memo = vec![vec![-1; (target + 2002) as usize]; n + 1];
        rec(0, target, &nums, &mut memo)
    }
    /// 23:01-23:26
    pub fn find_target_sum_ways_dp_vec_vec(nums: Vec<i32>, target: i32) -> i32 {
        println!("find_target_sum_ways({:?}, {})", nums, target);
        let z = nums.iter().sum::<i32>() as usize;
        let n = nums.len();
        let target = target.abs() as usize;
        let mt = z * 2 + 1;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; mt]; n + 1];
        dp[0][z + target] = 1;
        for i in 1..=n {
            let num = nums[i - 1] as usize;
            for t in num..mt - num {
                dp[i][t] = dp[i - 1][t - num].saturating_add(dp[i - 1][t + num]);
            }
        }
        dp[n][z]
    }
    /// 23:26
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        println!("find_target_sum_ways({:?}, {})", nums, target);
        let z = nums.iter().sum::<i32>() as usize;
        let n = nums.len();
        let target = target.abs() as usize;
        if target > z {
            0
        } else {
            let mt = z * 2 + 1;
            let mut curr = vec![0i32; mt];
            let mut prev = vec![0i32; mt];
            curr[z + target] = 1;
            for i in 0..n {
                std::mem::swap(&mut prev, &mut curr);
                let num = nums[i] as usize;
                for t in num..mt - num {
                    curr[t] = prev[t - num].saturating_add(prev[t + num]);
                }
            }
            curr[z]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n1_t1() {
        let c = vec![1];
        let t = 1;
        let e = 1;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
    }
    #[test]
    fn n11111_t3() {
        let c = vec![1, 1, 1, 1, 1];
        let t = 3;
        let e = 5;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
        // Explanation: There are 5 ways to assign symbols to make the sum of nums be target 3.
        // -1 + 1 + 1 + 1 + 1 = 3
        // +1 - 1 + 1 + 1 + 1 = 3
        // +1 + 1 - 1 + 1 + 1 = 3
        // +1 + 1 + 1 - 1 + 1 = 3
        // +1 + 1 + 1 + 1 - 1 = 3
    }
    #[test]
    fn n11111_tm3() {
        let c = vec![1, 1, 1, 1, 1];
        let t = -3;
        let e = 5;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
    }
    #[test]
    fn n11111_t500() {
        let c = vec![1, 1, 1, 1, 1];
        let t = 500;
        let e = 0;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
    }
    #[test]
    fn n11111_t5() {
        let c = vec![1, 1, 1, 1, 1];
        let t = 5;
        let e = 1;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
    }

    #[test]
    fn n1to20_t10() {
        let c = (1..=20).collect();
        let t = 10;
        let e = 15029;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
    }
    #[test]
    fn n1to20_t2() {
        let c = vec![1; 20];
        let t = 2;
        let e = 167_960;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
    }
    #[test]
    fn n50repeat20_t500() {
        let c = vec![50; 20];
        let t = 500;
        let e = 15504;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
    }
    #[test]
    fn n1000repeat20_t2000() {
        let c = vec![1000; 20];
        let t = 2000;
        let e = 167_960;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
    }

    //#[ignore]
    #[test]
    fn n1repeat100_t50() {
        let c = vec![1; 100];
        let t = 90;
        let e = 75_287_520;
        assert_eq!(Solution::find_target_sum_ways(c, t), e);
    }
}
