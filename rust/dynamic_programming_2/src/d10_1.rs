#![allow(dead_code)]
/// 646. Maximum Length of Pair Chain
/// =================================
///
/// You are given an array of `n` pairs `pairs` where `pairs[i] = [left_i, right_i]` and `left_i < right_i`.
///
/// A pair `p2 = [c, d]` __follows__ a pair `p1 = [a, b]` if `b < c`.
/// A __chain__ of pairs can be formed in this fashion.
///
/// Return _the length longest chain which can be formed_.
///
/// You do not need to use up all the given intervals.
/// You can select pairs in any order.
///
/// __Constraints:__
///
/// - `1 <= pairs.length <= 1000`
/// - `-1000 <= lefti < righti <= 1000`
///
/// https://leetcode.com/problems/maximum-length-of-pair-chain/
struct Solution;
impl Solution {
    pub fn find_longest_chain_rec(mut pairs: Vec<Vec<i32>>) -> i32 {
        println!("find_longest_chain({:?})", pairs);

        /// l - last picked idx + 1; 0 - nothing picked yet
        /// r - current idx
        fn rec(l: usize, r: usize, ps: &[Vec<i32>]) -> i32 {
            if r == ps.len() {
                0
            } else if l == 0 || ps[l - 1][1] < ps[r][0] {
                rec(l, r + 1, ps).max(1 + rec(r + 1, r + 1, ps))
            } else {
                rec(l, r + 1, ps)
            }
        }

        pairs.sort_unstable();
        rec(0, 0, &pairs)
    }
    pub fn find_longest_chain_rec_with_memo(mut pairs: Vec<Vec<i32>>) -> i32 {
        println!("find_longest_chain({:?})", pairs);

        /// l - last picked idx + 1; 0 - nothing picked yet
        /// r - current idx
        fn rec(l: usize, r: usize, ps: &[Vec<i32>], memo: &mut Vec<Vec<i32>>) -> i32 {
            if memo[l][r] != -1 {
                memo[l][r]
            } else {
                memo[l][r] = if r == ps.len() {
                    0
                } else if l == 0 || ps[l - 1][1] < ps[r][0] {
                    rec(l, r + 1, ps, memo).max(1 + rec(r + 1, r + 1, ps, memo))
                } else {
                    rec(l, r + 1, ps, memo)
                };
                memo[l][r]
            }
        }

        pairs.sort_unstable();
        let n = pairs.len();
        let mut memo = vec![vec![-1; n + 1]; n + 1];
        rec(0, 0, &pairs, &mut memo)
    }
    pub fn find_longest_chain_dp_vec_vec(mut pairs: Vec<Vec<i32>>) -> i32 {
        println!("find_longest_chain({:?})", pairs);

        pairs.sort_unstable();
        let n = pairs.len();
        let mut dp = vec![vec![0; n + 1]; n + 1];
        for r in (0..n).rev() {
            for l in (0..=r).rev() {
                dp[l][r] = if l == 0 || pairs[l - 1][1] < pairs[r][0] {
                    dp[l][r + 1].max(1 + dp[r + 1][r + 1])
                } else {
                    dp[l][r + 1]
                };
            }
        }
        dp[0][0]
    }
    pub fn find_longest_chain_dp_vec(mut pairs: Vec<Vec<i32>>) -> i32 {
        println!("find_longest_chain({:?})", pairs);

        pairs.sort_unstable();
        let n = pairs.len();
        let mut dp = vec![0; n + 1];
        for r in (0..n).rev() {
            let next = 1 + dp[r + 1];
            dp[0] = dp[0].max(next);
            for l in (1..=r).rev() {
                if pairs[l - 1][1] < pairs[r][0] {
                    dp[l] = dp[l].max(next);
                }
            }
        }
        dp[0]
    }
    /// Approach #2: Greedy [Accepted]
    /// https://leetcode.com/problems/maximum-length-of-pair-chain/solution/
    pub fn find_longest_chain(mut pairs: Vec<Vec<i32>>) -> i32 {
        println!("find_longest_chain({:?})", pairs);
        pairs.sort_unstable_by_key(|p| p[1]);
        let mut curr = i32::MIN;
        let mut result = 0;
        for p in pairs {
            if curr < p[0] {
                curr = p[1];
                result += 1;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[rustfmt::skip] #[test] fn p_12()    { assert_eq!(Solution::find_longest_chain(vv![[1,2]      ]), 1); }
    #[rustfmt::skip] #[test] fn p_12_23() { assert_eq!(Solution::find_longest_chain(vv![[1,2],[2,3]]), 1); }
    #[rustfmt::skip] #[test] fn p_12_12() { assert_eq!(Solution::find_longest_chain(vv![[1,2],[1,2]]), 1); }
    #[rustfmt::skip] #[test] fn p_12_34() { assert_eq!(Solution::find_longest_chain(vv![[1,2],[3,4]]), 2); }
    #[rustfmt::skip] #[test] fn p_12_59() { assert_eq!(Solution::find_longest_chain(vv![[1,2],[5,9]]), 2); }
    #[rustfmt::skip] #[test] fn p_34_12() { assert_eq!(Solution::find_longest_chain(vv![[3,4],[1,2]]), 2); }
    #[rustfmt::skip] #[test] fn p_59_12() { assert_eq!(Solution::find_longest_chain(vv![[5,9],[1,2]]), 2); }

    #[rustfmt::skip] #[test] fn p_19_34_12() { assert_eq!(Solution::find_longest_chain(vv![[1,9],[3,4],[1,2]]), 2); }
    #[test]
    fn p_12_23_34() {
        let p = vv![[1, 2], [2, 3], [3, 4]];
        assert_eq!(Solution::find_longest_chain(p), 2);
        // Explanation: The longest chain is [1,2] -> [3,4].
    }
    #[test]
    fn p_12_78_45() {
        let p = vv![[1, 2], [7, 8], [4, 5]];
        assert_eq!(Solution::find_longest_chain(p), 3);
        // Explanation: The longest chain is [1,2] -> [4,5] -> [7,8].
    }

    #[test]
    fn p_12_repeat_1000() {
        let p = vec![vec![1, 2]; 1000];
        assert_eq!(Solution::find_longest_chain(p), 1);
    }
    #[test]
    fn p_m1000_until_p1000_chunks_2() {
        let p = (-1000..1000).step_by(2).map(|i| vec![i, i + 1]).collect();
        assert_eq!(Solution::find_longest_chain(p), 1000);
    }
}
