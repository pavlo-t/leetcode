import {expect} from 'chai';

/**
 * ### Pseudo-Palindromic Paths in a Binary Tree
 *
 * {@link https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3585/}
 */
function pseudoPalindromicPaths(root: TreeNode | null): number {
    const addToPath = (p: number, n: number): number => p ^ (1 << (n - 1));
    const isPPP = (n: number): boolean => (n & (n - 1)) === 0;

    const stack: ([TreeNode, number])[] = [[root!, addToPath(0, root!.val)]];
    let paths = 0;

    while (stack.length > 0) {
        const [n, p] = stack.pop()!;

        if (!n.left && !n.right && isPPP(p)) paths++;
        if (n.left) stack.push([n.left, addToPath(p, n.left.val)]);
        if (n.right) stack.push([n.right, addToPath(p, n.right.val)]);
    }

    return paths;
}

class TreeNode {
    val: number;
    left: TreeNode | null;
    right: TreeNode | null;

    constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
        this.val = (val === undefined ? 0 : val);
        this.left = (left === undefined ? null : left);
        this.right = (right === undefined ? null : right);
    }
}

describe('c2020_12_29 Pseudo-Palindromic Paths in a Binary Tree', () => {
    type Node = TreeNode | null;
    const n = (v: number, l?: Node, r?: Node): Node => new TreeNode(v, l, r);

    it('Example 1: (root = [2,3,1,3,1,null,1]) -> 2', () => {
        const root = n(2, n(3, n(3), n(1)), n(1, null, n(1)));
        expect(pseudoPalindromicPaths(root)).to.eql(2);
        // Explanation:
        //     2
        //   3   1
        //  3 1   1
        // There are three paths going from the root node to leaf nodes: [2,3,3], [2,1,1], [2,3,1].
        // [2,3,3] can be rearranged in [3,2,3] (palindrome).
        // [2,1,1] can be rearranged in [1,2,1] (palindrome).
    });
    it('Example 2: (root = [2,1,1,1,3,null,null,null,null,null,1]) -> 1', () => {
        const root = n(2, n(1, n(1), n(3, null, n(1))), n(1));
        expect(pseudoPalindromicPaths(root)).to.eql(1);
        // Explanation:
        //     2
        //   1   1
        //  1 3
        //     1
        // There are three paths going from the root node to leaf nodes: [2,1,1], [2,1,3,1], [2,1].
        // Among these paths only [2,1,1] can be rearranged in [1,2,1] (palindrome).
    });
    it('Example 3: (root = [9]) -> 1', () => {
        expect(pseudoPalindromicPaths(n(9))).to.eql(1);
    });
});
