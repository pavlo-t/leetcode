#![allow(dead_code)]

// @formatter:off
struct Solution;
impl Solution {
   pub fn calculate(s: String) -> i32 {
        let (rsf, a, op, b) = s.chars()
            .fold((0, 0, '+', 0), |(rsf, a, op, b), ch| {
                if let Some(d) = ch.to_digit(10) {
                    (rsf, a, op, b * 10 + d as i32)
                } else if ch != ' ' {
                    match op {
                        '+' => (rsf + a, b, ch, 0),
                        '-' => (rsf + a, -b, ch, 0),
                        '*' => (rsf, a * b, ch, 0),
                        '/' => (rsf, a / b, ch, 0),
                        _ => panic!("Unexpected char: {}", ch)
                    }
                } else { (rsf, a, op, b) }
            });

        rsf + match op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            ch => panic!("Unexpected char: {}", ch)
        }
    }
}

struct SolutionImperative;
//noinspection DuplicatedCode
impl SolutionImperative {
    pub fn calculate(s: String) -> i32 {
        let mut result = 0;
        let mut a = 0;
        let mut b = 0;
        let mut op = '+';

        for ch in s.chars() {
            if let Some(d) = ch.to_digit(10) {
                b = b * 10 + d as i32;
            } else if ch != ' ' {
                match op {
                    '+' => {
                        result += a;
                        a = b;
                        b = 0;
                    }
                    '-' => {
                        result += a;
                        a = -b;
                        b = 0;
                    }
                    '*' => {
                        a *= b;
                        b = 0;
                    }
                    '/' => {
                        a /= b;
                        b = 0;
                    }
                    c => panic!("Unexpected char {}", c)
                }
                op = ch;
            }
        }
        match op {
            '+' => result += a + b,
            '-' => result += a - b,
            '*' => result += a * b,
            '/' => result += a / b,
            c => panic!("Unexpected char {}", c),
        }

        result
    }
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn test_3p2t2_is_7() {
        let s = "3+2*2".to_string();
        assert_eq!(Solution::calculate(s), 7);
    }
    #[test]
    fn test__3d2__is_1() {
        let s = " 3/2 ".to_string();
        assert_eq!(Solution::calculate(s), 1);
    }
    #[test]
    fn test__3p5_d_2__is_5() {
        let s = " 3+5 / 2 ".to_string();
        assert_eq!(Solution::calculate(s), 5);
    }
    #[test]
    fn test__1_0_d_2__is_5() {
        let s = " 1 0 / 2 ".to_string();
        assert_eq!(Solution::calculate(s), 5);
    }
    #[test]
    fn test___is_0() {
        let s = "  ".to_string();
        assert_eq!(Solution::calculate(s), 0);
    }
    #[test]
    fn test__10_d_2__is_5() {
        let s = " 10 / 2 ".to_string();
        assert_eq!(Solution::calculate(s), 5);
    }
    #[test]
    fn test_10t3d6_is_5() {
        let s = "10*3/6".to_string();
        assert_eq!(Solution::calculate(s), 5);
    }
    #[test]
    fn test_10d6t3_is_3() {
        let s = "10/6*3".to_string();
        assert_eq!(Solution::calculate(s), 3);
    }
    #[test]
    fn test_1p10t3d6m4_is_2() {
        let s = "1+10*3/6-4".to_string();
        assert_eq!(Solution::calculate(s), 2);
    }
    #[test]
    fn test_1p10d6t3m5_is_m1() {
        let s = "1 + 10 / 6 * 3 - 5".to_string();
        assert_eq!(Solution::calculate(s), -1);
    }
    #[test]
    fn test_1p10m6t3m5_is_m12() {
        let s = "1 + 10 - 6 * 3 - 5".to_string();
        assert_eq!(Solution::calculate(s), -12);
    }
}
// @formatter:on