#![allow(dead_code)]
/// Super Palindromes
/// =================
///
/// Let's say a positive integer is a __super-palindrome__ if it is a palindrome,
/// and it is also the square of a palindrome.
///
/// Given two positive integers `left` and `right` represented as strings,
/// return _the number of __super-palindromes__ integers in the inclusive range `[left, right]`_.
///
/// __Constraints:__
///
/// - `1 <= left.length, right.length <= 18`
/// - `left` and `right` consist of only digits.
/// - `left` and `right` cannot have leading zeros.
/// - `left` and `right` represent integers in the range [1, 10^18 - 1].
/// - `left` is less than or equal to `right`.
///
/// https://leetcode.com/explore/challenge/card/may-leetcoding-challenge-2021/599/week-2-may-8th-may-14th/3736/
struct Solution;
impl Solution {
    pub fn superpalindromes_in_range(left: String, right: String) -> i32 {
        fn is_palindrome(s: &[u8]) -> bool {
            let mut l = 0;
            let mut r = s.len() - 1;
            while l < r {
                if s[l] != s[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }

        let l = left.parse::<f64>().unwrap().sqrt();
        let l = l.max(l.ceil()) as i64;
        let r = right.parse::<f64>().unwrap().sqrt().trunc() as i64;

        let mut result = 0;
        let mut increase_result = |palindrome: &str| -> bool {
            if let Ok(i) = palindrome.parse::<i64>() {
                if i >= l && i <= r {
                    let p2s = i.pow(2).to_string();
                    if is_palindrome(p2s.as_bytes()) {
                        result += 1;
                    }
                } else if i > r {
                    return false;
                }
            }
            true
        };

        for left_half_of_palindrome in 1..100000 {
            let s = left_half_of_palindrome.to_string();
            let bs = s.as_bytes();

            let r = bs.iter().rev().skip(1).map(|&u| u).collect::<Vec<_>>();
            let mut p = s.clone();
            p.push_str(&String::from_utf8(r).unwrap());
            if !increase_result(&p) {
                return result;
            }

            let r = bs.iter().rev().map(|&u| u).collect::<Vec<_>>();
            let mut p = s.clone();
            p.push_str(&String::from_utf8(r).unwrap());
            increase_result(&p);
        }
        result
    }

    pub fn superpalindromes_in_range_dictionary(left: String, right: String) -> i32 {
        let store: [u64; 70] = [
            1,
            4,
            9,
            121,
            484,
            10201,
            12321,
            14641,
            40804,
            44944,
            1002001,
            1234321,
            4008004,
            100020001,
            102030201,
            104060401,
            121242121,
            123454321,
            125686521,
            400080004,
            404090404,
            10000200001,
            10221412201,
            12102420121,
            12345654321,
            40000800004,
            1000002000001,
            1002003002001,
            1004006004001,
            1020304030201,
            1022325232201,
            1024348434201,
            1210024200121,
            1212225222121,
            1214428244121,
            1232346432321,
            1234567654321,
            4000008000004,
            4004009004004,
            100000020000001,
            100220141022001,
            102012040210201,
            102234363432201,
            121000242000121,
            121242363242121,
            123212464212321,
            123456787654321,
            400000080000004,
            10000000200000001,
            10002000300020001,
            10004000600040001,
            10020210401202001,
            10022212521222001,
            10024214841242001,
            10201020402010201,
            10203040504030201,
            10205060806050201,
            10221432623412201,
            10223454745432201,
            12100002420000121,
            12102202520220121,
            12104402820440121,
            12122232623222121,
            12124434743442121,
            12321024642012321,
            12323244744232321,
            12343456865434321,
            12345678987654321,
            40000000800000004,
            40004000900040004,
        ];

        let lr = left.parse::<u64>().unwrap();
        let rr = right.parse::<u64>().unwrap();

        store.iter().filter(|&&x| x >= lr && x <= rr).count() as i32
    }

    pub fn superpalindromes_in_range_brutish_force(left: String, right: String) -> i32 {
        const MAX: i64 = 1_000_000_000;

        fn is_palindrome(s: &[u8]) -> bool {
            let mut l = 0;
            let mut r = s.len() - 1;
            while l < r {
                if s[l] != s[r] {
                    return false;
                }
                l += 1;
                r -= 1;
            }
            true
        }
        let left = left.parse::<f64>().unwrap().sqrt();
        // let mut l = if left == left.trunc() { left } else { left + 1.0 } as i64;
        let mut l = left.max(left.ceil()) as i64;
        let r = right.parse::<f64>().unwrap().sqrt().trunc() as i64;

        println!("l:{},r:{}", l, r);
        let mut result = 0;
        while l <= r {
            if is_palindrome(l.to_string().as_bytes())
                && is_palindrome(l.pow(2).to_string().as_bytes())
            {
                // println!("{}->{}", l, l.pow(2));
                result += 1;
            }
            l += 1;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_l_4_r_1000_produces_4() {
        let left = "4".to_string();
        let right = "1000".to_string();
        assert_eq!(Solution::superpalindromes_in_range(left, right), 4);
        // Explanation: 4, 9, 121, and 484 are superpalindromes.
        // Note that 676 is not a superpalindrome: 26 * 26 = 676, but 26 is not a palindrome.
    }
    #[test]
    fn example2_l_1_r_2_produces_1() {
        let left = "1".to_string();
        let right = "2".to_string();
        assert_eq!(Solution::superpalindromes_in_range(left, right), 1);
    }

    #[test]
    fn l_1_r_1_produces_1() {
        let left = "1".to_string();
        let right = "1".to_string();
        assert_eq!(Solution::superpalindromes_in_range(left, right), 1);
    }

    #[test]
    fn l_1_r_10pow12_produces_26() {
        let left = "1".to_string();
        let right = "1000000000000".to_string();
        assert_eq!(Solution::superpalindromes_in_range(left, right), 26);
    }
    #[test]
    fn l_1_r_10pow18min1_produces_70() {
        let left = "1".to_string();
        let right = "999999999999999999".to_string();
        assert_eq!(Solution::superpalindromes_in_range(left, right), 70);
    }
}
