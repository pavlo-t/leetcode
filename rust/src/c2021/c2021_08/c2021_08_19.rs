#![allow(dead_code)]
use std::cell::RefCell;
use std::rc::Rc;
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

/// Maximum Product of Splitted Binary Tree
/// =======================================
///
/// Given the `root` of a binary tree, split the binary tree into two subtrees by removing one edge
/// such that the product of the sums of the subtrees is maximized.
///
/// Return _the maximum product of the sums of the two subtrees_.
/// Since the answer may be too large, return it __modulo__ `10^9 + 7`.
///
/// __Note__ that you need to maximize the answer before taking the mod and not after taking it.
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[2, 50_000]`.
/// - `1 <= Node.val <= 10_000`
///
/// https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/615/week-3-august-15th-august-21st/3903/
struct Solution;
impl Solution {
    /// inspired by: Approach 3: Advanced Strategies for Dealing with 32-Bit Integers
    /// https://leetcode.com/problems/maximum-product-of-splitted-binary-tree/solution/
    pub fn max_product(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn post_order(n: &Option<Rc<RefCell<TreeNode>>>, rsf: &mut Vec<i32>) -> i32 {
            if let Some(n) = n {
                let nb = n.borrow();
                let sum = post_order(&nb.left, rsf) + post_order(&nb.right, rsf) + nb.val as i32;
                rsf.push(sum);
                sum
            } else {
                0
            }
        }
        fn modular_multiplication(a: i32, mut b: i32, m: i32) -> i32 {
            let mut product = 0;
            let mut current_sum = a;
            while b > 0 {
                let bit = b & 1;
                b >>= 1;
                if bit == 1 {
                    product += current_sum;
                    product %= m;
                }
                current_sum <<= 1;
                current_sum %= m;
            }
            product
        }
        let mut sums = Vec::new();
        post_order(&root, &mut sums);
        let t = sums.pop().unwrap();
        let (s, _) = sums
            .iter()
            .fold((t, t), |(ps, pd), &s| match (t - s - s).abs() {
                d if d < pd => (s, d),
                _ => (ps, pd),
            });
        // my initial take on modular multiplication:
        //(1..t - s).fold(s, |rsf, _| (rsf + s) % 1_000_000_007)
        modular_multiplication(s, t - s, 1_000_000_007)
    }

    pub fn max_product_1(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn post_order(n: &Option<Rc<RefCell<TreeNode>>>, rsf: &mut Vec<i64>) -> i64 {
            if let Some(n) = n {
                let nb = n.borrow();
                let sum = post_order(&nb.left, rsf) + post_order(&nb.right, rsf) + nb.val as i64;
                rsf.push(sum);
                sum
            } else {
                0
            }
        }
        let mut sums = Vec::new();
        post_order(&root, &mut sums);
        let t = sums.pop().unwrap();
        (sums.into_iter().fold(0, |rsf, s| rsf.max((t - s) * s)) % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type N = Option<Rc<RefCell<TreeNode>>>;
    fn wrap(n: TreeNode) -> N {
        Some(Rc::new(RefCell::new(n)))
    }
    fn nlr(v: i32, l: N, r: N) -> N {
        wrap(TreeNode {
            val: v,
            left: l,
            right: r,
        })
    }
    fn nl(v: i32, l: N) -> N {
        wrap(TreeNode {
            val: v,
            left: l,
            right: None,
        })
    }
    fn nr(v: i32, r: N) -> N {
        wrap(TreeNode {
            val: v,
            left: None,
            right: r,
        })
    }
    fn n(v: i32) -> N {
        wrap(TreeNode::new(v))
    }

    #[test]
    fn example1() {
        let root = nlr(1, nlr(2, n(4), n(5)), nl(3, n(6)));
        assert_eq!(Solution::max_product(root), 110);
        // Explanation: Remove the edge 1-2 and get 2 binary trees with sum 11 and 10.
        // Their product is 110 (11*10)
    }
    #[test]
    fn example2() {
        let root = nl(1, nlr(2, n(3), nlr(4, n(5), n(6))));
        assert_eq!(Solution::max_product(root), 90);
        // Explanation: Remove the edge 2-4 and get 2 binary trees with sum 15 and 6.
        // Their product is 90 (15*6)
    }
    #[test]
    fn example3() {
        let root = nlr(
            2,
            nlr(3, nlr(10, n(5), n(4)), nlr(7, n(11), n(1))),
            nlr(9, n(8), n(6)),
        );
        assert_eq!(Solution::max_product(root), 1025);
    }
    #[test]
    fn example4() {
        let root = nl(1, n(1));
        assert_eq!(Solution::max_product(root), 1);
    }

    mod performance {
        use super::*;

        /// If getting stack overflow:
        ///
        /// ```sh
        /// thread 'c2021::c2021_08::c2021_08_19::tests::performance::test_50000_nodes_in_left' has overflowed its stack
        /// fatal runtime error: stack overflow
        /// ```
        ///
        /// Add `RUST_MIN_STACK=67108864` to env
        #[test]
        fn test_50000_nodes_in_left() {
            let mut root = n(10000);
            for _ in 1..50000 {
                root = nl(10000, root);
            }
            assert_eq!(Solution::max_product(root), 562_500_007);
        }
    }
}
