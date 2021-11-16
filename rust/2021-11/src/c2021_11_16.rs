#![allow(dead_code)]
/// 668. Kth Smallest Number in Multiplication Table
/// ================================================
///
/// Nearly everyone has used the [Multiplication Table](https://en.wikipedia.org/wiki/Multiplication_table).
/// The multiplication table of size `m x n` is an integer matrix `mat` where `mat[i][j] == i * j` __(1-indexed)__.
///
/// Given three integers `m`, `n`, and `k`,
/// return _the `k`th smallest element in the `m x n` multiplication table_.
///
/// __Constraints:__
///
/// - `1 <= m, n <= 30_000`
/// - `1 <= k <= m * n`
///
/// https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/
struct Solution;
impl Solution {
    pub fn find_kth_number_my_brute_force(m: i32, n: i32, k: i32) -> i32 {
        println!("find_kth_number({}, {}, {})", m, n, k);
        let mut nums = Vec::with_capacity((m * n) as usize);
        for i in 1..=m {
            for j in 1..=n {
                nums.push(i * j);
            }
        }
        nums.sort_unstable();
        nums[(k - 1) as usize]
    }
    /// Approach #3: Binary Search [Accepted]
    /// https://leetcode.com/problems/kth-smallest-number-in-multiplication-table/solution/
    /// https://www.tutorialspoint.com/kth-smallest-number-in-multiplication-table-in-cplusplus
    /// https://www.programmerall.com/article/703272243/
    pub fn find_kth_number(m: i32, n: i32, k: i32) -> i32 {
        println!("find_kth_number({}, {}, {})", m, n, k);
        let (mut l, mut r) = (1, m * n);
        while l < r {
            let mid = l + (r - l) / 2;
            let count = (1..=m).fold(0, |rsf, i| rsf + n.min(mid / i));
            if count < k {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn m1_n1_k1() { assert_eq!(Solution::find_kth_number(1, 1, 1), 1); }

    #[rustfmt::skip] #[test] fn m1_n2_k1() { assert_eq!(Solution::find_kth_number(1, 2, 1), 1); }
    #[rustfmt::skip] #[test] fn m1_n2_k2() { assert_eq!(Solution::find_kth_number(1, 2, 2), 2); }

    #[rustfmt::skip] #[test] fn m2_n1_k1() { assert_eq!(Solution::find_kth_number(2, 1, 1), 1); }
    #[rustfmt::skip] #[test] fn m2_n1_k2() { assert_eq!(Solution::find_kth_number(2, 1, 2), 2); }

    #[rustfmt::skip] #[test] fn m2_n2_k1() { assert_eq!(Solution::find_kth_number(2, 2, 1), 1); }
    #[rustfmt::skip] #[test] fn m2_n2_k2() { assert_eq!(Solution::find_kth_number(2, 2, 2), 2); }
    #[rustfmt::skip] #[test] fn m2_n2_k3() { assert_eq!(Solution::find_kth_number(2, 2, 3), 2); }
    #[rustfmt::skip] #[test] fn m2_n2_k4() { assert_eq!(Solution::find_kth_number(2, 2, 4), 4); }

    // 1 2 3
    // 2 4 6
    //
    // 1 2 2 3 4 6
    #[rustfmt::skip] #[test] fn m2_n3_k1() { assert_eq!(Solution::find_kth_number(2, 3, 1), 1); }
    #[rustfmt::skip] #[test] fn m2_n3_k2() { assert_eq!(Solution::find_kth_number(2, 3, 2), 2); }
    #[rustfmt::skip] #[test] fn m2_n3_k3() { assert_eq!(Solution::find_kth_number(2, 3, 3), 2); }
    #[rustfmt::skip] #[test] fn m2_n3_k4() { assert_eq!(Solution::find_kth_number(2, 3, 4), 3); }
    #[rustfmt::skip] #[test] fn m2_n3_k5() { assert_eq!(Solution::find_kth_number(2, 3, 5), 4); }
    #[rustfmt::skip] #[test] fn m2_n3_k6() { assert_eq!(Solution::find_kth_number(2, 3, 6), 6); }

    #[rustfmt::skip] #[test] fn m3_n2_k1() { assert_eq!(Solution::find_kth_number(3, 2, 1), 1); }
    #[rustfmt::skip] #[test] fn m3_n2_k2() { assert_eq!(Solution::find_kth_number(3, 2, 2), 2); }
    #[rustfmt::skip] #[test] fn m3_n2_k3() { assert_eq!(Solution::find_kth_number(3, 2, 3), 2); }
    #[rustfmt::skip] #[test] fn m3_n2_k4() { assert_eq!(Solution::find_kth_number(3, 2, 4), 3); }
    #[rustfmt::skip] #[test] fn m3_n2_k5() { assert_eq!(Solution::find_kth_number(3, 2, 5), 4); }
    #[rustfmt::skip] #[test] fn m3_n2_k6() { assert_eq!(Solution::find_kth_number(3, 2, 6), 6); }

    // 1 2 3
    // 2 4 6
    // 3 6 9
    //
    // 1 2 2 3 3 4 6 6 9
    #[rustfmt::skip] #[test] fn m3_n3_k1() { assert_eq!(Solution::find_kth_number(3, 3, 1), 1); }
    #[rustfmt::skip] #[test] fn m3_n3_k2() { assert_eq!(Solution::find_kth_number(3, 3, 2), 2); }
    #[rustfmt::skip] #[test] fn m3_n3_k3() { assert_eq!(Solution::find_kth_number(3, 3, 3), 2); }
    #[rustfmt::skip] #[test] fn m3_n3_k4() { assert_eq!(Solution::find_kth_number(3, 3, 4), 3); }
    #[rustfmt::skip] #[test] fn m3_n3_k5() { assert_eq!(Solution::find_kth_number(3, 3, 5), 3); }
    #[rustfmt::skip] #[test] fn m3_n3_k6() { assert_eq!(Solution::find_kth_number(3, 3, 6), 4); }
    #[rustfmt::skip] #[test] fn m3_n3_k7() { assert_eq!(Solution::find_kth_number(3, 3, 7), 6); }
    #[rustfmt::skip] #[test] fn m3_n3_k8() { assert_eq!(Solution::find_kth_number(3, 3, 8), 6); }
    #[rustfmt::skip] #[test] fn m3_n3_k9() { assert_eq!(Solution::find_kth_number(3, 3, 9), 9); }

    #[test]
    fn m10_n10_k100() {
        assert_eq!(Solution::find_kth_number(10, 10, 100), 100);
    }

    //#[ignore]
    #[rustfmt::skip] #[test]
    fn m10000_n10000_k100_000_000() { assert_eq!(Solution::find_kth_number(10000, 10000, 100_000_000), 100_000_000); }
    //#[ignore]
    #[rustfmt::skip] #[test]
    fn m30000_n30000_k900_000_000() { assert_eq!(Solution::find_kth_number(30000, 30000, 900_000_000), 900_000_000); }
}
