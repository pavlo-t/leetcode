#![allow(dead_code)]
/// Sort Array By Parity II
/// =======================
///
/// Given an array of integers `nums`, half of the integers in `nums` are __odd__, and the other half are __even__.
///
/// Sort the array so that whenever `nums[i]` is odd, `i` is __odd__, and whenever `nums[i]` is even, i is __even__.
///
/// Return _any answer array that satisfies this condition_.
///
/// __Constraints:__
///
/// - `2 <= nums.length <= 20_000`
/// - `nums.length` is even.
/// - Half of the integers in `nums` are even.
/// - `0 <= nums[i] <= 1000`
///
/// __Follow Up:__ Could you solve it in-place?
///
/// https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/639/week-4-september-22nd-september-28th/3990/
struct Solution;
impl Solution {
    pub fn sort_array_by_parity_ii(mut nums: Vec<i32>) -> Vec<i32> {
        let mut e = 0;
        let mut o = 1;
        while e < nums.len() {
            if nums[e] % 2 != 0 {
                while nums[o] % 2 == 1 {
                    o += 2;
                }
                nums.swap(e, o);
            }
            e += 2;
        }

        nums
    }

    pub fn sort_array_by_parity_ii_my_1(mut nums: Vec<i32>) -> Vec<i32> {
        for i in 0..nums.len() {
            let p = (i % 2) as i32;
            if nums[i] % 2 != p {
                let mut j = i + 1;
                while nums[j] % 2 != p {
                    j += 2;
                }
                nums.swap(i, j);
            }
        }

        nums
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_4_2_5_7() {
        let n = vec![4, 2, 5, 7];
        let e = [4, 5, 2, 7];
        assert_eq!(Solution::sort_array_by_parity_ii(n), e);
        // Explanation: [4,7,2,5], [2,5,4,7], [2,7,4,5] would also have been accepted.
    }
    #[test]
    fn n_2_3() {
        let n = vec![2, 3];
        let e = [2, 3];
        assert_eq!(Solution::sort_array_by_parity_ii(n), e);
    }
}
