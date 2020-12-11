#![allow(dead_code)]

struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() < 2 {
            nums.len() as i32
        } else {
            let mut j = 2;
            for i in j..nums.len() {
                if nums[j - 2] != nums[i] {
                    nums[j] = nums[i];
                    j += 1;
                }
            }
            j as i32
        }
    }

    /// https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solution/
    pub fn remove_duplicates_2_pointers(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            0
        } else {
            let mut count = 1;
            let mut j = 1;

            for i in 1..nums.len() {
                if nums[i - 1] == nums[i] {
                    count += 1;
                } else {
                    count = 1;
                }

                if count <= 2 {
                    nums[j] = nums[i];
                    j += 1;
                }
            }

            j as i32
        }
    }

    /// 33-40ms
    pub fn remove_duplicates_actually_remove(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            0
        } else {
            let mut count = 1;
            let mut i = 1;

            while i < nums.len() {
                if nums[i - 1] == nums[i] {
                    count += 1;
                } else {
                    count = 1;
                }

                if count > 2 {
                    nums.remove(i);
                } else {
                    i += 1;
                }
            }

            nums.len() as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_remove_duplicates(nums: &mut Vec<i32>, expected_size: i32, expected_nums: Vec<i32>) {
        let size = Solution::remove_duplicates(nums);
        assert_eq!(size, expected_size);

        let mutated_nums: Vec<_> = nums.iter().take(size as usize).collect();
        let expected_nums: Vec<_> = expected_nums.iter().collect();
        assert_eq!(mutated_nums, expected_nums);
    }

    #[test]
    fn example1_nums111223_is_5_nums11223() {
        let mut nums = vec![1, 1, 1, 2, 2, 3];
        let expected = vec![1, 1, 2, 2, 3];

        test_remove_duplicates(&mut nums, 5, expected);
        //Explanation:
        // Your function should return length = 5,
        // with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
        // It doesn't matter what you leave beyond the returned length.
    }

    #[test]
    fn example2_nums001111233_is_7_nums0011233() {
        let mut nums = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
        let expected = vec![0, 0, 1, 1, 2, 3, 3];

        test_remove_duplicates(&mut nums, 7, expected);
        //Explanation:
        // Your function should return length = 7,
        // with the first seven elements of nums being modified to 0, 0, 1, 1, 2, 3 and 3 respectively.
        // It doesn't matter what values are set beyond the returned length.
    }

    #[test]
    fn test_nums_empty_is_0_nums_empty1() {
        let mut nums = vec![];
        let expected: Vec<i32> = vec![];

        test_remove_duplicates(&mut nums, 0, expected);
    }

    #[test]
    fn test_nums30000_1s_is_2_num11() {
        let mut nums = vec![];
        for _ in 1..=30000 { nums.push(1); }
        let expected: Vec<i32> = vec![1, 1];

        test_remove_duplicates(&mut nums, 2, expected);
    }
}