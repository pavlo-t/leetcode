#![allow(dead_code)]
/// Complex Number Multiplication
/// =============================
///
/// A [complex number] can be represented as a string on the form `"real+imaginaryi"` where:
///
/// - `real` is the real part and is an integer in the range `[-100, 100]`.
/// - `imaginary` is the imaginary part and is an integer in the range `[-100, 100]`.
/// - `i^2 == -1`.
///
/// Given two complex numbers `num1` and `num2` as strings,
/// return _a string of the complex number that represents their multiplications_.
///
/// [complex number]:https://en.wikipedia.org/wiki/Complex_number
///
/// __Constraints:__
///
/// - `num1` and `num2` are valid complex numbers.
///
/// https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/616/week-4-august-22nd-august-28th/3917/
struct Solution;
impl Solution {
    pub fn complex_number_multiply(num1: String, num2: String) -> String {
        fn parse_complex(n: String) -> (i32, i32) {
            let i = n.find('+').unwrap();
            let real = n[..i].parse().unwrap();
            let imaginary = n[i + 1..n.len() - 1].parse().unwrap();
            (real, imaginary)
        }
        println!("complex_number_multiply({}, {})", num1, num2);
        let (r1, i1) = parse_complex(num1);
        let (r2, i2) = parse_complex(num2);
        println!("num1: ({}, {}), num2: ({}, {})", r1, i1, r2, i2);
        format!("{}+{}i", r1 * r2 - i1 * i2, r1 * i2 + r2 * i1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_1p1i_1p1i_produces_0p2i() {
        let num1 = "1+1i".to_string();
        let num2 = "1+1i".to_string();
        assert_eq!(Solution::complex_number_multiply(num1, num2), "0+2i");
        // Explanation: (1 + i) * (1 + i) = 1 + i^2 + 2 * i = 2i, and you need convert it to the form of 0+2i.
    }
    #[test]
    fn example2_1pm1i_1pm1i_produces_0pm2i() {
        let num1 = "1+-1i".to_string();
        let num2 = "1+-1i".to_string();
        assert_eq!(Solution::complex_number_multiply(num1, num2), "0+-2i");
        // Explanation: (1 - i) * (1 - i) = 1 + i^2 - 2 * i = -2i, and you need convert it to the form of 0+-2i.
    }

    #[test]
    fn nums_1p2i_3p4i_produces_m5p10i() {
        let num1 = "1+2i".to_string();
        let num2 = "3+4i".to_string();
        assert_eq!(Solution::complex_number_multiply(num1, num2), "-5+10i");
        // Explanation: (1 + 2i) * (3 + 4i) = 3 + 8*i^2 + 10*i = -5+10i
    }
    #[test]
    fn nums_100p100i_m100pm100i_produces_0pm20000i() {
        let num1 = "100+100i".to_string();
        let num2 = "-100+-100i".to_string();
        assert_eq!(Solution::complex_number_multiply(num1, num2), "0+-20000i");
        // Explanation: (100+100i) * (-100-100i) = -10000-10000*i^2-20000*i = 0+-20000i
    }
}
