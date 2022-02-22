#![allow(dead_code)]
/// 171. Excel Sheet Column Number
/// ==============================
///
/// Given a string `columnTitle` that represents the column title as appear in an Excel sheet,
/// return _its corresponding column number_.
///
/// For example:
///
/// ```text
/// A -> 1
/// B -> 2
/// C -> 3
/// ...
/// Z -> 26
/// AA -> 27
/// AB -> 28
/// ...
/// ```
///
/// __Constraints:__
///
/// - `1 <= columnTitle.length <= 7`
/// - `columnTitle` consists only of uppercase English letters.
/// - `columnTitle` is in the range `["A", "FXSHRXW"]`.
///
/// https://leetcode.com/problems/excel-sheet-column-number/
struct Solution;
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        column_title.bytes().fold(0, |r, b| r * 26 + (b - b'A' + 1) as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        assert_eq!(Solution::title_to_number("A".into()), 1);
    }
    #[test]
    fn ab() {
        assert_eq!(Solution::title_to_number("AB".into()), 28);
    }
    #[test]
    fn zy() {
        assert_eq!(Solution::title_to_number("ZY".into()), 701);
    }
    #[test]
    fn fxshrxw() {
        assert_eq!(Solution::title_to_number("FXSHRXW".into()), 2147483647);
        assert_eq!(Solution::title_to_number("FXSHRXW".into()), i32::MAX);
    }
}
