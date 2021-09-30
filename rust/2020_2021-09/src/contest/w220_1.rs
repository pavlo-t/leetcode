#![allow(dead_code)]

/// ### 5629. Reformat Phone Number
///
/// https://leetcode.com/contest/weekly-contest-220/problems/reformat-phone-number/
struct Solution;

impl Solution {
    pub fn reformat_number(number: String) -> String {
        let number: String = number.chars().filter(|c| c.is_digit(10)).collect();

        fn hyphenate(chars: core::str::Chars) -> String {
            let cnt = chars.clone().count();
            if cnt <= 3 {
                chars.collect()
            } else if cnt == 4 {
                let mut result = String::with_capacity(5);
                for (i, c) in chars.enumerate() {
                    if i == 2 { result.push('-') };
                    result.push(c);
                }

                result
            } else {
                let mut result = String::with_capacity(cnt + cnt / 3);
                let mut rest = String::with_capacity(cnt + cnt / 3);
                for (i, c) in chars.enumerate() {
                    if i < 3 {
                        result.push(c);
                    } else if i == 3 {
                        result.push('-');
                        rest.push(c);
                    } else {
                        rest.push(c);
                    }
                }
                result.push_str(&hyphenate(rest.chars()));

                result
            }
        }

        hyphenate(number.chars())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(Solution::reformat_number("1-23-45 6".to_string()), "123-456");
        //Explanation: The digits are "123456".
        // Step 1: There are more than 4 digits, so group the next 3 digits. The 1st block is "123".
        // Step 2: There are 3 digits remaining, so put them in a single block of length 3. The 2nd block is "456".
        // Joining the blocks gives "123-456".
    }

    #[test]
    fn example2() {
        assert_eq!(Solution::reformat_number("123 4-567".to_string()), "123-45-67");
        //Explanation: The digits are "1234567".
        // Step 1: There are more than 4 digits, so group the next 3 digits. The 1st block is "123".
        // Step 2: There are 4 digits left, so split them into two blocks of length 2. The blocks are "45" and "67".
        // Joining the blocks gives "123-45-67".
    }

    #[test]
    fn example3() {
        assert_eq!(Solution::reformat_number("123 4-5678".to_string()), "123-456-78");
        //Explanation: The digits are "12345678".
        // Step 1: The 1st block is "123".
        // Step 2: The 2nd block is "456".
        // Step 3: There are 2 digits left, so put them in a single block of length 2. The 3rd block is "78".
        // Joining the blocks gives "123-456-78".
    }

    #[test]
    fn example4() {
        assert_eq!(Solution::reformat_number("12".to_string()), "12");
    }

    #[test]
    fn example5() {
        assert_eq!(Solution::reformat_number("--17-5 229 35-39475 ".to_string()), "175-229-353-94-75");
    }
}
