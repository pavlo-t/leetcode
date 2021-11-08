#![allow(dead_code)]
/// 152. Maximum Product Subarray
/// =============================
///
/// Given an integer array `nums`,
/// find a contiguous non-empty subarray within the array that has the largest product,
/// and return _the product_.
///
/// It is __guaranteed__ that the answer will fit in a __32-bit__ integer.
///
/// A __subarray__ is a contiguous subsequence of the array.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 20_000`
/// - `-10 <= nums[i] <= 10`
/// - The product of any prefix or suffix of nums is __guaranteed__ to fit in a __32-bit__ integer.
///
/// https://leetcode.com/problems/maximum-product-subarray/
struct Solution;
impl Solution {
    /// 22:54-22:59
    pub fn max_product(nums: Vec<i32>) -> i32 {
        println!("max_product({:?})", nums);
        let (mut max, mut min) = (i32::MIN, i32::MAX);
        let mut result = i32::MIN;
        for &n in nums.iter() {
            if n == 0 {
                max = 0;
                min = 0;
            } else {
                if n < 0 {
                    std::mem::swap(&mut max, &mut min);
                }
                max = n.max(n.saturating_mul(max));
                min = n.min(n.saturating_mul(min));
            };
            result = result.max(max);
        }
        result
    }
    /// 22:53-22:54
    pub fn max_product_dp_vec_2(nums: Vec<i32>) -> i32 {
        println!("max_product({:?})", nums);
        let mut dp = vec![(i32::MIN, i32::MAX); nums.len() + 1];
        let mut result = i32::MIN;
        for i in (0..nums.len()).rev() {
            let n = nums[i];
            dp[i] = if n == 0 {
                (0, 0)
            } else {
                let (max, min) = dp[i + 1];
                if n < 0 {
                    (n.max(n.saturating_mul(min)), n.min(n.saturating_mul(max)))
                } else {
                    (n.max(n.saturating_mul(max)), n.min(n.saturating_mul(min)))
                }
            };
            result = result.max(dp[i].0);
        }
        result
    }
    /// 22:48-22:53
    pub fn max_product_dp_vec_1(nums: Vec<i32>) -> i32 {
        println!("max_product({:?})", nums);
        let mut dp = vec![(i32::MIN, i32::MAX); nums.len() + 1];
        for i in (0..nums.len()).rev() {
            let n = nums[i];
            dp[i] = if n == 0 {
                (0, 0)
            } else {
                let (max, min) = dp[i + 1];
                if n < 0 {
                    (n.max(n.saturating_mul(min)), n.min(n.saturating_mul(max)))
                } else {
                    (n.max(n.saturating_mul(max)), n.min(n.saturating_mul(min)))
                }
            };
        }
        dp.into_iter().map(|(max, _)| max).max().unwrap()
    }
    /// 22:42-22:48
    pub fn max_product_rec_with_memo(nums: Vec<i32>) -> i32 {
        println!("max_product({:?})", nums);
        fn rec(i: usize, ns: &[i32], memo: &mut Vec<(i32, i32)>) -> (i32, i32) {
            if i == ns.len() {
                (i32::MIN, i32::MAX)
            } else if memo[i] != (i32::MIN, i32::MAX) {
                memo[i]
            } else {
                memo[i] = if ns[i] == 0 {
                    (0, 0)
                } else {
                    let (nmax, nmin) = rec(i + 1, ns, memo);
                    if ns[i] < 0 {
                        let max = ns[i].max(ns[i].saturating_mul(nmin));
                        let min = ns[i].min(ns[i].saturating_mul(nmax));
                        (max, min)
                    } else {
                        let max = ns[i].max(ns[i].saturating_mul(nmax));
                        let min = ns[i].min(ns[i].saturating_mul(nmin));
                        (max, min)
                    }
                };
                memo[i]
            }
        }
        let mut memo = vec![(i32::MIN, i32::MAX); nums.len()];
        (0..nums.len())
            .map(|i| rec(i, &nums, &mut memo).0)
            .max()
            .unwrap()
    }
    /// 22:20-22:42
    pub fn max_product_rec(nums: Vec<i32>) -> i32 {
        println!("max_product({:?})", nums);
        fn rec(i: usize, ns: &[i32]) -> (i32, i32) {
            if i == ns.len() {
                (i32::MIN, i32::MAX)
            } else if ns[i] == 0 {
                (0, 0)
            } else {
                let (nmax, nmin) = rec(i + 1, ns);
                if ns[i] < 0 {
                    let max = ns[i].max(ns[i].saturating_mul(nmin));
                    let min = ns[i].min(ns[i].saturating_mul(nmax));
                    (max, min)
                } else {
                    let max = ns[i].max(ns[i].saturating_mul(nmax));
                    let min = ns[i].min(ns[i].saturating_mul(nmin));
                    (max, min)
                }
            }
        }
        (0..nums.len()).map(|i| rec(i, &nums).0).max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_p1() {
        assert_eq!(Solution::max_product(vec![1]), 1);
    }
    #[test]
    fn n_0() {
        assert_eq!(Solution::max_product(vec![0]), 0);
    }
    #[test]
    fn n_m1() {
        assert_eq!(Solution::max_product(vec![-1]), -1);
    }
    #[test]
    fn n_00() {
        assert_eq!(Solution::max_product(vec![0, 0]), 0);
    }
    #[test]
    fn n_p2p3m2p4() {
        let n = vec![2, 3, -2, 4];
        assert_eq!(Solution::max_product(n), 6);
        // Explanation: [2,3] has the largest product 6.
    }
    #[test]
    fn n_p0p2p3m2p4() {
        let n = vec![0, 2, 3, -2, 4];
        assert_eq!(Solution::max_product(n), 6);
    }
    #[test]
    fn n_m2p0m1() {
        let n = vec![-2, 0, -1];
        assert_eq!(Solution::max_product(n), 0);
        // Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
    }
    #[test]
    fn n_m2p1m2() {
        let n = vec![-2, 1, -2];
        assert_eq!(Solution::max_product(n), 4);
    }

    /// If getting stack overflow:
    /// Add `RUST_MIN_STACK=67108864` to env:
    /// RUST_MIN_STACK=67108864 cargo test --lib d04_3
    #[test]
    fn n_20000x1() {
        let n = vec![1; 20000];
        assert_eq!(Solution::max_product(n), 1);
    }
}
