#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 437. Path Sum III
/// =================
///
/// Given the `root` of a binary tree and an integer `targetSum`,
/// return _the number of paths where the sum of the values along the path equals `targetSum`_.
///
/// The path does not need to start or end at the root or a leaf,
/// but it must go downwards (i.e., traveling only from parent nodes to child nodes).
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 1000]`.
/// - `-10^9 <= Node.val <= 10^9`
/// - `-1000 <= targetSum <= 1000`
///
/// https://leetcode.com/problems/path-sum-iii/
struct Solution;
impl Solution {
    /// 14:48-14:52
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        println!("path_sum({:?}, {})", root, target_sum);
        fn rec(r: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32) -> (Vec<i32>, i32) {
            if let Some(n) = r {
                let n = n.borrow();
                let (l_path_sums, l_target_sums) = rec(n.left.as_ref(), target_sum);
                let (r_path_sums, r_target_sums) = rec(n.right.as_ref(), target_sum);
                let size = 1 + l_path_sums.len() + r_path_sums.len();
                let init = (Vec::with_capacity(size), l_target_sums + r_target_sums);
                std::iter::once(0)
                    .chain(l_path_sums.into_iter())
                    .chain(r_path_sums.into_iter())
                    .map(|s| s + n.val)
                    .fold(init, |(mut path_sums, target_sums), s| {
                        path_sums.push(s);
                        (path_sums, target_sums + (s == target_sum) as i32)
                    })
                //let mut target_sums = l_target_sums + r_target_sums;
                //let path_sums = std::iter::once(0)
                //    .chain(l_path_sums.into_iter())
                //    .chain(r_path_sums.into_iter())
                //    .map(|mut s| {
                //        s += n.val;
                //        #[rustfmt::skip] if s == target_sum { target_sums += 1; };
                //        s
                //    })
                //    .collect();
                //(path_sums, target_sums)
            } else {
                (vec![], 0)
            }
        }
        rec(root.as_ref(), target_sum).1
    }
    /// 14:15-14:42
    pub fn path_sum_rec_1(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        println!("path_sum({:?}, {})", root, target_sum);
        fn rec(r: Option<&Rc<RefCell<TreeNode>>>, target_sum: i32) -> (Vec<i32>, i32) {
            if let Some(n) = r {
                let n = n.borrow();
                let (mut l_path_sums, l_target_sums) = rec(n.left.as_ref(), target_sum);
                let (mut r_path_sums, r_target_sums) = rec(n.right.as_ref(), target_sum);
                let mut path_sums = Vec::with_capacity(l_path_sums.len() + r_path_sums.len() + 1);
                path_sums.push(0);
                path_sums.append(&mut l_path_sums);
                path_sums.append(&mut r_path_sums);
                let mut target_sums = l_target_sums + r_target_sums;
                for i in 0..path_sums.len() {
                    path_sums[i] += n.val;
                    if path_sums[i] == target_sum {
                        target_sums += 1;
                    }
                }
                (path_sums, target_sums)
            } else {
                (vec![], 0)
            }
        }
        rec(root.as_ref(), target_sum).1
    }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type N = Option<Rc<RefCell<TreeNode>>>;
    #[rustfmt::skip] fn wrap(n: TreeNode)       -> N {Some(Rc::new(RefCell::new(n)))}
    #[rustfmt::skip] fn nlr(v: i32, l: N, r: N) -> N {wrap(TreeNode{val:v, left:l   , right:r})}
    #[rustfmt::skip] fn  nl(v: i32, l: N      ) -> N {wrap(TreeNode{val:v, left:l   , right:None})}
    #[rustfmt::skip] fn  nr(v: i32,       r: N) -> N {wrap(TreeNode{val:v, left:None, right:r})}
    #[rustfmt::skip] fn   n(v: i32            ) -> N {wrap(TreeNode{val:v, left:None, right:None})}

    #[test]
    fn r_p10p5m3p3p2np11p3m2np1_ts_8() {
        let r = nlr(10, nlr(5, nlr(3, n(3), n(-2)), nr(2, n(1))), nr(-3, n(11)));
        assert_eq!(Solution::path_sum(r, 8), 3);
        //         10
        //      5     -3
        //   3    2     11
        // 3  -2    1
        // Explanation: The paths that sum to 8: [-3, 11], [5, 3], [5, 2, 1].
    }
    #[test]
    fn r_p5p4p8p11np13p4p7p2nnp5p1_ts_22() {
        #[rustfmt::skip]
        let r = nlr(5, nl(4, nlr(11, n(7), n(2))), nlr(8, n(13), nlr(4, n(5), n(1))));
        assert_eq!(Solution::path_sum(r, 22), 3);
        //        5
        //     4     8
        //  11     13  4
        // 7  2       5 1
        // The paths that sum up to 22: [4,11,7], [5,4,11,2], [5,8,4,5]
    }

    #[rustfmt::skip] #[test] fn r_n_ts_0() { assert_eq!(Solution::path_sum(None, 0), 0); }
    #[rustfmt::skip] #[test] fn r_n_ts_1() { assert_eq!(Solution::path_sum(None, 1), 0); }
    #[rustfmt::skip] #[test] fn r_p1_ts_1() { assert_eq!(Solution::path_sum(n(1), 1), 1); }
    #[rustfmt::skip] #[test] fn r_p1_ts_2() { assert_eq!(Solution::path_sum(n(1), 2), 0); }
    #[rustfmt::skip] #[test] fn r_p0_ts_0() { assert_eq!(Solution::path_sum(n(0), 0), 1); }
    #[rustfmt::skip] #[test] fn r_p0p0p0_ts_0() {
        //  0
        // 0 0
        let r = nlr(0, n(0), n(0));
        assert_eq!(Solution::path_sum(r, 0), 5);
    }
    #[rustfmt::skip] #[test] fn r_p0p0np0_ts_0() {
        //   0
        //  0
        // 0
        let r = nl(0, nl(0, n(0)));
        assert_eq!(Solution::path_sum(r, 0), 6);
    }
    #[rustfmt::skip] #[test] fn r_p0p0p0p0p0p0p0_ts_0() {
        //    0
        //  0   0
        // 0 0 0 0
        let r = nlr(0, nlr(0, n(0), n(0)), nlr(0, n(0), n(0)));
        assert_eq!(Solution::path_sum(r, 0), 17);
    }

    #[test]
    fn r_1000x1_in_left_ts_1() {
        let mut r = n(1);
        for _ in 1..1000 {
            r = nl(1, r);
        }
        assert_eq!(Solution::path_sum(r, 1), 1000);
    }
    #[test]
    fn r_1000x1_balanced_ts_1() {
        fn build(l: usize) -> N {
            if l == 0 {
                None
            } else {
                nlr(1, build(l - 1), build(l - 1))
            }
        }
        let r = build(10);
        assert_eq!(Solution::path_sum(r, 1), 1023);
    }
    #[test]
    fn r_1000x0_in_right_ts_0() {
        let mut r = n(0);
        for _ in 1..1000 {
            r = nl(0, r);
        }
        assert_eq!(Solution::path_sum(r, 0), 500500);
    }
    #[test]
    fn r_1000x0_balanced_ts_0() {
        fn build(l: usize) -> N {
            if l == 0 {
                None
            } else {
                nlr(0, build(l - 1), build(l - 1))
            }
        }
        let r = build(10);
        assert_eq!(Solution::path_sum(r, 0), 9217);
    }
}
