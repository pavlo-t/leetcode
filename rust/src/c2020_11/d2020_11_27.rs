#![allow(dead_code)]

// @formatter:off
struct Solution;
// @formatter:on
impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum: i32 = nums.iter().sum();

        if sum % 2 == 1 {
            return false;
        }

        sum /= 2;
        sum += 1;

        let mut comp = vec![false; sum as usize];

        comp[0] = true;

        for i in nums {
            for j in (i..sum).rev() {
                comp[j as usize] = comp[j as usize] || comp[(j - i) as usize];
            }

            if *comp.last().unwrap() {
                return true;
            }
        }

        false
    }
}
// @formatter:off

struct Solution1;
impl Solution1 {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        if total % 2 == 1 {
            false
        } else {
            struct DFS {
                target: i32,
                nums: Box<Vec<i32>>,
                /// 0: not set; 1: false; 2: true
                memo: Vec<Vec<u8>>,
            }
            impl DFS {
                fn new(nums: Box<Vec<i32>>, target: i32) -> Self {
                    let outer_len = nums.len();
                    let inner_len = target as usize;
                    DFS {
                        target,
                        nums,
                        memo: vec![vec![0; inner_len]; outer_len],
                    }
                }

                fn dfs(&mut self, i: usize, sum: i32) -> bool {
                    let j = sum as usize;
                    if sum == self.target {
                        true
                    } else if sum > self.target || i == self.nums.len() {
                        false
                    } else if self.memo[i][j] != 0 {
                        self.memo[i][j] == 2
                    } else {
                        let result = self.dfs(i + 1, sum + self.nums[i]) || self.dfs(i + 1, sum);
                        self.memo[i][j] = if result { 2 } else { 1 };
                        result
                    }
                }
            }
            let mut dfs = DFS::new(Box::new(nums), total / 2);
            dfs.dfs(0, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_5_11_5_is_t() {
        let nums = vec![1,5,11,5];
        assert!(Solution::can_partition(nums));
    }
    #[test]
    fn test_1_2_3_5_is_f() {
        let nums = vec![1,2,3,5];
        assert!(!Solution::can_partition(nums));
    }
    #[test]
    fn test_200elements_is_f() {
        let mut nums = Vec::new();
        for _ in 1..200 { nums.push(100); };
        nums.push(98);
        assert!(!Solution::can_partition(nums));
    }
    #[test]
    fn test_200elements_is_t() {
        let mut nums = Vec::new();
        for i in 1..=100 {
            nums.push(i);
            nums.push(i);
        };
        assert!(Solution::can_partition(nums));
    }
}
