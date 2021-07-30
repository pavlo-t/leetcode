#![allow(dead_code)]
/// Beautiful Array
/// ===============
///
/// For some fixed `n`, an array `nums` is _beautiful_ if it is a permutation of the integers `1, 2, ..., n`, such that:
///
/// For every `i < j`, there is no `k` with `i < k < j` such that `nums[k] * 2 = nums[i] + nums[j]`.
///
/// Given `n`, return __any__ beautiful array `nums`. (It is guaranteed that one exists.)
///
/// __Note:__
///
/// `1 <= n <= 1000`
///
/// https://leetcode.com/explore/challenge/card/july-leetcoding-challenge-2021/611/week-4-july-22nd-july-28th/3829/
struct Solution;
impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        use std::mem::swap;

        let mut a = vec![1];
        let mut b = vec![];
        while a.len() < n as usize {
            b.clear();
            for &i in &a {
                if i * 2 - 1 <= n {
                    b.push(i * 2 - 1);
                }
            }
            for &i in &a {
                if i * 2 <= n {
                    b.push(i * 2);
                }
            }
            swap(&mut a, &mut b);
        }
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_is_valid(v: &[i32]) -> Result<(), String> {
        let n = v.len();
        if n == 1 {
            if v[0] == 1 { Ok(()) } else {
                Err(format!("{:?}; WTF", v))
            }
        } else {
            for i in 0..n - 2 {
                for k in i + 1..n - 1 {
                    for j in k + 1..n {
                        if v[k] * 2 == v[i] + v[j] {
                            return Err(format!("{:?}; i={},j={},k={}; {} * 2 == {} + {}", v, i, j, k, v[k], v[i], v[j]));
                        }
                    }
                }
            }
            Ok(())
        }
    }

    #[test] fn n_1() { assert_eq!(check_is_valid(&Solution::beautiful_array(1)), Ok(())); }
    #[test] fn n_2() { assert_eq!(check_is_valid(&Solution::beautiful_array(2)), Ok(())); }
    #[test] fn n_3() { assert_eq!(check_is_valid(&Solution::beautiful_array(3)), Ok(())); }
    #[test] fn n_4() { assert_eq!(check_is_valid(&Solution::beautiful_array(4)), Ok(())); }
    #[test] fn n_5() { assert_eq!(check_is_valid(&Solution::beautiful_array(5)), Ok(())); }
    #[test] fn n_6() { assert_eq!(check_is_valid(&Solution::beautiful_array(6)), Ok(())); }
    //#[test] fn n_1_produces_1()           { assert_eq!(Solution::beautiful_array(1), [1]); }
    //#[test] fn n_2_produces_1_2()         { assert_eq!(Solution::beautiful_array(2), [1, 2]); }
    //#[test] fn n_3_produces_1_3_2()       { assert_eq!(Solution::beautiful_array(3), [1, 3, 2]); }
    //#[test] fn n_4_produces_2_1_4_3()     { assert_eq!(Solution::beautiful_array(4), [2, 1, 4, 3]); }
    //#[test] fn n_5_produces_3_1_2_5_4()   { assert_eq!(Solution::beautiful_array(5), [3, 1, 2, 5, 4]); }
    //#[test] fn n_6_produces_1_5_3_2_6_4() { assert_eq!(Solution::beautiful_array(6), [1, 5, 3, 2, 6, 4]); }
    // 1,3,5,6,4,2

    mod performance {
        use super::*;

        #[test]
        fn n_1000() {
            assert_eq!(check_is_valid(&Solution::beautiful_array(1000)), Ok(()));
        }
    }
}
