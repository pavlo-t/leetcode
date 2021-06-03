#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        if nums.is_empty() {
            return false
        }

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l <= r {
            let mid = l + (r - l) / 2;
            if nums[mid] == target {
                return true
            } else if nums[l] == nums[mid] {
                l += 1;
            } else {
                let m_in_left = nums[l] <= nums[mid];
                let t_in_left = nums[l] <= target;

                if (m_in_left && !t_in_left) || (m_in_left == t_in_left && nums[mid] < target) {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }
        }

        false
    }
}

struct SolutionRecursion;
impl SolutionRecursion {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        Self::bs(&nums, target)
    }

    fn bs(nums: &[i32], target: i32) -> bool {
        if nums.is_empty() {
            false
        } else {
            let mid = nums.len() / 2;

            if nums[mid] == target {
                true
            } else if nums[0] == nums[mid] {
                Self::bs(&nums[1..], target)
            } else {
                let m_in_left = nums[0] <= nums[mid];
                let t_in_left = nums[0] <= target;

                if (m_in_left && !t_in_left) || (m_in_left == t_in_left && nums[mid] < target) {
                    Self::bs(&nums[(mid + 1)..], target)
                } else {
                    Self::bs(&nums[..mid], target)
                }
            }
        }
    }
}

struct SolutionBuiltin;
impl SolutionBuiltin {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        nums.contains(&target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_5_6_0_0_1_2_t_0() {
        assert!(Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 0));
    }

    #[test]
    fn test_2_5_6_0_0_1_2_t_3() {
        assert!(!Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
    }

    #[test]
    fn test_5_1_3_t_3() {
        assert!(Solution::search(vec![5, 1, 3], 3));
    }

    #[test]
    fn test_empty() {
        assert!(!Solution::search(vec![], 1));
    }

    #[test]
    fn test_one_element() {
        assert!(Solution::search(vec![1], 1));
        assert!(!Solution::search(vec![1], 2));
    }

    #[test]
    fn test_2_elements() {
        assert!(Solution::search(vec![1, 2], 1));
        assert!(Solution::search(vec![2, 1], 2));
        assert!(!Solution::search(vec![1, 2], 0));
        assert!(!Solution::search(vec![2, 1], 0));
    }

    #[test]
    fn test_3_elements() {
        assert!(Solution::search(vec![1, 1, 1], 1));
        assert!(Solution::search(vec![1, 1, 2], 1));
        assert!(Solution::search(vec![2, 1, 1], 1));
        assert!(Solution::search(vec![1, 2, 1], 1));

        assert!(Solution::search(vec![1, 2, 3], 1));
        assert!(Solution::search(vec![2, 3, 1], 1));
        assert!(Solution::search(vec![3, 1, 2], 1));

        assert!(!Solution::search(vec![2, 1, 2], 0));
    }
}