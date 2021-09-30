#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn smallest_range_ii(mut a: Vec<i32>, k: i32) -> i32 {
        if a.len() < 2 {
            return 0;
        }

        a.sort_unstable();

        let mut result = a[a.len() - 1] - a[0];
        let l = a[0] + k;
        let r = a[a.len() - 1] - k;

        for i in 1..a.len() {
            let x = r.max(a[i - 1] + k);
            let y = l.min(a[i] - k);
            result = result.min(x - y);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_a1_k0_is_0() {
        assert_eq!(Solution::smallest_range_ii(vec![1], 0), 0);
        //Explanation: B = [1]
    }

    #[test]
    fn example2_a0_10_k2_is_6() {
        assert_eq!(Solution::smallest_range_ii(vec![0, 10], 2), 6);
        //Explanation: B = [2,8]
    }

    #[test]
    fn example3_a1_3_6_k3_is_3() {
        assert_eq!(Solution::smallest_range_ii(vec![1, 3, 6], 3), 3)
        //Explanation: B = [4,6,3]
    }

    #[test]
    fn a123456_k4_is_4() {
        assert_eq!(Solution::smallest_range_ii(vec![1, 2, 3, 4, 5], 4), 4)
        //Explanation: B = [5,6,7,8,9]
    }
}
