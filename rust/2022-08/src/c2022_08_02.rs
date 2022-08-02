#![allow(dead_code)]
//! \#378. Kth Smallest Element in a Sorted Matrix
//! ==============================================
//!
//! <https://leetcode.com/problems/kth-smallest-element-in-a-sorted-matrix>
//!
//! Given an `n x n` `matrix` where each of the rows and columns is sorted in ascending order,
//! return _the `k`th smallest element in the matrix_.
//!
//! Note that it is the `k`th smallest element __in the sorted order__, not the `k`th __distinct__ element.
//!
//! You must find a solution with a memory complexity better than `O(n²)`.
//!
//! __Constraints:__
//!
//! - `n == matrix.length == matrix[i].length`
//! - `1 <= n <= 300`
//! - `-1_000_000_000 <= matrix[i][j] <= 1_000_000_000`
//! - All the rows and columns of `matrix` are __guaranteed__ to be sorted in __non-decreasing order__.
//! - `1 <= k <= n²`
//!
//! __Follow up:__
//!
//! - Could you solve the problem with a constant memory (i.e., `O(1)` memory complexity)?
//! - Could you solve the problem in `O(n)` time complexity?
//!   The solution may be too advanced for an interview but you may find reading [this paper][1] fun.
//!
//! [1]:http://www.cse.yorku.ca/~andy/pubs/X+Y.pdf
//!
//! # Examples
//!
//! ```
//! # use c2022_08::c2022_08_02::Solution;
//! # macro_rules! vv { ($($t:tt),*) => { vec![$(vec!$t),*] }; }
//! let matrix = vv![[ 1,  5,  9],
//!                  [10, 11, 13],
//!                  [12, 13, 15]];
//! assert_eq!(Solution::kth_smallest(matrix, 8), 13);
//! ```
//!
//! __Explanation:__
//! The elements in the matrix are `[1,5,9,10,11,12,13,13,15]`,
//! and the `8`th smallest number is `13`.
//!
//! ```
//! # use c2022_08::c2022_08_02::Solution;
//! # macro_rules! vv { ($($t:tt),*) => { vec![$(vec!$t),*] }; }
//! let matrix = vv![[-5]];
//! assert_eq!(Solution::kth_smallest(matrix, 1), -5);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_02::Solution;
//! # macro_rules! vv { ($($t:tt),*) => { vec![$(vec!$t),*] }; }
//! let matrix = vv![[ 1, 2, 3],
//!                  [ 4, 5, 6],
//!                  [ 7, 8, 9]];
//! assert_eq!(Solution::kth_smallest(matrix, 1), 1);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_02::Solution;
//! # macro_rules! vv { ($($t:tt),*) => { vec![$(vec!$t),*] }; }
//! let matrix = vv![[ 1, 2, 3],
//!                  [ 4, 5, 6],
//!                  [ 7, 8, 9]];
//! assert_eq!(Solution::kth_smallest(matrix, 8), 8);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_02::Solution;
//! # macro_rules! vv { ($($t:tt),*) => { vec![$(vec!$t),*] }; }
//! let matrix = vv![[ 1, 4, 7],
//!                  [ 2, 5, 8],
//!                  [ 3, 6, 9]];
//! assert_eq!(Solution::kth_smallest(matrix, 8), 8);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_02::Solution;
//! let matrix = vec![vec![1; 300]; 300];
//! assert_eq!(Solution::kth_smallest(matrix, 90_000), 1);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_02::Solution;
//! let mut matrix = vec![vec![1_000_000_000; 300]; 300];
//! matrix[0][0] = -1_000_000_000;
//! assert_eq!(Solution::kth_smallest(matrix, 1), -1_000_000_000);
//! ```
//!
//! ```
//! # use c2022_08::c2022_08_02::Solution;
//! let mut matrix = vec![vec![-1_000_000_000; 300]; 300];
//! matrix[299][299] = 1_000_000_000;
//! assert_eq!(Solution::kth_smallest(matrix, 90_000), 1_000_000_000);
//! ```

pub struct Solution;
impl Solution {
    /// Using min heap
    ///
    /// - `O(n * log n)` time
    /// - `O(n)` memory
    pub fn kth_smallest_v1(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let init_heap = |mut heap: BinaryHeap<Reverse<(i32, usize, usize)>>, (col, &num)| {
            heap.push(Reverse((num, 0, col)));
            heap
        };

        let has_next_row = |Reverse((_, r, _)): &Reverse<(i32, usize, usize)>| r + 1 < matrix.len();
        let next_row = |Reverse((_, r, c)): Reverse<(i32, usize, usize)>| {
            Reverse((matrix[r + 1][c], r + 1, c))
        };

        let next_heap = |mut heap: BinaryHeap<Reverse<(i32, usize, usize)>>, _: i32| {
            if let Some(next_row) = heap.pop().filter(has_next_row).map(next_row) {
                heap.push(next_row);
            }
            heap
        };

        let heap = BinaryHeap::new();
        let heap = matrix[0].iter().enumerate().fold(heap, init_heap);
        let heap = (1..k).fold(heap, next_heap);
        heap.peek().unwrap().0 .0
    }

    /// Using binary search
    ///
    /// - `O(n * log (max - min))` time
    /// - `O(1)` memory
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::Ordering;

        let n = matrix.len();

        let count_less_than_or_equal = |x: i32| {
            let mut count = 0;
            let (mut r, mut c) = (n - 1, 0);
            while r < n && c < n {
                if matrix[r][c] <= x {
                    count += r + 1;
                    c += 1;
                } else if r > 0 {
                    r -= 1;
                } else {
                    break;
                }
            }
            count as i32
        };

        let greatest_less_than_or_equal = |x: i32| {
            *matrix
                .iter()
                .flat_map(|v| v.iter())
                .filter(|&&num| num <= x)
                .max()
                .unwrap()
        };

        let (mut l, mut r) = (matrix[0][0], matrix[n - 1][n - 1]);

        while l < r {
            let mid = (l + r) / 2;
            match count_less_than_or_equal(mid).cmp(&k) {
                Ordering::Equal => {
                    l = mid;
                    break;
                }
                Ordering::Less => l = mid + 1,
                Ordering::Greater => r = mid - 1,
            }
        }

        greatest_less_than_or_equal(l)
    }
}
