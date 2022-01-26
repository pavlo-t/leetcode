#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
/// 1305. All Elements in Two Binary Search Trees
/// =============================================
///
/// Given two binary search trees `root1` and `root2`, return
/// _a list containing all the integers from both trees sorted in __ascending__ order_.
///
/// __Constraints:__
///
/// - The number of nodes in each tree is in the range `[0, 5000]`.
/// - `-100_000 <= Node.val <= 100_000`
///
/// https://leetcode.com/problems/all-elements-in-two-binary-search-trees/
struct Solution;
type T = Option<Rc<RefCell<TreeNode>>>;
const N: T = None;
impl Solution {
    pub fn get_all_elements(a: T, b: T) -> Vec<i32> {
        println!("get_all_elements({:?}, {:?})", a, b);
        fn get_tree_node_elements(n: T) -> Vec<i32> {
            if let Some(n) = n {
                let b = n.borrow();
                let mut left = get_tree_node_elements(b.left.clone());
                left.push(b.val);
                let mut right = get_tree_node_elements(b.right.clone());
                left.append(&mut right);

                left
            } else {
                vec![]
            }
        }

        let a = get_tree_node_elements(a);
        let b = get_tree_node_elements(b);

        let (mut ai, mut bi) = (0, 0);
        let mut result = vec![];
        while ai < a.len() && bi < b.len() {
            if a[ai] <= b[bi] {
                result.push(a[ai]);
                ai += 1;
            } else {
                result.push(b[bi]);
                bi += 1;
            }
        }
        while ai < a.len() {
            result.push(a[ai]);
            ai += 1;
        }
        while bi < b.len() {
            result.push(b[bi]);
            bi += 1;
        }

        result
    }
}

#[rustfmt::skip] #[derive(PartialEq, Eq)] pub struct TreeNode { pub val: i32, pub left: T, pub right: T }
impl std::fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut entries = vec![self.val.to_string()];
        const LVL: &str = "∷";
        const NONE: &str = "_";
        let mut curr = vec![];
        let mut next = vec![];
        if self.left.is_some() || self.right.is_some() {
            curr.push(self.right.clone());
            curr.push(self.left.clone());
        }
        while !curr.is_empty() {
            entries.push(LVL.to_string());
            let mut seen_some = false;
            while let Some(n) = curr.pop() {
                if let Some(n) = n {
                    let n = n.borrow();
                    entries.push(n.val.to_string());
                    next.push(n.left.clone());
                    next.push(n.right.clone());
                    if n.left.is_some() || n.right.is_some() {
                        seen_some = true;
                    }
                } else {
                    entries.push(NONE.to_string());
                    next.push(None);
                    next.push(None);
                }
            }
            if seen_some {
                std::mem::swap(&mut curr, &mut next);
                curr.reverse();
            }
        }

        f.debug_list().entries(entries.iter()).finish()
    }
}
#[rustfmt::skip] impl TreeNode {
  #[inline] pub fn new(val: i32) -> Self { TreeNode { val, left: None, right: None } }
  #[inline] fn wrap(self) -> T { Some(Rc::new(RefCell::new(self))) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip] fn t(v: i32, l: T, r: T) -> T { TreeNode { val: v, left: l, right: r }.wrap() }
    #[rustfmt::skip] fn l(v: i32            ) -> T { TreeNode::new(v).wrap() }

    #[test]
    fn a_n_b_n() {
        let a = N;
        let b = N;
        let e: [i32; 0] = [];
        assert_eq!(Solution::get_all_elements(a, b), e);
    }
    #[test]
    fn a_2_1_3_b_n() {
        let a = t(2, l(1), l(3));
        let b = N;
        let e = [1, 2, 3];
        assert_eq!(Solution::get_all_elements(a, b), e);
    }
    #[test]
    fn a_n_b_2_1_3() {
        let a = N;
        let b = t(2, l(1), l(3));
        let e = [1, 2, 3];
        assert_eq!(Solution::get_all_elements(a, b), e);
    }

    #[test]
    fn a_2_1_4_b_1_0_3() {
        let a = t(2, l(1), l(4));
        let b = t(1, l(0), l(3));
        let e = [0, 1, 1, 2, 3, 4];
        assert_eq!(Solution::get_all_elements(a, b), e);
    }
    #[test]
    fn a_1_n_8_b_8_1() {
        let a = t(1, N, l(8));
        let b = t(8, l(1), N);
        let e = [1, 1, 8, 8];
        assert_eq!(Solution::get_all_elements(a, b), e);
    }
}
