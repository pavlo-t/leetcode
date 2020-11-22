#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn get_smallest_string(n: i32, k: i32) -> String {
        let mut z_s: i32 = k / 26;
        while k - z_s * 26 < n - z_s {
            z_s -= 1;
        }

        let mut mid_char: i32 = 0;

        while k - z_s * 26 - mid_char > n - z_s - (if mid_char > 0 { 1 } else { 0 }) {
            mid_char += 1;
        }

        let a_s: i32 = n - z_s - (if mid_char == 0 { 0 } else { 1 });

        let mut s = String::new();

        if a_s > 0 { s.push_str(&std::iter::repeat('a').take(a_s as usize).collect::<String>()); }
        if mid_char > 0 { s.push((mid_char as u8 + 96) as char) }
        if z_s > 0 { s.push_str(&std::iter::repeat('z').take(z_s as usize).collect::<String>()); }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3_27_aay() {
        assert_eq!(Solution::get_smallest_string(3, 27), "aay");
    }

    #[test]
    fn test_5_73_aaszz() {
        assert_eq!(Solution::get_smallest_string(5, 73), "aaszz");
    }

    #[test]
    fn test_1_1_a() {
        assert_eq!(Solution::get_smallest_string(1, 1), "a");
    }

    #[test]
    fn test_1_2_b() {
        assert_eq!(Solution::get_smallest_string(1, 2), "b");
    }

    #[test]
    fn test_1_26_z() {
        assert_eq!(Solution::get_smallest_string(1, 26), "z");
    }
}