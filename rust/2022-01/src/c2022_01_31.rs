#![allow(dead_code)]
/// 1672. Richest Customer Wealth
/// =============================
///
/// You are given an `m x n` integer grid accounts where `accounts[i][j]`
/// is the amount of money the `i`th customer has in the `j`th bank.
/// Return _the __wealth__ that the richest customer has_.
///
/// A customer's __wealth__ is the amount of money they have in all their bank accounts.
/// The richest customer is the customer that has the maximum __wealth__.
///
/// __Constraints:__
///
/// - `m == accounts.length`
/// - `n == accounts[i].length`
/// - `1 <= m, n <= 50`
/// - `1 <= accounts[i][j] <= 100`
///
/// https://leetcode.com/problems/richest-customer-wealth/
struct Solution;
impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        accounts.iter().fold(0, |rsf, a| rsf.max(a.iter().sum()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }

    #[test]
    fn a_l_1_2_3_l_3_2_1() {
        let a = vv![[1, 2, 3], [3, 2, 1]];
        assert_eq!(Solution::maximum_wealth(a), 6);
        // Explanation:
        // 1st customer has wealth = 1 + 2 + 3 = 6
        // 2nd customer has wealth = 3 + 2 + 1 = 6
        // Both customers are considered the richest with a wealth of 6 each, so return 6.
    }
    #[test]
    fn a_l_1_5_l_7_3_l_3_5() {
        let a = vv![[1, 5], [7, 3], [3, 5]];
        assert_eq!(Solution::maximum_wealth(a), 10);
        // Explanation:
        // 1st customer has wealth = 6
        // 2nd customer has wealth = 10
        // 3rd customer has wealth = 8
        // The 2nd customer is the richest with a wealth of 10.
    }
    #[test]
    fn a_l_2_8_7_7_1_3_l_1_9_5() {
        let a = vv![[2, 8, 7], [7, 1, 3], [1, 9, 5]];
        assert_eq!(Solution::maximum_wealth(a), 17);
    }
}
