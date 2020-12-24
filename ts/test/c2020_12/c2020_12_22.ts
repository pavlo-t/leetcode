import {expect} from 'chai';

/**
 * ### Balanced Binary Tree
 *
 * {@link https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3577/}
 */
function isBalanced(root: TreeNode | null): boolean {
    const rec = (node: TreeNode | null): number => {
        if (node == null) return 0;
        else {
            const l = rec(node.left);
            const r = rec(node.right);
            if (l < 0 || r < 0 || Math.abs(l - r) > 1) {
                return -1;
            } else {
                return Math.max(l, r) + 1;
            }
        }
    };

    return rec(root) >= 0;
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

describe('c2020_12_22 Balanced Binary Tree', () => {
    type Node = TreeNode | null;
    const N = (v: number, l?: Node, r?: Node): Node => new TreeNode(v, l, r);

    it('Example 1: (root = [3,9,20,null,null,15,7]) -> true', () => {
        const root = N(3, N(9), N(20, N(15), N(7)));
        expect(isBalanced(root)).to.be.true;
    });
    it('Example 2: (root = [1,2,2,3,3,null,null,4,4]) -> false', () => {
        const root = N(1, N(2, N(3, N(4), N(4)), N(3)), N(2));
        expect(isBalanced(root)).to.be.false;
    });
    it('Example 3: (root = []) -> true', () => {
        expect(isBalanced(null)).to.be.true;
    });

    const balancedTree = (from: number, to: number): Node => {
        if (from > to) return null;
        else {
            const mid = from + (to - from) / 2;
            return N(mid, balancedTree(from, mid - 1), balancedTree(mid + 1, to));
        }
    };

    it('(root = 5000 balanced nodes) -> true', () => {
        const root = balancedTree(1, 5000);
        expect(isBalanced(root)).to.be.true;
    });
});
