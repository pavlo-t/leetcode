struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut nums = nums;
        nums.push(1);
        nums.insert(0, 1);
        let nums = nums;

        let mut dp = vec![vec![0; nums.len()]; nums.len()];

        for size in 1..=len {
            for li in 1..=(len - size + 1) {
                let ri = li + size - 1;
                for last in li..=ri {
                    let m = dp[li][last - 1] + nums[li - 1] * nums[last] * nums[ri + 1] + dp[last + 1][ri];
                    dp[li][ri] = dp[li][ri].max(m);
                }
            }
        }

        dp[1][len]
    }

    pub fn max_coins_dp2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut dp = vec![vec![0; nums.len()]; nums.len()];

        for size in 1..=nums.len() {
            for li in 0..=(nums.len() - size) {
                let ri = li + size - 1;
                for last in li..=ri {
                    let lv = if li == 0 { 1 } else { nums[li - 1] };
                    let rv = if ri == (nums.len() - 1) { 1 } else { nums[ri + 1] };

                    let l_sum = if li == last { 0 } else { dp[li][last - 1] };
                    let r_sum = if ri == last { 0 } else { dp[last + 1][ri] };

                    dp[li][ri] = dp[li][ri].max(l_sum + lv * nums[last] * rv + r_sum);
                }
            }
        }

        dp[0][nums.len() - 1]
    }

    pub fn max_coins_no_overflow(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut dp = vec![vec![0i64; nums.len()]; nums.len()];

        for size in 1..=nums.len() {
            for li in 0..=(nums.len() - size) {
                let ri = li + size - 1;
                for last in li..=ri {
                    let lv = if li == 0 { 1 } else { nums[li - 1] as i64 };
                    let rv = if ri == (nums.len() - 1) { 1 } else { nums[ri + 1] as i64 };

                    let l_sum = if li == last { 0 } else { dp[li][last - 1] };
                    let r_sum = if ri == last { 0 } else { dp[last + 1][ri] };

                    dp[li][ri] = dp[li][ri].max(l_sum + lv * nums[last] as i64 * rv + r_sum);
                }
            }
        }

        dp[0][nums.len() - 1] as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example3158_is167() {
        let nums = vec![3, 1, 5, 8];
        assert_eq!(Solution::max_coins(nums), 167);
        //Output: 167
        //Explanation: nums = [3,1,5,8] --> [3,5,8] -->   [3,8]   -->  [8]  --> []
        //             coins =  3*1*5      +  3*5*8    +  1*3*8      + 1*8*1   = 167
    }

    #[test]
    fn nums_empty_is0() {
        assert_eq!(Solution::max_coins(vec![]), 0);
    }

    #[test]
    fn nums1to5_is110() {
        assert_eq!(Solution::max_coins(vec![1, 2, 3, 4, 5]), 110);
    }

    #[test]
    fn nums1to10_is2420() {
        let nums = (1..=10).collect();
        assert_eq!(Solution::max_coins(nums), 2420);
    }

    #[test]
    fn nums10to1_is2420() {
        let nums = (1..=10).rev().collect();
        assert_eq!(Solution::max_coins(nums), 2420);
    }

    #[test]
    fn nums500_2s_is3990() {
        assert_eq!(Solution::max_coins(vec![2; 500]), 3990);
    }

    // #[test]
    // fn nums1to500_is2_147_483_640() {
    //     let nums = (1..=500).collect();
    //     assert_eq!(Solution::max_coins(nums), -766_335_480);
    // }
}