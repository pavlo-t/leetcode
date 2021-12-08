#![allow(dead_code)]
/// 364. Nested List Weight Sum II
/// ==============================
///
/// You are given a nested list of integers `nestedList`.
/// Each element is either an integer or a list whose elements may also be integers or other lists.
///
/// The __depth__ of an integer is the number of lists that it is inside of.
/// For example, the nested list `[1,[2,2],[[3],2],1]` has each integer's value set to its __depth__.
/// Let `maxDepth` be the __maximum depth__ of any integer.
///
/// The __weight__ of an integer is `maxDepth - (the depth of the integer) + 1`.
///
/// Return _the sum of each integer in `nestedList` multiplied by its __weight___.
///
/// __Constraints:__
///
/// - `1 <= nestedList.length <= 50`
/// - The values of the integers in the nested list is in the range `[-100, 100]`.
/// - The maximum __depth__ of any integer is less than or equal to `50`.
///
/// https://leetcode.com/problems/nested-list-weight-sum-ii/
struct Solution;
impl Solution {
    pub fn depth_sum_inverse(nested_list: Vec<NestedInteger>) -> i32 {
        println!("depth_sum_inverse({:?})", nested_list);
        use NestedInteger::*;

        fn find_max_depth(ns: &[NestedInteger]) -> i32 {
            let mut result = 0;
            let mut s = vec![];
            for n in ns {
                s.push((n, 1));
            }
            while let Some((n, d)) = s.pop() {
                result = result.max(d);
                match n {
                    Int(_) => (),
                    List(vec) => vec.iter().for_each(|n| s.push((n, d + 1))),
                }
            }
            result
        }

        fn find_sum(ns: &[NestedInteger], max_depth: i32) -> i32 {
            let mut result = 0;
            let mut s = vec![];
            for n in ns {
                s.push((n, 1));
            }
            while let Some((n, d)) = s.pop() {
                match n {
                    Int(i) => result += i * (max_depth - d + 1),
                    List(vec) => vec.iter().for_each(|n| s.push((n, d + 1))),
                }
            }
            result
        }
        let max_depth = find_max_depth(&nested_list);
        find_sum(&nested_list, max_depth)
    }
}
//#[derive(Debug, PartialEq, Eq)]
#[derive(PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
impl std::fmt::Debug for NestedInteger {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NestedInteger::Int(i) => write!(f, "{}", i),
            NestedInteger::List(v) => write!(f, "{:?}", v),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] #[inline] fn i(i: i32)                -> NestedInteger { NestedInteger::Int(i) }
    #[rustfmt::skip] #[inline] fn l(v: Vec<NestedInteger>) -> NestedInteger { NestedInteger::List(v) }

    #[test]
    fn l_l_1_1_n_2_l_1_1_n_n() {
        let n = vec![l(vec![i(1), i(1)]), i(2), l(vec![i(1), i(1)])];
        assert_eq!(Solution::depth_sum_inverse(n), 8);
        // nested_list: [[1, 1], 2, [1, 1]]
        // depth:         2  2   1   2  2
        // max_depth: max(2  2   1   2  2) => 2
        // weight:        1  1   2   1  1
        // Explanation: Four 1's with a weight of 1, one 2 with a weight of 2.
        // 1*1 + 1*1 + 2*2 + 1*1 + 1*1 = 8
    }
    #[test]
    fn l_1_l_4_l_6_n_n_n() {
        let n = vec![i(1), l(vec![i(4), l(vec![i(6)])])];
        assert_eq!(Solution::depth_sum_inverse(n), 17);
        // nested_list:  [1, [4, [6]]]
        // depth:         1   2   3
        // max_depth: max(1   2   3) => 3
        // weight:        3   2   1
        // Explanation: One 1 at depth 3, one 4 at depth 2, and one 6 at depth 1.
        // 1*3 + 4*2 + 6*1 = 17
    }
}
