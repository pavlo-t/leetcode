#![allow(dead_code)]

struct Solution;
impl Solution {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let gcd = Self::gcd(p, q);
        if (p / gcd) % 2 == 0 {
            2
        } else if (q / gcd) % 2 == 1 {
            1
        } else { 0 }
    }

    fn gcd(n1: i32, n2: i32) -> i32 {
        let mut a = n1;
        let mut b = n2;
        while a != 0 {
            let tmp = b % a;
            b = a;
            a = tmp;
        }
        b
    }
}

struct SolutionMy;
impl SolutionMy {
    pub fn mirror_reflection(p: i32, q: i32) -> i32 {
        let mut sum = q;
        let mut cnt = 1;

        while sum % p > 0 {
            sum += q;
            cnt += 1;
        }

        if cnt % 2 == 0 {
            2
        } else if (sum / p) % 2 == 1 {
            1
        } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_1() {
        assert_eq!(Solution::mirror_reflection(2, 1), 2);
    }

    #[test]
    fn test_1_0() {
        assert_eq!(Solution::mirror_reflection(1, 0), 0);
    }

    #[test]
    fn test_1_1() {
        assert_eq!(Solution::mirror_reflection(1, 1), 1);
    }
}