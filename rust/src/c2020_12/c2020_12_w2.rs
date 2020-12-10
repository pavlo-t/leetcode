struct Solution;

impl Solution {
    pub fn find_missing_ranges(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<String> {
        fn range_to_string(lower: i32, upper: i32) -> String {
            if lower == upper {
                format!("{}", lower)
            } else {
                format!("{}->{}", lower, upper)
            }
        }

        if nums.is_empty() {
            return vec![range_to_string(lower, upper)];
        }

        let mut result = Vec::new();

        if lower < nums[0] {
            result.push(range_to_string(lower, nums[0] - 1))
        }

        for j in 1..nums.len() {
            let i = j - 1;
            if nums[i] + 1 < nums[j] {
                result.push(range_to_string(nums[i] + 1, nums[j] - 1))
            }
        }

        if nums[nums.len() - 1] < upper {
            result.push(range_to_string(nums[nums.len() - 1] + 1, upper))
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let nums = vec![0, 1, 3, 50, 75];
        let expected =
            vec!["2".to_string(), "4->49".to_string(), "51->74".to_string(), "76->99".to_string()];

        assert_eq!(Solution::find_missing_ranges(nums, 0, 99), expected);
        //Explanation: The ranges are:
        //[2,2] --> "2"
        //[4,49] --> "4->49"
        //[51,74] --> "51->74"
        //[76,99] --> "76->99"
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::find_missing_ranges(vec![], 1, 1), vec!["1".to_string()]);
        //Explanation: The only missing range is [1,1], which becomes "1".
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::find_missing_ranges(vec![], -3, -1), vec!["-3->-1".to_string()]);
        //Explanation: The only missing range is [-3,-1], which becomes "-3->-1".
    }

    #[test]
    fn example4() {
        let expected: Vec<String> = Vec::new();
        assert_eq!(Solution::find_missing_ranges(vec![-1], -1, -1), expected);
        //Explanation: There are no missing ranges since there are no missing numbers.
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::find_missing_ranges(vec![-1], -2, -1), vec!["-2".to_string()]);
    }

    #[test]
    fn test_nums1to100_lm1000000000_u1000000000() {
        let nums = (1..=100).collect();
        let expected = vec!["-1000000000->0".to_string(), "101->1000000000".to_string()];
        assert_eq!(Solution::find_missing_ranges(nums, -1000000000, 1000000000), expected);
    }
}