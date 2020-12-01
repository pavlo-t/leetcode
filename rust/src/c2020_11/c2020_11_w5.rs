// @formatter:off
#![allow(dead_code)]
struct Solution;
impl Solution {
    const ERROR_TOLERANCE: f64 = 0.00001;

    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let (min, max) = nums
            .iter()
            .fold((std::i32::MAX, std::i32::MIN), |(min, max), &i| (min.min(i), max.max(i)));

        Self::bs(&nums, k as usize, min as f64, max as f64)
    }

    fn bs(nums: &Vec<i32>, k: usize, l: f64, r: f64) -> f64 {
        if r - l < Self::ERROR_TOLERANCE {
            l
        } else {
            let mid = (l + r) / 2.0;
            if Self::can_get_higher_avg(nums, k, mid) {
                Self::bs(nums, k, mid, r)
            } else {
                Self::bs(nums, k, l, mid)
            }
        }
    }
    fn can_get_higher_avg(nums: &Vec<i32>, k: usize, avg: f64) -> bool {
        let mut sum = 0.0;
        for i in 0..k {
            sum += nums[i] as f64 - avg;
        }
        if sum >= 0.0 {
            true
        } else {
            let mut l_sum: f64 = 0.0;
            let mut min_l_sum: f64 = 0.0;
            for i in k..nums.len() {
                sum += nums[i] as f64 - avg;
                l_sum += nums[i - k] as f64 - avg;
                min_l_sum = min_l_sum.min(l_sum);
                if (sum - min_l_sum) >= 0.0 {
                    return true;
                }
            }
            false
        }
    }
}

struct SolutionBruteForce;
impl SolutionBruteForce {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let mut result = std::f64::MIN;

        for s in 0..=(nums.len() - k as usize) {
            let mut sum = 0;
            for i in s..nums.len() {
                sum += nums[i];
                let l = i - s + 1;
                if l >= k as usize {
                    result = result.max(sum as f64 / l as f64);
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n1_12_m5_m6_50_3_k4_is_12p75() {
        let result = Solution::find_max_average(vec![1,12,-5,-6,50,3], 4);
        let expected = 12.75;
        assert!((result - expected).abs() < Solution::ERROR_TOLERANCE);
        // Explanation:
        //  - When the length is 4, averages are [0.5, 12.75, 10.5] and the maximum average is 12.75
        //  - When the length is 5, averages are [10.4, 10.8] and the maximum average is 10.8
        //  - When the length is 6, averages are [9.16667] and the maximum average is 9.16667
        //  The maximum average is when we choose a subarray of length 4 (i.e., the sub array [12, -5, -6, 50])
        //  which has the max average 12.75, so we return 12.75
        //  Note that we do not consider the subarrays of length < 4.
    }
    #[test]
    fn example2_n5_k1_is_5p0() {
        let result = Solution::find_max_average(vec![5], 1);
        let expected = 5.0;
        assert!((result - expected).abs() < Solution::ERROR_TOLERANCE);
    }

    #[test]
    fn n50_12_m5_m6_50_3_k4_is_20p2() {
        let result = Solution::find_max_average(vec![50,12,-5,-6,50,3], 4);
        let expected = 20.2;
        assert!((result - expected).abs() < Solution::ERROR_TOLERANCE);
    }
    #[test]
    fn n1_20_m5_m6_50_20_k4_is_15p8() {
        let result = Solution::find_max_average(vec![1,20,-5,-6,50,20], 4);
        let expected = 15.8;
        assert!((result - expected).abs() < Solution::ERROR_TOLERANCE);
    }

    #[test]
    fn n_1to10000_k1_is_10000() {
        let nums = (1..=10000).collect();
        let result = Solution::find_max_average(nums, 1);
        let expected = 10000.0;
        assert!((result - expected).abs() < Solution::ERROR_TOLERANCE);
    }
    #[test]
    fn n_1to10000_k2_is_9999p5() {
        let nums = (1..=10000).collect();
        let result = Solution::find_max_average(nums, 2);
        let expected = 9999.5;
        assert!((result - expected).abs() < Solution::ERROR_TOLERANCE);
    }
    #[test]
    fn n_1repeated10000_k2_is_1() {
        let result = Solution::find_max_average(vec![1;10000], 2);
        let expected = 1.0;
        assert!((result - expected).abs() < Solution::ERROR_TOLERANCE);
    }
}
