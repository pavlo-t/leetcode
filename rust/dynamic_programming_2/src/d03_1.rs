#![allow(dead_code)]
/// 256. Paint House
/// ================
///
/// There is a row of `n` houses, where each house can be painted one of three colors: red, blue, or green.
/// The cost of painting each house with a certain color is different.
/// You have to paint all the houses such that no two adjacent houses have the same color.
///
/// The cost of painting each house with a certain color is represented by an `n x 3` cost matrix costs.
///
/// For example, `costs[0][0]` is the cost of painting house `0` with the color red;
/// `costs[1][2]` is the cost of painting house `1` with color green, and so on...
///
/// Return _the minimum cost to paint all houses_.
///
/// __Constraints:__
///
/// - `1 <= costs.length <= 100`
/// - `costs[i].length == 3`
/// - `1 <= costs[i][j] <= 20`
///
/// https://leetcode.com/problems/paint-house/
struct Solution;
impl Solution {
    /// 00:31-00:33
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        println!("min_cost({:?})", costs);
        const COLORS: usize = 3;
        let mut curr = [0; COLORS];
        let mut prev = curr.clone();
        for i in (0..costs.len()).rev() {
            std::mem::swap(&mut curr, &mut prev);
            for c in 0..COLORS {
                let min_next = (0..COLORS)
                    .filter(|&p| p != c)
                    .map(|c| prev[c])
                    .min()
                    .unwrap();
                curr[c] = costs[i][c] + min_next;
            }
        }
        curr.iter().min().unwrap().to_owned()
    }
    /// 00:20-00:31
    pub fn min_cost_dp_vec_vec(costs: Vec<Vec<i32>>) -> i32 {
        println!("min_cost({:?})", costs);
        const COLORS: usize = 3;
        let mut dp: Vec<Vec<i32>> = vec![vec![0; 3]; costs.len() + 1];
        for i in (0..costs.len()).rev() {
            for c in 0..COLORS {
                let next_min = (0..COLORS)
                    .filter(|&pc| pc != c)
                    .map(|c| dp[i + 1][c])
                    .min()
                    .unwrap();
                dp[i][c] = costs[i][c] + next_min;
            }
        }
        dp[0].iter().min().unwrap().to_owned()
    }
    /// 00:16-00:20
    pub fn min_cost_rec_with_memo(costs: Vec<Vec<i32>>) -> i32 {
        println!("min_cost({:?})", costs);
        const COLORS: usize = 3;
        static mut MEMO: Vec<Vec<i32>> = vec![];
        unsafe {
            MEMO = vec![vec![-1; 3]; costs.len()];
        }

        fn rec(i: usize, pc: usize, cs: &[Vec<i32>]) -> i32 {
            unsafe {
                if i == cs.len() {
                    0
                } else if pc < COLORS && MEMO[i][pc] >= 0 {
                    MEMO[i][pc]
                } else {
                    let result = (0..COLORS)
                        .filter(|&c| c != pc)
                        .map(|c| cs[i][c] + rec(i + 1, c, cs))
                        .min()
                        .unwrap();
                    if pc < COLORS {
                        MEMO[i][pc] = result;
                    }
                    result
                }
            }
        }
        rec(0, 4, &costs)
    }
    /// 00:06-00:16
    pub fn min_cost_rec(costs: Vec<Vec<i32>>) -> i32 {
        println!("min_cost({:?})", costs);
        const COLORS: usize = 3;

        fn rec(i: usize, pc: usize, cs: &[Vec<i32>]) -> i32 {
            if i == cs.len() {
                0
            } else {
                (0..COLORS)
                    .filter(|&c| c != pc)
                    .map(|c| cs[i][c] + rec(i + 1, c, cs))
                    .min()
                    .unwrap()
            }
        }
        rec(0, 4, &costs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv {($($x:tt),*) => {vec![$(vec!$x),*]};}

    #[test]
    fn c_17c2c17_17c2c17_16c16c5_14c3c19() {
        let c = vv![[17, 2, 17], [16, 16, 5], [14, 3, 19]];
        assert_eq!(Solution::min_cost(c), 10);
        // Explanation: Paint house 0 into blue, paint house 1 into green, paint house 2 into blue.
        // Minimum cost: 2 + 5 + 3 = 10.
    }
    #[test]
    fn c_762() {
        let c = vv![[7, 6, 2]];
        assert_eq!(Solution::min_cost(c), 2);
    }
    #[test]
    fn c_111_() {
        let c = vec![vec![1, 1, 1]; 100];
        assert_eq!(Solution::min_cost(c), 100);
    }
}
