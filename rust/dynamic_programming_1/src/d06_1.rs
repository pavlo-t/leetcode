#![allow(dead_code)]
/// 152. Maximum Product Subarray
/// =============================
///
/// Given an integer array `nums`, find a contiguous non-empty subarray within the array that has
/// the largest product, and _return the product_.
///
/// It is __guaranteed__ that the answer will fit in a __32-bit__ integer.
///
/// A __subarray__ is a contiguous subsequence of the array.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 20_000` - `-10 <= nums[i] <= 10` - The product of any prefix or suffix
/// of nums is __guaranteed__ to fit in a __32-bit__ integer.
///
/// https://leetcode.com/problems/maximum-product-subarray/
struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut max = nums[0];
        let mut min = nums[0];
        for &n in nums.iter().skip(1) {
            if n == 0 {
                max = 0;
                min = 0;
            } else if n > 0 {
                max = n.max(max * n);
                min = n.min(min * n);
            } else {
                let prev_max = max;
                max = n.max(min * n);
                min = n.min(prev_max * n);
            }
            result = result.max(max);
        }
        result
    }
    pub fn max_product_dp_vector(nums: Vec<i32>) -> i32 {
        let mut dp = vec![(nums[0], nums[0]); nums.len()];
        for i in 1..nums.len() {
            let n = nums[i];
            if n == 0 {
                dp[i] = (0, 0);
            } else if n > 0 {
                dp[i].0 = n.max(dp[i - 1].0 * n);
                dp[i].1 = n.min(dp[i - 1].1 * n);
            } else if n < 0 {
                dp[i].0 = n.max(dp[i - 1].1 * n);
                dp[i].1 = n.min(dp[i - 1].0 * n);
            }
        }
        dp.into_iter().map(|(p, _)| p).max().unwrap()
    }
    pub fn max_product_brute_force_i(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        for l in 0..nums.len() {
            let mut curr = 1;
            for r in l..nums.len() {
                if nums[r] == 0 {
                    result = result.max(0);
                    break;
                } else {
                    curr *= nums[r];
                    result = result.max(curr);
                }
            }
        }
        result
    }

    /// from the comments in https://leetcode.com/problems/maximum-product-subarray/solution/
    pub fn max_product_two_passes(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        let mut curr = 1;
        for &n in nums.iter() {
            curr *= n;
            result = result.max(curr);
            if curr == 0 {
                curr = 1;
            }
        }
        curr = 1;
        for &n in nums.iter().rev() {
            curr *= n;
            result = result.max(curr);
            if curr == 0 {
                curr = 1;
            }
        }
        result
    }
    /// Approach 2: Dynamic Programming
    /// https://leetcode.com/problems/maximum-product-subarray/solution/
    pub fn max_product_dp(nums: Vec<i32>) -> i32 {
        let mut result = nums[0];
        let mut curr_max = 1;
        let mut curr_min = 1;
        for n in nums {
            let prev_max = curr_max;
            curr_max = n.max(curr_max * n).max(curr_min * n);
            curr_min = n.min(curr_min * n).min(prev_max * n);
            result = result.max(curr_max);
        }
        result
    }
    pub fn max_product_divide_and_conquer(nums: Vec<i32>) -> i32 {
        fn rec(ns: &[i32]) -> i32 {
            match ns.len() {
                0 => i32::MIN,
                1 => ns[0],
                n => {
                    let m = n / 2;
                    let (l, r) = (&ns[..m], &ns[m + 1..]);
                    let bp = if ns[m] == 0 {
                        0
                    } else {
                        let bmm = |(max, min, mut curr): (i32, i32, i32), &n| {
                            curr *= n;
                            (max.max(curr), min.min(curr), curr)
                        };
                        let z = (1, 1, 1);
                        let (lmax, lmin, _) = l.iter().rev().take_while(|&&n| n != 0).fold(z, bmm);
                        let (rmax, rmin, _) = r.iter().take_while(|&&n| n != 0).fold(z, bmm);
                        let mut bp = i32::MIN;
                        for bl in [1, lmax, lmin] {
                            for br in [1, rmax, rmin] {
                                bp = bp.max(ns[m] * bl * br);
                            }
                        }
                        bp
                    };
                    rec(l).max(bp).max(rec(r))
                }
            }
        }
        rec(&nums)
    }
    pub fn max_product_brute_force_improved(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        for l in 0..nums.len() {
            let mut curr = 1;
            for r in l..nums.len() {
                curr *= nums[r];
                result = result.max(curr);
            }
        }
        result
    }
    pub fn max_product_brute_force(nums: Vec<i32>) -> i32 {
        let mut result = i32::MIN;
        for l in 0..nums.len() {
            for r in l..nums.len() {
                let curr = nums[l..=r].iter().product();
                result = result.max(curr);
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p2p3m2p4() {
        let nums = vec![2, 3, -2, 4];
        assert_eq!(Solution::max_product(nums), 6);
        // Explanation: [2,3] has the largest product 6.
    }
    #[test]
    fn m2p0m1() {
        let nums = vec![-2, 0, -1];
        assert_eq!(Solution::max_product(nums), 0);
        // Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
    }
    #[test]
    fn p0m2m1() {
        let nums = vec![0, -2, -1];
        assert_eq!(Solution::max_product(nums), 2);
    }
    #[test]
    fn m1() {
        let nums = vec![-1];
        assert_eq!(Solution::max_product(nums), -1);
    }
    #[rustfmt::skip] #[test] fn p0() { assert_eq!(Solution::max_product(vec![0]), 0); }
    #[rustfmt::skip] #[test] fn p0p0() { assert_eq!(Solution::max_product(vec![0, 0]), 0); }

    #[test]
    fn ns_20000xm1() {
        let nums = vec![-1; 20000];
        assert_eq!(Solution::max_product(nums), 1);
    }
    #[test]
    fn ns_20000x0() {
        let nums = vec![0; 20000];
        assert_eq!(Solution::max_product(nums), 0);
    }
    #[test]
    fn ns_9xm2_0_repeat_2000() {
        let mut nums = vec![-2; 20000];
        for i in (1..=2000).map(|i| i * 10 - 1) {
            nums[i] = 0;
        }
        assert_eq!(Solution::max_product(nums), 256);
    }
}
