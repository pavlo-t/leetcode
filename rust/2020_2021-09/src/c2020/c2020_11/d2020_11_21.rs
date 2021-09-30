#![allow(dead_code)]
struct Solution;
impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let s = n.to_string();
        let mut result = 0;

        for i in 1..s.len() {
            result += digits.len().pow(i as u32);
        }

        for (i, c) in s.chars().enumerate() {
            if i == s.len() - 1 {
                result += digits.iter().filter(|&d| d.chars().nth(0).unwrap() <= c).count();
            } else {
                let m = digits.len().pow((s.len() - i - 1) as u32);

                result += m * digits.iter().filter(|&d| d.chars().nth(0).unwrap() < c).count();

                if !digits.iter().find(|&d| d.chars().nth(0).unwrap() == c).is_some() {
                    break;
                }
            }
        }

        result as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1357_100() {
        let digits = vec!["1".to_string(), "3".to_string(), "5".to_string(), "7".to_string()];
        assert_eq!(Solution::at_most_n_given_digit_set(digits, 100), 20)
        // Explanation:
        // The 20 numbers that can be written are:
        // 1, 3, 5, 7, 11, 13, 15, 17, 31, 33, 35, 37, 51, 53, 55, 57, 71, 73, 75, 77.
    }

    #[test]
    fn test_149_1000000000() {
        let digits = vec!["1".to_string(), "4".to_string(), "9".to_string()];
        assert_eq!(Solution::at_most_n_given_digit_set(digits, 1_000_000_000), 29523)
        // Explanation:
        // We can write 3 one digit numbers, 9 two digit numbers, 27 three digit numbers,
        // 81 four digit numbers, 243 five digit numbers, 729 six digit numbers,
        // 2187 seven digit numbers, 6561 eight digit numbers, and 19683 nine digit numbers.
        // In total, this is 29523 integers that can be written using the digits array.
    }

    #[test]
    fn test_149_999999999() {
        let digits = vec!["1".to_string(), "4".to_string(), "9".to_string()];
        assert_eq!(Solution::at_most_n_given_digit_set(digits, 999_999_999), 29523)
    }

    #[test]
    fn test_7_8() {
        let digits = vec!["7".to_string()];
        assert_eq!(Solution::at_most_n_given_digit_set(digits, 8), 1)
    }

    #[test]
    fn test_71_10() {
        let digits = vec!["7".to_string(), "1".to_string()];
        assert_eq!(Solution::at_most_n_given_digit_set(digits, 10), 2)
    }

    #[test]
    fn test_71_11() {
        let digits = vec!["7".to_string(), "1".to_string()];
        assert_eq!(Solution::at_most_n_given_digit_set(digits, 11), 3)
    }
}
