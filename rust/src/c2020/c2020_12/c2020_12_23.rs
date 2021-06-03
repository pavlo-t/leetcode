#![allow(dead_code)]

/// ### Next Greater Element III
///
/// https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3578/
struct Solution;

//noinspection DuplicatedCode
impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        if n < 12 {
            -1
        } else {
            let mut bytes = n.to_string().bytes().collect::<Vec<_>>();

            for r in (1..bytes.len()).rev() {
                let l = r - 1;
                if bytes[l] < bytes[r] {
                    let s = bytes.iter().rposition(|&b| bytes[l] < b).unwrap();
                    bytes.swap(l, s);

                    (&mut bytes[r..]).sort();

                    return String::from_utf8(bytes).expect("Illegal bytes")
                        .parse().unwrap_or(-1);
                }
            }

            -1
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_n12_is_21() { assert_eq!(Solution::next_greater_element(12), 21); }

    #[test]
    fn example2_n21_is_m1() { assert_eq!(Solution::next_greater_element(21), -1); }

    #[test]
    fn n21321_is_22113() { assert_eq!(Solution::next_greater_element(21321), 22113); }

    #[test]
    fn n2147483647_is_m1() { assert_eq!(Solution::next_greater_element(2147483647), -1); }
}
