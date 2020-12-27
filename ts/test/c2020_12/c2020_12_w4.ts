import {expect} from 'chai';

/**
 * ### Find Nearest Right Node in Binary Tree
 *
 * {@link https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3576/}
 */
function findNearestRightNode(root: TreeNode, u: TreeNode): TreeNode | null {
    const queue = [root, null];

    while (queue.length > 1) {
        const n = queue.shift();
        if (n) {
            if (n.val == u.val) {
                return queue.shift()!;
            } else {
                n.left && queue.push(n.left);
                n.right && queue.push(n.right);
            }
        } else {
            queue.push(null);
        }
    }

    throw new Error('Illegal input');
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

describe('c2020_12_w4 Find Nearest Right Node in Binary Tree', () => {
    type Node = TreeNode | null;
    const N = (v: number, l?: Node, r?: Node): TreeNode => new TreeNode(v, l, r);

    it('Example 1: (root = [1,2,3,null,4,5,6], u = 4) -> 5', () => {
        const root = N(1, N(2, null, N(4)), N(3, N(5), N(6)));
        expect(findNearestRightNode(root, N(4))).to.eql(N(5));
        // Explanation: The nearest node on the same level to the right of node 4 is node 5.
    });
    it('Example 2: (root = [3,null,4,2], u = 2) -> null', () => {
        const u = N(2);
        const root = N(3, null, N(4, u));
        expect(findNearestRightNode(root, u)).to.be.null;
        // Explanation: There are no nodes to the right of 2.
    });
    it('Example 3: (root = [1], u = 1) -> null', () => {
        expect(findNearestRightNode(N(1), N(1))).to.be.null;
    });
    it('Example 4: (root = [3,4,2,null,null,null,1], u = 4) -> 2', () => {
        const u = N(4);
        const expected = N(2, null, N(1));
        const root = N(3, u, expected);
        expect(findNearestRightNode(root, u)).to.eql(expected);
    });

    const buildBST = (left: number, right: number): Node => {
        if (left > right) return null;
        else {
            const mid = left + Math.floor((right - left) / 2);
            return N(mid, buildBST(left, mid - 1), buildBST(mid + 1, right));
        }
    };

    it('(root = 100000 balanced nodes, u = 99998) -> 100000', () => {
        const root = buildBST(1, 100000);
        root && expect(findNearestRightNode(root, N(99998))).to.eql(N(100000));
    });
    it('(root = 100000 balanced nodes, u = 50000) -> null', () => {
        const root = buildBST(1, 100000);
        root && expect(findNearestRightNode(root, N(50000))).to.be.null;
    });
});
