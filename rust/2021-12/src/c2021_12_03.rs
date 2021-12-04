#![allow(dead_code)]
/// 152. Maximum Product Subarray
/// =============================
///
/// https://leetcode.com/problems/maximum-product-subarray/
struct Solution;
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        println!("max_product({:?})", nums);
        let (mut max, mut min) = (nums[0], nums[0]);
        let mut result = nums[0];
        for num in nums.into_iter().skip(1) {
            if num == 0 {
                max = 0;
                min = 0;
            } else {
                if num < 0 {
                    std::mem::swap(&mut max, &mut min);
                }
                max = num.max(num * max);
                min = num.min(num * min);
            }
            result = result.max(max);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_n0() { assert_eq!(Solution::max_product(vec![ 0]),  0); }
    #[rustfmt::skip] #[test] fn n_p1() { assert_eq!(Solution::max_product(vec![ 1]),  1); }
    #[rustfmt::skip] #[test] fn n_m1() { assert_eq!(Solution::max_product(vec![-1]), -1); }

    #[rustfmt::skip] #[test] fn n_p2p3() { assert_eq!(Solution::max_product(vec![ 2, 3]),  6); }
    #[rustfmt::skip] #[test] fn n_p3p2() { assert_eq!(Solution::max_product(vec![ 3, 2]),  6); }
    #[rustfmt::skip] #[test] fn n_m2m3() { assert_eq!(Solution::max_product(vec![-2,-3]),  6); }
    #[rustfmt::skip] #[test] fn n_m3m2() { assert_eq!(Solution::max_product(vec![-3,-2]),  6); }
    #[rustfmt::skip] #[test] fn n_m2p3() { assert_eq!(Solution::max_product(vec![-2, 3]),  3); }
    #[rustfmt::skip] #[test] fn n_m3p2() { assert_eq!(Solution::max_product(vec![-3, 2]),  2); }
    #[rustfmt::skip] #[test] fn n_p2m3() { assert_eq!(Solution::max_product(vec![ 2,-3]),  2); }
    #[rustfmt::skip] #[test] fn n_p3m2() { assert_eq!(Solution::max_product(vec![ 3,-2]),  3); }

    #[rustfmt::skip] #[test] fn n_p2p3n0p5() { assert_eq!(Solution::max_product(vec![ 2, 3, 0, 5]),  6); }
    #[rustfmt::skip] #[test] fn n_p2p3n0p7() { assert_eq!(Solution::max_product(vec![ 2, 3, 0, 7]),  7); }

    #[test]
    fn n_p2p3m2p4() {
        let n = vec![2, 3, -2, 4];
        assert_eq!(Solution::max_product(n), 6);
        // Explanation: [2,3] has the largest product 6.
    }
    #[test]
    fn n_m2n0m1() {
        let n = vec![-2, 0, -1];
        assert_eq!(Solution::max_product(n), 0);
        // Explanation: The result cannot be 2, because [-2,-1] is not a subarray.
    }

    fn check(n: Vec<i32>, e: i32) {
        assert_eq!(Solution::max_product(n), e);
    }
    fn check_large_stack(n: Vec<i32>, e: i32) {
        const STACK_SIZE: usize = 2usize.pow(27);
        let child = std::thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(move || check(n, e))
            .unwrap();
        child.join().unwrap();
    }

    #[test]
    fn n_0repeat20000() {
        let n = vec![0; 20000];
        assert_eq!(Solution::max_product(n), 0);
    }
    #[test]
    fn n_p1repeat20000() {
        check_large_stack(vec![1; 20000], 1);
    }
    #[test]
    fn n_m1repeat20000() {
        check_large_stack(vec![-1; 20000], 1);
    }
}
