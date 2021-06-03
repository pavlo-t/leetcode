use std::collections::HashSet;

/// # Reordered Power of 2
///
/// Starting with a positive integer `N`, we reorder the digits in any order
/// (including the original order) such that the leading digit is not zero.
///
/// Return `true` if and only if we can do this in a way such that the resulting number is a power of 2.
///
/// __Note:__
///
/// - `1 <= N <= 10^9`
///
/// https://leetcode.com/explore/featured/card/march-leetcoding-challenge-2021/590/week-3-march-15th-march-21st/3679/
struct Solution;
impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut ns: HashSet<Vec<char>> = HashSet::new();
        for i in 0..32 {
            let mut v: Vec<char> = (1 << i).to_string().chars().collect();
            v.sort_unstable();
            ns.insert(v);
        }
        let mut v: Vec<char> = n.to_string().chars().collect();
        v.sort_unstable();
        ns.contains(&v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn example1() { assert!(Solution::reordered_power_of2(1)); }
    #[test] fn example2() { assert!(!Solution::reordered_power_of2(10)); }
    #[test] fn example3() { assert!(Solution::reordered_power_of2(16)); }
    #[test] fn example4() { assert!(!Solution::reordered_power_of2(24)); }
    #[test] fn example5() { assert!(Solution::reordered_power_of2(46)); }
}
