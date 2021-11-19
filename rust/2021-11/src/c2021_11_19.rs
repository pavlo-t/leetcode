#![allow(dead_code)]
/// 461. Hamming Distance
/// =====================
///
/// The [Hamming distance](https://en.wikipedia.org/wiki/Hamming_distance)
/// between two integers is the number of positions at which the corresponding bits are different.
///
/// Given two integers `x` and `y`, return _the __Hamming distance__ between them_.
///
/// __Constraints:__
///
/// - `0 <= x, y <= 2**31 - 1`
///
/// https://leetcode.com/problems/hamming-distance/
struct Solution;
impl Solution {
    pub fn hamming_distance_my(x: i32, y: i32) -> i32 {
        println!("hamming_distance({}, {})", x, y);
        (0..32).map(|s| 1 << s).filter(|b| x & b != y & b).count() as i32
    }
    /// Approach 1: Built-in BitCounting Functions
    /// https://leetcode.com/problems/hamming-distance/solution/
    pub fn hamming_distance_leetcode(x: i32, y: i32) -> i32 {
        println!("hamming_distance({}, {})", x, y);
        (x ^ y).count_ones() as i32
    }
    /// Approach 3: Brian Kernighan's Algorithm
    /// https://leetcode.com/problems/hamming-distance/solution/
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        println!("hamming_distance({}, {})", x, y);
        let mut xor = x ^ y;
        let mut distance = 0;
        while xor != 0 {
            distance += 1;
            // remove the rightmost bit of '1'
            xor = xor & (xor - 1);
        }
        distance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[test] fn x0y0() { assert_eq!(Solution::hamming_distance(0, 0), 0); }
    #[rustfmt::skip] #[test] fn x1y0() { assert_eq!(Solution::hamming_distance(1, 0), 1); }
    #[rustfmt::skip] #[test] fn x0y1() { assert_eq!(Solution::hamming_distance(0, 1), 1); }
    #[test]
    fn x1y4() {
        assert_eq!(Solution::hamming_distance(1, 4), 2);
        // Explanation:
        // 1   (0 0 0 1)
        // 4   (0 1 0 0)
        //        ↑   ↑
        // The above arrows point to positions where the corresponding bits are different.
    }
    #[test]
    fn x3y1() {
        assert_eq!(Solution::hamming_distance(3, 1), 1);
    }
}
