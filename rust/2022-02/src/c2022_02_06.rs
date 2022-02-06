#![allow(dead_code)]
/// 80. Remove Duplicates from Sorted Array II
/// ==========================================
///
/// Given an integer array `nums` sorted in __non-decreasing order__,
/// remove some duplicates [in-place] such that each unique element appears __at most twice__.
/// The __relative order__ of the elements should be kept the __same__.
///
/// Since it is impossible to change the length of the array in some languages,
/// you must instead have the result be placed in the __first part__ of the array `nums`.
/// More formally, if there are `k` elements after removing the duplicates,
/// then the first `k` elements of `nums` should hold the final result.
/// It does not matter what you leave beyond the first `k` elements.
///
/// Return _`k` after placing the final result in the first `k` slots of `nums`_.
///
/// Do __not__ allocate extra space for another array.
/// You must do this by __modifying the input array__ [in-place] with `O(1)` extra memory.
///
/// __Custom Judge:__
///
/// The judge will test your solution with the following code:
///
/// ```
/// let mut nums: Vec<i32> = vec![...]; // Input array
/// let expected_nums: Vec<i32> = vec![...]; // The expected answer with correct length
///
/// let k = remove_duplicates(&mut nums); // Calls your implementation
///
/// assert_eq!(k, expected_nums.len());
/// for i in 0..k {
///     assert_eq!(nums[i], expected_nums[i]);
/// }
/// ```
///
/// If all assertions pass, then your solution will be __accepted__.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 30_000`
/// - `-10_000 <= nums[i] <= 10_000`
/// - `nums` is sorted in __non-decreasing__ order.
///
/// [in-place]:https://en.wikipedia.org/wiki/In-place_algorithm
///
/// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/
struct Solution;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            nums.len() as i32
        } else {
            let mut write = 1;
            let mut written_same = 1;
            const MAX_REPEAT: i32 = 2;
            for read in 1..nums.len() {
                if nums[read] != nums[read - 1] {
                    written_same = 0;
                }
                if written_same < MAX_REPEAT {
                    nums[write] = nums[read];
                    write += 1;
                    written_same += 1;
                }
            }
            nums.truncate(write);
            write as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        let mut ns = vec![1];
        assert_eq!(Solution::remove_duplicates(&mut ns), 1);
        let e = [1];
        for i in 0..e.len() {
            assert_eq!(ns[i], e[i]);
        }
    }
    #[test]
    fn p1p1() {
        let mut ns = vec![1, 1];
        assert_eq!(Solution::remove_duplicates(&mut ns), 2);
        let e = [1, 1];
        for i in 0..e.len() {
            assert_eq!(ns[i], e[i]);
        }
    }
    #[test]
    fn p1p1p1() {
        let mut ns = vec![1, 1, 1];
        assert_eq!(Solution::remove_duplicates(&mut ns), 2);
        let e = [1, 1];
        for i in 0..e.len() {
            assert_eq!(ns[i], e[i]);
        }
    }
    #[test]
    fn p1p1p1p2p2p2() {
        let mut ns = vec![1, 1, 1, 2, 2, 2];
        assert_eq!(Solution::remove_duplicates(&mut ns), 4);
        let e = [1, 1, 2, 2];
        for i in 0..e.len() {
            assert_eq!(ns[i], e[i]);
        }
    }
    #[test]
    fn p1p1p1p2p2p3() {
        let mut ns = vec![1, 1, 1, 2, 2, 3];
        assert_eq!(Solution::remove_duplicates(&mut ns), 5);
        let e = [1, 1, 2, 2, 3];
        for i in 0..e.len() {
            assert_eq!(ns[i], e[i]);
        }
        // Explanation: Your function should return k = 5,
        // with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
        // It does not matter what you leave beyond the returned k (hence they are underscores).
    }
    #[test]
    fn n0n0p1p1p1p1p2p3p3() {
        let mut ns = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        assert_eq!(Solution::remove_duplicates(&mut ns), 7);
        let e = [0, 0, 1, 1, 2, 3, 3];
        for i in 0..e.len() {
            assert_eq!(ns[i], e[i]);
        }
        // Explanation: Your function should return k = 7,
        // with the first seven elements of nums being 0, 0, 1, 1, 2, 3 and 3 respectively.
        // It does not matter what you leave beyond the returned k (hence they are underscores).
    }
}
