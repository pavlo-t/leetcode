#![allow(dead_code)]
/// 1359. Count All Valid Pickup and Delivery Options
/// =================================================
///
/// Given `n` orders, each order consist in pickup and delivery services.
///
/// Count all valid pickup/delivery possible sequences such that delivery(i) is always after of pickup(i).
///
/// Since the answer may be too large, return it modulo `10^9 + 7`.
///
/// __Constraints:__
///
/// - `1 <= n <= 500`
///
/// https://leetcode.com/problems/count-all-valid-pickup-and-delivery-options/
struct Solution;
impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut result = 1i64;
        for i in 1..=n as i64 * 2 {
            if i % 2 == 0 {
                result *= i / 2;
            } else {
                result *= i;
            }
            result %= 1_000_000_007;
        }
        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_1() {
        assert_eq!(Solution::count_orders(1), 1);
        // Explanation: Unique order (P1, D1), Delivery 1 always is after of Pickup 1.
    }
    #[test]
    fn n_2() {
        assert_eq!(Solution::count_orders(2), 6);
        // Explanation: All possible orders:
        // (P1,P2,D1,D2), (P1,P2,D2,D1), (P1,D1,P2,D2), (P2,P1,D1,D2), (P2,P1,D2,D1) and (P2,D2,P1,D1).
        // This is an invalid order (P1,D2,P2,D1) because Pickup 2 is after of Delivery 2.
    }
    #[rustfmt::skip] #[test] fn n_3() { assert_eq!(Solution::count_orders(3), 90); } // (3*2)! / 2**3
    #[rustfmt::skip] #[test] fn n_4() { assert_eq!(Solution::count_orders(4), 2520); } // (4*2)! / 2**4
    #[rustfmt::skip] #[test] fn n_5() { assert_eq!(Solution::count_orders(5), 113400); } // (5*2)! / 2**5
    #[rustfmt::skip] #[test] fn n_6() { assert_eq!(Solution::count_orders(6), 7484400); } // (6*2)! / 2**6
    #[rustfmt::skip] #[test] fn n_7() { assert_eq!(Solution::count_orders(7), 681080400); }
    #[rustfmt::skip] #[test] fn n_8() { assert_eq!(Solution::count_orders(8), 729647433); }
    #[rustfmt::skip] #[test] fn n_9() { assert_eq!(Solution::count_orders(9), 636056472); }

    #[rustfmt::skip] #[test] fn n_500() { assert_eq!(Solution::count_orders(500), 764678010); }
}
