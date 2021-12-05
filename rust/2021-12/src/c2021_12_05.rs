#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 337. House Robber III
/// =====================
///
/// The thief has found himself a new place for his thievery again.
/// There is only one entrance to this area, called `root`.
///
/// Besides the `root`, each house has one and only one parent house.
/// After a tour, the smart thief realized that all houses in this place form a binary tree.
/// It will automatically contact the police if __two directly-linked houses were broken into on the same night__.
///
/// Given the `root` of the binary tree,
/// return _the maximum amount of money the thief can rob __without alerting the police___.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[1, 10_000]`.
/// - `0 <= Node.val <= 10_000`
///
/// https://leetcode.com/problems/house-robber-iii/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
impl Solution {
    pub fn rob_rec(root: T) -> i32 {
        println!("rob({:?})", root);
        fn rec(t: T, can_rob: bool) -> i32 {
            t.map(|t| {
                let b = t.borrow();
                let skip = rec(b.left.clone(), true) + rec(b.right.clone(), true);
                if can_rob {
                    let rob = b.val + rec(b.left.clone(), false) + rec(b.right.clone(), false);
                    rob.max(skip)
                } else {
                    skip
                }
            })
            .unwrap_or(0)
        }
        rec(root.clone(), true)
    }
    pub fn rob(root: T) -> i32 {
        use std::collections::VecDeque;

        const NO_CHILDREN: [Option<usize>; 2] = [None; 2];

        let mut vals = vec![root.clone().map(|n| n.borrow().val).unwrap()];
        let mut tree = vec![NO_CHILDREN];
        let mut queue = VecDeque::new();
        queue.push_back((root.clone(), 0));
        while let Some((n, i)) = queue.pop_front() {
            let n = n.unwrap();
            let b = n.borrow();
            if b.left.is_some() {
                let l = tree.len();
                queue.push_back((b.left.clone(), l));
                vals.push(b.left.clone().map(|n| n.borrow().val).unwrap());
                tree.push(NO_CHILDREN);
                tree[i][0] = Some(l);
            }
            if b.right.is_some() {
                let r = tree.len();
                queue.push_back((b.right.clone(), r));
                vals.push(b.right.clone().map(|n| n.borrow().val).unwrap());
                tree.push(NO_CHILDREN);
                tree[i][1] = Some(r);
            }
        }

        let n = vals.len();
        // dp: vec[node_idx][cannot_rob_max, can_rob_max]
        let mut dp = vec![[0; 2]; n];
        for i in (0..n).rev() {
            let [l, r] = tree[i];
            let rob_l = l.map(|l| dp[l][1]).unwrap_or(0);
            let rob_r = r.map(|r| dp[r][1]).unwrap_or(0);
            dp[i][0] = rob_l + rob_r;
            let skip_l = l.map(|l| dp[l][0]).unwrap_or(0);
            let skip_r = r.map(|r| dp[r][0]).unwrap_or(0);
            dp[i][1] = dp[i][0].max(vals[i] + skip_l + skip_r);
        }
        dp[0][1]
    }
}

#[rustfmt::skip]
//#[derive(Debug, PartialEq, Eq)]
#[derive(PartialEq, Eq)]
pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
#[rustfmt::skip] impl TreeNode {
    #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
    fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}
use std::fmt;
impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut preorder = vec![self.val.to_string()];
        let mut stack = vec![self.right.clone(), self.left.clone()];
        while let Some(n) = stack.pop() {
            if let Some(n) = n {
                let b = n.borrow();
                preorder.push(b.val.to_string());
                if b.right.is_some() || b.left.is_some() {
                    stack.push(b.right.clone());
                    stack.push(b.left.clone());
                }
            } else {
                preorder.push("n".into());
            }
        }
        f.debug_list().entries(preorder.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const N: T = None;
    #[rustfmt::skip] fn t(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(v: i32            ) -> T { TreeNode::new(v).wrap() }

    #[rustfmt::skip] #[test] fn r_2() { assert_eq!(Solution::rob(l(2)), 2); }

    #[rustfmt::skip] #[test] fn r_12n() { assert_eq!(Solution::rob(t(1, l(2), N)), 2); }
    #[rustfmt::skip] #[test] fn r_1n2() { assert_eq!(Solution::rob(t(1, N, l(2))), 2); }
    #[rustfmt::skip] #[test] fn r_21n() { assert_eq!(Solution::rob(t(2, l(1), N)), 2); }
    #[rustfmt::skip] #[test] fn r_2n1() { assert_eq!(Solution::rob(t(2, N, l(1))), 2); }
    #[rustfmt::skip] #[test] fn r_123() { assert_eq!(Solution::rob(t(1, l(2), l(3))), 5); }
    #[rustfmt::skip] #[test] fn r_623() { assert_eq!(Solution::rob(t(6, l(2), l(3))), 6); }

    //      1
    //    /   \
    //   2     3
    //  / \   / \
    // 4   5 6   7
    #[rustfmt::skip] #[test] fn r_1234567() { assert_eq!(Solution::rob(t(1, t(2,l(4),l(5)), t(3,l(6),l(7)))), 23); }

    #[test]
    fn r_3_2_3_n_3_n_1() {
        let r = t(3, t(2, N, l(3)), t(3, N, l(1)));
        assert_eq!(Solution::rob(r), 7);
        //   3r
        //  / \
        // 2   3
        //  \   \
        //   3r  1r
        // Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.
    }
    #[test]
    fn r_3_4_5_1_3_n_1() {
        let r = t(3, t(4, l(1), l(3)), t(5, N, l(1)));
        assert_eq!(Solution::rob(r), 9);
        //     3
        //    / \
        //   4r  5r
        //  / \   \
        // 1   3   1
        // Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.
    }

    const STACK_SIZE: usize = 2usize.pow(27);

    //#[ignore]
    #[test]
    fn r_16383_nodes_in_a_balanced_tree() {
        fn build_tree(depth: i32) -> T {
            if depth <= 0 {
                N
            } else {
                t(depth, build_tree(depth - 1), build_tree(depth - 1))
            }
        }
        let child = std::thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(move || assert_eq!(Solution::rob(build_tree(14)), 18194))
            .unwrap();
        child.join().unwrap();
    }
    //#[ignore]
    #[test]
    fn r_10000_nodes_in_left_tree() {
        fn build_tree(mut depth: i32) -> T {
            let mut root = N;
            while depth > 0 {
                root = t(depth, root, N);
                depth -= 1;
            }
            root
        }
        let child = std::thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(move || assert_eq!(Solution::rob(build_tree(10_000)), 25_005_000))
            .unwrap();
        child.join().unwrap();
    }
    //#[ignore]
    #[test]
    fn r_10000_nodes_in_right_tree() {
        fn build_tree(mut depth: i32) -> T {
            let mut root = N;
            while depth > 0 {
                root = t(depth, N, root);
                depth -= 1;
            }
            root
        }
        let child = std::thread::Builder::new()
            .stack_size(STACK_SIZE)
            .spawn(move || assert_eq!(Solution::rob(build_tree(10_000)), 25_005_000))
            .unwrap();
        child.join().unwrap();
    }
}
