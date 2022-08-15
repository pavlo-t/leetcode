#![allow(dead_code)]
//! \#1570. Dot Product of Two Sparse Vectors
//! =========================================
//!
//! <https://leetcode.com/problems/dot-product-of-two-sparse-vectors>
//!
//! Given two sparse vectors, compute their dot product.
//!
//! Implement struct `SparseVector`:
//!
//! - `SparseVector::new(nums)` Initializes the object with the vector `nums`
//! - `dot_product(vec)` Compute the dot product between the instance of _SparseVector_ and `vec`
//!
//! A __sparse vector__ is a vector that has mostly zero values,
//! you should store the sparse vector __efficiently__ and compute the dot product between two _SparseVector_.
//!
//! __Follow up:__ What if only one of the vectors is sparse?
//!
//! ##### Examples
//!
//! ###### Example 1:
//!
//! ```
//! # use c2022_08::c2022_08_w3::*;
//! let v1 = SparseVector::new(vec![1, 0, 0, 2, 3]);
//! let v2 = SparseVector::new(vec![0, 3, 0, 4, 0]);
//! assert_eq!(v1.dot_product(v2), 8);
//! ```
//!
//! __Explanation:__ `v1.dot_product(v2) = 1*0 + 0*3 + 0*0 + 2*4 + 3*0 = 8`
//!
//! ###### Example 2:
//!
//! ```
//! # use c2022_08::c2022_08_w3::*;
//! let v1 = SparseVector::new(vec![0, 1, 0, 0, 0]);
//! let v2 = SparseVector::new(vec![0, 0, 0, 0, 2]);
//! assert_eq!(v1.dot_product(v2), 0);
//! ```
//!
//! __Explanation:__ `v1.dot_product(v2) = 0*0 + 1*0 + 0*0 + 0*0 + 0*2 = 0`
//!
//! ###### Example 3:
//!
//! ```
//! # use c2022_08::c2022_08_w3::*;
//! let v1 = SparseVector::new(vec![0, 1, 0, 0, 2, 0, 0]);
//! let v2 = SparseVector::new(vec![1, 0, 0, 0, 3, 0, 4]);
//! assert_eq!(v1.dot_product(v2), 6);
//! ```
//!
//! ##### Constraints
//!
//! - `n == nums1.length == nums2.length`
//! - `1 <= n <= 100_000`
//! - `0 <= nums1[i], nums2[i] <= 100`

pub struct SparseVector {
    nums: Vec<(usize, i32)>,
}
impl SparseVector {
    pub fn new(nums: Vec<i32>) -> Self {
        Self {
            nums: nums
                .into_iter()
                .enumerate()
                .filter(|&(_, num)| num != 0)
                .collect(),
        }
    }

    pub fn dot_product(&self, vec: SparseVector) -> i32 {
        let mut result = 0;
        let (mut i1, mut i2) = (0, 0);

        while i1 < self.nums.len() && i2 < vec.nums.len() {
            let (idx_1, num_1) = self.nums[i1];
            let (idx_2, num_2) = vec.nums[i2];
            if idx_1 < idx_2 {
                i1 += 1;
            } else if idx_1 > idx_2 {
                i2 += 1;
            } else {
                result += num_1 * num_2;
                i1 += 1;
                i2 += 1;
            }
        }

        result
    }
}
