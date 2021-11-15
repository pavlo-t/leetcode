#![allow(dead_code)]
/// 368. Largest Divisible Subset
/// =============================
///
/// Given a set of __distinct__ positive integers `nums`,
/// return the largest subset `answer` such that every pair `(answer[i], answer[j])` of elements in this subset
/// satisfies `answer[i] % answer[j] == 0`, or `answer[j] % answer[i] == 0`.
///
/// If there are multiple solutions, return any of them.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 1000`
/// - `1 <= nums[i] <= 2_000_000_000`
/// - All the integers in nums are unique.
///
/// https://leetcode.com/problems/largest-divisible-subset/
struct Solution;
impl Solution {
    pub fn largest_divisible_subset_rec(mut nums: Vec<i32>) -> Vec<i32> {
        println!("largest_divisible_subset({:?})", nums);
        /// l: last picked ids + 1; 0 means nothing was picked yet
        fn rec(l: usize, r: usize, ns: &[i32]) -> Vec<i32> {
            if r == ns.len() {
                vec![]
            } else if l == 0 || ns[r] % ns[l - 1] == 0 {
                let skip = rec(l, r + 1, ns);
                let mut pick = rec(r + 1, r + 1, ns);
                if pick.len() >= skip.len() {
                    pick.push(ns[r]);
                    pick
                } else {
                    skip
                }
            } else {
                rec(l, r + 1, ns)
            }
        }
        nums.sort_unstable();
        (0..nums.len())
            .map(|r| rec(0, r, &nums))
            .max_by_key(|v| v.len())
            .unwrap()
    }
    pub fn largest_divisible_subset_rec_with_memo(mut nums: Vec<i32>) -> Vec<i32> {
        println!("largest_divisible_subset({:?})", nums);
        static mut MEMO: Vec<Vec<Option<Vec<i32>>>> = vec![];
        let n = nums.len();
        nums.sort_unstable();
        unsafe {
            MEMO = vec![vec![None; n + 1]; n + 1];
        }

        fn to_vec(s: &[i32]) -> Vec<i32> {
            s.iter().map(|&i| i).collect()
        }

        /// l: last picked ids + 1; 0 means nothing was picked yet
        fn rec(l: usize, r: usize, ns: &[i32]) -> &[i32] {
            unsafe {
                if let Some(res) = MEMO[r][l].as_ref() {
                    res
                } else {
                    MEMO[r][l] = Some(if r == ns.len() {
                        vec![]
                    } else if l == 0 || ns[r] % ns[l - 1] == 0 {
                        let skip = rec(l, r + 1, ns);
                        let pick = rec(r + 1, r + 1, ns);
                        if pick.len() >= skip.len() {
                            let mut pick = to_vec(pick);
                            pick.push(ns[r]);
                            pick
                        } else {
                            to_vec(skip)
                        }
                    } else {
                        to_vec(rec(l, r + 1, ns))
                    });
                    MEMO[r][l].as_ref().unwrap()
                }
            }
        }

        (0..nums.len())
            .map(|r| rec(0, r, &nums))
            .max_by_key(|v| v.len())
            .map(|v| v.iter().map(|&i| i).collect())
            .unwrap()
    }
    pub fn largest_divisible_subset_dp_vec_vec(mut nums: Vec<i32>) -> Vec<i32> {
        println!("largest_divisible_subset({:?})", nums);
        let n = nums.len();
        nums.sort_unstable();
        let mut dp: Vec<Vec<Vec<i32>>> = vec![vec![vec![]; n + 1]; n + 1];
        let mut max_idx = 0;
        for r in (0..n).rev() {
            for l in 0..=r {
                dp[r][l] = if l == 0 || nums[r] % nums[l - 1] == 0 {
                    if dp[r + 1][r + 1].len() >= dp[r + 1][l].len() {
                        let mut res = dp[r + 1][r + 1].clone();
                        res.push(nums[r]);
                        res
                    } else {
                        dp[r + 1][l].clone()
                    }
                } else {
                    dp[r + 1][l].clone()
                };
            }
            if dp[max_idx][0].len() < dp[r][0].len() {
                max_idx = r;
            }
        }

        dp[max_idx][0].to_owned()
    }
    pub fn largest_divisible_subset_dp_vec_1(mut nums: Vec<i32>) -> Vec<i32> {
        println!("largest_divisible_subset({:?})", nums);
        let n = nums.len();
        nums.sort_unstable();
        let mut dp: Vec<Vec<i32>> = vec![vec![]; n + 1];
        for r in (0..n).rev() {
            #[rustfmt::skip]
            let pick = dp[r + 1].iter().map(|&i| i).chain(std::iter::once(nums[r])).collect::<Vec<i32>>();
            if pick.len() > dp[0].len() {
                dp[0] = pick.clone();
            }
            for l in 1..=r {
                if nums[r] % nums[l - 1] == 0 && pick.len() > dp[l].len() {
                    dp[l] = pick.clone();
                }
            }
        }

        dp[0].to_owned()
    }
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        println!("largest_divisible_subset({:?})", nums);
        let n = nums.len();
        nums.sort_unstable();
        let mut dp: Vec<Vec<i32>> = vec![vec![]; n + 1];
        for r in (0..n).rev() {
            let mut pick = dp.pop().unwrap();
            pick.push(nums[r]);
            for l in 1..=r {
                if nums[r] % nums[l - 1] == 0 && pick.len() > dp[l].len() {
                    dp[l] = pick.clone();
                }
            }
            if pick.len() > dp[0].len() {
                dp[0] = pick;
            }
        }

        dp[0].to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    fn check(n: Vec<i32>, e: Vec<i32>) {
        let mut r = Solution::largest_divisible_subset(n);
        r.sort_unstable();
        assert_eq!(r, e);
    }
    fn check_mult(n: Vec<i32>, es: Vec<Vec<i32>>) {
        let mut r = Solution::largest_divisible_subset(n);
        r.sort_unstable();
        assert!(es.iter().any(|e| e == &r), "{:?} != any of {:?}", r, es);
    }

    #[rustfmt::skip] #[test] fn n_2()   { check(vec![2], vec![2]); }
    #[rustfmt::skip] #[test] fn n_23()  { check_mult(vec![2,3], vv![[2], [3]]); }
    #[rustfmt::skip] #[test] fn n_32()  { check_mult(vec![3,2], vv![[2], [3]]); }
    #[rustfmt::skip] #[test] fn n_24()  { check(vec![2,4]  , vec![2,4]); }
    #[rustfmt::skip] #[test] fn n_42()  { check(vec![4,2]  , vec![2,4]); }
    #[rustfmt::skip] #[test] fn n_239() { check(vec![2,3,9], vec![3,9]); }
    #[test]
    fn n_123() {
        check_mult(vec![1, 2, 3], vv![[1, 2], [1, 3]]);
        // Explanation: [1,3] is also accepted.
    }
    #[test]
    fn n_1248() {
        check(vec![1, 2, 4, 8], vec![1, 2, 4, 8]);
    }

    //#[ignore]
    #[test]
    fn test_48() {
        let n = vec![
            1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768, 65536,
            131072, 262144, 524288, 1048576, 2097152, 4194304, 8388608, 16777216, 33554432,
            67108864, 134217728, 268435456, 536870912, 1073741824,
        ];
        let e = n.clone();
        check(n, e);
    }

    //#[ignore]
    #[test]
    fn n_1to1000() {
        let n = (1..=1000).collect();
        let e = vv![
            [1, 2, 4, 8, 16, 32, 64, 128, 256, 512],
            [1, 3, 6, 12, 24, 48, 96, 192, 384, 768]
        ];
        check_mult(n, e);
    }
}
