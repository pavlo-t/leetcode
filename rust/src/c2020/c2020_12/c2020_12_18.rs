#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn increasing_triplet(nums: Vec<i32>) -> bool {
        let mut min = i32::max_value();
        let mut mid = i32::max_value();

        for i in nums {
            if i <= min {
                min = i;
            } else if i <= mid {
                mid = i;
            } else {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_nums12345_is_true() {
        assert!(Solution::increasing_triplet(vec![1, 2, 3, 4, 5]));
        //Explanation: Any triplet where i < j < k is valid.
    }

    #[test]
    fn example2_nums54321_is_false() {
        assert!(!Solution::increasing_triplet(vec![5, 4, 3, 2, 1]));
        //Explanation: No triplet exists.
    }

    #[test]
    fn example3_nums215046_is_true() {
        assert!(Solution::increasing_triplet(vec![2, 1, 5, 0, 4, 6]));
        //Explanation: The triplet (3, 4, 5) is valid because nums[3] == 0 < nums[4] == 4 < nums[5] == 6.
    }

    #[test]
    fn nums111_is_false() {
        assert!(!Solution::increasing_triplet(vec![1, 1, 1]));
    }

    #[test]
    fn nums213_is_false() {
        assert!(!Solution::increasing_triplet(vec![2, 1, 3]));
    }

    #[test]
    fn nums225544_is_false() {
        assert!(!Solution::increasing_triplet(vec![2, 2, 5, 5, 4, 4]));
    }

    #[test]
    fn nums25454_is_true() {
        assert!(Solution::increasing_triplet(vec![2, 5, 4, 5, 4]));
    }

    #[test]
    fn nums_100000_elements_is_true() {
        let mut nums = vec![1; 100000];
        nums[9998] = 2;
        nums[9999] = 3;
        assert!(Solution::increasing_triplet(nums));
    }

    #[test]
    fn nums_100000_elements_is_false() {
        assert!(!Solution::increasing_triplet(vec![1; 100000]));
    }
}