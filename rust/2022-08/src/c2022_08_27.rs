#![allow(dead_code)]
//! \#363. Max Sum of Rectangle No Larger Than K
//! ============================================
//!
//! <https://leetcode.com/problems/max-sum-of-rectangle-no-larger-than-k>
//!
//! Given an `m x n` matrix `matrix` and an integer `k`, return
//! _the max sum of a rectangle in the matrix such that its sum is no larger than `k`_.
//!
//! It is __guaranteed__ that there will be a rectangle with a sum no larger than `k`.
//!
//! ##### Examples
//!
//! ###### Example 1
//!
//! ```
//! # use c2022_08::c2022_08_27::*;
//! # use c2022_08::vv;
//! let matrix = vv![[1, 0, 1], [0, -2, 3]];
//! assert_eq!(Solution::max_sum_submatrix(matrix, 2), 2);
//! ```
//!
//! __Explanation:__ Because the sum of the rectangle `[[0, 1], [-2, 3]]` is `2`,
//! and `2` is the max number no larger than `k` (`k = 2`).
//!
//! ###### Example 2
//!
//! ```
//! # use c2022_08::c2022_08_27::*;
//! # use c2022_08::vv;
//! let matrix = vv![[2, 2, -1]];
//! assert_eq!(Solution::max_sum_submatrix(matrix, 3), 3);
//! ```
//!
//! ##### Constraints
//!
//! - `m == matrix.length`
//! - `n == matrix[i].length`
//! - `1 <= m, n <= 100`
//! - `-100 <= matrix[i][j] <= 100`
//! - `-100_000 <= k <= 100_000`
//!
//! ##### Follow up
//!
//! What if the number of rows is much larger than the number of columns?

pub struct Solution;
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::collections::BTreeSet;

        let (n, m) = (matrix.len(), matrix[0].len());

        let mut prefix = vec![vec![0; m + 1]; n];
        for i in 0..n {
            for j in 0..m {
                prefix[i][j + 1] = prefix[i][j] + matrix[i][j];
            }
        }

        let mut result = i32::MIN;
        for start in 0..m {
            for end in start + 1..=m {
                let mut bts: BTreeSet<i32> = BTreeSet::new();
                bts.insert(0);
                let mut sum = 0;
                for i in 0..n {
                    sum += prefix[i][end] - prefix[i][start];
                    if let Some(prev) = bts.range(sum - k..).take(1).next() {
                        result = result.max(sum - prev);
                    }
                    bts.insert(sum);
                }
            }
        }
        result
    }
}
