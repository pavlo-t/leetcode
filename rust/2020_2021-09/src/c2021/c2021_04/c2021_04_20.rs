#![allow(dead_code)]

struct Node {
    value: i32,
    children: Vec<Node>,
}

/// N-ary Tree Preorder Traversal
/// =============================
///
/// Given the `root` of an n-ary tree, return _the preorder traversal of its nodes' values_.
///
/// Nary-Tree input serialization is represented in their level order traversal.
/// Each group of children is separated by the null value (See examples)
///
/// __Constraints:__
///
/// - The number of nodes in the tree is in the range `[0, 10_000]`.
/// - `0 <= Node.val <= 10_000`
/// - The height of the n-ary tree is less than or equal to `1000`.
///
/// __Follow up:__ Recursive solution is trivial, could you do it iteratively?
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/595/week-3-april-15th-april-21st/3714/
struct Solution;
impl Solution {
    pub fn preorder_iter(root: Node) -> Vec<i32> {
        let mut result = Vec::new();
        let mut ns = vec![&root];
        while let Some(n) = ns.pop() {
            result.push(n.value);
            for c in n.children.iter().rev() {
                ns.push(c);
            }
        }
        result
    }

    pub fn preorder(root: Node) -> Vec<i32> {
        let mut result = vec![root.value];
        for c in root.children {
            result.append(&mut Self::preorder(c));
        }
        result
    }

    pub fn preorder_rec_1(root: Node) -> Vec<i32> {
        let mut result = Vec::new();
        fn rec(root: &Node, result: &mut Vec<i32>) {
            result.push(root.value);
            for n in root.children.iter() {
                rec(n, result);
            }
        }
        rec(&root, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn n(v: i32) -> Node {
        Node {
            value: v,
            children: Vec::new(),
        }
    }
    fn ns(v: i32, ns: Vec<Node>) -> Node {
        Node {
            value: v,
            children: ns,
        }
    }

    #[test]
    fn example1_r1n324n56_produces_135624() {
        let root = ns(1, vec![ns(3, vec![n(5), n(6)]), n(2), n(4)]);
        assert_eq!(Solution::preorder(root), vec![1, 3, 5, 6, 2, 4]);
    }
    #[test]
    fn example2_r1n2345nn67n8n9_10nn11n12n13nn14_produces_1_2_3_6_7_11_14_4_8_12_5_9_13_10() {
        let root = ns(
            1,
            vec![
                n(2),
                ns(3, vec![n(6), ns(7, vec![ns(11, vec![n(14)])])]),
                ns(4, vec![ns(8, vec![n(12)])]),
                ns(5, vec![ns(9, vec![n(13)]), n(10)]),
            ],
        );
        let e = vec![1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10];
        assert_eq!(Solution::preorder(root), e);
    }
}
