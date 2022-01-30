#![allow(dead_code)]
/// 421. Maximum XOR of Two Numbers in an Array
/// ===========================================
///
/// Given an integer array `nums`, return _the maximum result of `nums[i] XOR nums[j]`, where `0 <= i <= j < n`_.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 200_000`
/// - `0 <= nums[i] <= 2**31 - 1`
///
/// https://leetcode.com/problems/maximum-xor-of-two-numbers-in-an-array/
struct Solution;
impl Solution {
    pub fn find_maximum_xor_binary_search_wrong(nums: Vec<i32>) -> i32 {
        //println!("find_maximum_xor({:?})", nums);
        //nums.iter().for_each(|&n| println!("{}: {:8.0b}", n, n));
        use std::collections::HashSet;

        if nums.len() == 1 {
            return 0;
        }

        fn can_get(e: i32, msb: i32, nums: &HashSet<i32>) -> bool {
            for &a in nums.iter().filter(|&&n| n & msb != 0) {
                if nums.contains(&(a ^ e)) {
                    return true;
                }
            }
            false
        }

        let (msb, max) = {
            let mut max = nums.iter().max().map(|&n| n).unwrap();
            let (mut msb, mut max_result) = (1, 1);
            while max > 1 {
                max >>= 1;
                msb <<= 1;
                max_result = (max_result << 1) + 1;
            }
            (msb, max_result)
        };
        //println!("msb: {:8.0b}", msb);
        //println!("max: {:8.0b}", max);

        let nums_set = nums.iter().map(|&n| n).collect::<HashSet<_>>();

        let (mut l, mut r) = (0, max);
        while l < r {
            let m = l + (r - l) / 2 + 1;
            if can_get(m, msb, &nums_set) {
                l = m;
            } else {
                r = m - 1;
            }
        }
        l
    }

    pub fn find_maximum_xor_brute_force(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                result = result.max(nums[i] ^ nums[j]);
            }
        }
        result
    }

    /// https://www.geeksforgeeks.org/maximum-xor-of-two-numbers-in-an-array/
    pub fn find_maximum_xor_geeksforgeeks(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut maxx = 0;
        let mut mask = 0;
        let mut se: HashSet<i32> = HashSet::new();

        for i in (0..=30).rev() {
            // set the i'th bit in mask, like 100000, 110000, 111000..
            mask |= 1 << i;
            for &n in &nums {
                // Just keep the prefix till
                // i'th bit neglecting all
                // the bit's after i'th bit
                se.insert(n & mask);
            }
            let new_max = maxx | (1 << i);

            for &prefix in &se {
                // find two pair in set such that a^b = newMaxx, which is the highest possible bit can be obtained
                if se.contains(&(new_max ^ prefix)) {
                    maxx = new_max;
                    break;
                }
            }

            // clear the set for next
            // iteration
            se.clear();
        }

        maxx
    }

    /// https://www.geeksforgeeks.org/maximum-xor-of-two-numbers-in-an-array/
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        use std::collections::HashSet;

        let mut result = 0;
        let mut mask = 0;
        let mut set = HashSet::new();

        for i in (0..=30).rev() {
            mask |= 1 << i;
            for &n in &nums {
                set.insert(n & mask);
            }
            let candidate = result | (1 << i);
            for &pref in &set {
                if set.contains(&(candidate ^ pref)) {
                    result = candidate;
                    break;
                }
            }
            set.clear();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn n_1() { assert_eq!(Solution::find_maximum_xor(vec![1]), 0); }
    #[rustfmt::skip] #[test] fn n_1_2() { assert_eq!(Solution::find_maximum_xor(vec![1, 2]), 3); }
    #[test]
    fn n_1_2_3() {
        assert_eq!(Solution::find_maximum_xor(vec![1, 2, 3]), 3);
        // 1:  1
        // 2: 10
        // 3: 11
        // 1^2=3: 11
    }
    #[test]
    fn n_1_2_3_4() {
        assert_eq!(Solution::find_maximum_xor(vec![1, 2, 3, 4]), 7);
        // 1:   1
        // 2:  10
        // 3:  11
        // 4: 100
        // 3^4=7: 111
    }
    #[test]
    fn n_1_2_3_4_5() {
        assert_eq!(Solution::find_maximum_xor(vec![1, 2, 3, 4, 5]), 7);
        // 1:   1
        // 2:  10
        // 3:  11
        // 4: 100
        // 5: 101
        // 3^4=7 || 2^5=7: 111
    }

    #[test]
    fn n_3_10_5_25_2_8() {
        let n = vec![3, 10, 5, 25, 2, 8];
        assert_eq!(Solution::find_maximum_xor(n), 28);
        //  3:    11
        // 10:  1010
        //  5:   101
        // 25: 11001
        //  2:    10
        //  8:  1000
        //
        // 28: 11100
        //
        // Explanation: The maximum result is 5 XOR 25 = 28.
    }
    #[test]
    fn n_14_70_53_83_49_91_36_80_92_51_66_70() {
        let n = vec![14, 70, 53, 83, 49, 91, 36, 80, 92, 51, 66, 70];
        assert_eq!(Solution::find_maximum_xor(n), 127);
        //  14:    1110
        //  70: 1000110
        //  53:  110101
        //  83: 1010011
        //  49:  110001
        //  91: 1011011
        //  36:  100100
        //  80: 1010000
        //  92: 1011100
        //  51:  110011
        //  66: 1000010
        //  70: 1000110
        //
        // 127: 1111111
        //
        // 91 XOR 36 = 127
    }

    #[test]
    fn n_0_i32_max() {
        assert_eq!(Solution::find_maximum_xor(vec![0, i32::MAX]), i32::MAX);
    }

    //#[ignore]
    #[test]
    fn n_0_until_200_000() {
        assert_eq!(Solution::find_maximum_xor((0..200_000).collect()), 262_143);
    }
}
