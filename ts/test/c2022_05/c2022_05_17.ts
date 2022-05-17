// noinspection DuplicatedCode

import {expect} from "chai";

// Definition for a binary tree node.
class TreeNode {
    val: number
    left: TreeNode | null
    right: TreeNode | null

    constructor(val?: number, left?: TreeNode | null, right?: TreeNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.left = (left === undefined ? null : left)
        this.right = (right === undefined ? null : right)
    }
}

/**
 * \#1379. Find a Corresponding Node of a Binary Tree in a Clone of That Tree
 * ==========================================================================
 *
 * Given two binary trees `original` and `cloned` and given a reference to a node `target` in the original tree.
 *
 * The `cloned` tree is a __copy of__ the `original` tree.
 *
 * Return _a reference to the same node_ in the `cloned` tree.
 *
 * __Note__ that you are __not allowed__ to change any of the two trees
 * or the `target` node and the answer __must be__ a reference to a node in the `cloned` tree.
 *
 * ## Constraints:
 *
 * - The number of nodes in the tree is in the range `[1, 10_000]`.
 * - The values of the nodes of the `tree` are unique.
 * - `target` node is a node from the `original` tree and is not `null`.
 *
 * ## Follow up:
 *
 * Could you solve the problem if repeated values on the tree are allowed?
 *
 * @param original
 * @param cloned
 * @param target
 */
function getTargetCopy(original: TreeNode | null,
                       cloned: TreeNode | null,
                       target: TreeNode | null): TreeNode | null {
    const stackO = [original]
    const stackC = [cloned]
    while (stackO.length > 0) {
        const currO = stackO.pop()!
        const currC = stackC.pop()!
        if (currO == target) {
            return currC
        }
        currO.left !== null && (stackO.push(currO.left) && stackC.push(currC.left))
        currO.right !== null && (stackO.push(currO.right) && stackC.push(currC.right))
    }
    return null
}

describe('1379. Find a Corresponding Node of a Binary Tree in a Clone of That Tree', () => {
    const N = null
    const T = (v: number, l?: TreeNode | null, r?: TreeNode | null) => new TreeNode(v, l, r)

    it('tree = [7,4,3,null,null,6,19], target = 3', () => {
        /** ```text
         *   7                 7
         *  / \               / \
         * 4   3 <- target   4   3 <- expected
         *    / \               / \
         *   6   19            6   19
         * ``` */
        const target = T(3, T(6), T(19))
        const o = T(7, T(4), target)
        const expected = T(3, T(6), T(19))
        const c = T(7, T(4), expected)
        expect(getTargetCopy(o, c, target)).to.equal(expected).and.not.equal(target)
        // Explanation: In all examples the original and cloned trees are shown.
    })

    it('tree = [7], target = 7', () => {
        /** ```text
         *   7 <- target   7 <- expected
         * ``` */
        const target = T(7)
        const expected = T(7)
        expect(getTargetCopy(target, expected, target)).to.equal(expected).and.not.equal(target)
    })

    it('[8,null,6,null,5,null,4,null,3,null,2,null,1], target = 4', () => {
        /** ```text
         * 8                     8
         *  \                     \
         *   6                     6
         *    \                     \
         *     5                     5
         *      \                     \
         *       4 <- target           4 <- expected
         *        \                     \
         *         3                     3
         *          \                     \
         *           2                     2
         *            \                     \
         *             1                     1
         * ``` */
        const target = T(4, N, T(3, N, T(2, N, T(1))))
        const o = T(8, N, T(6, N, T(5, N, target)))
        const expected = T(4, N, T(3, N, T(2, N, T(1))))
        const c = T(8, N, T(6, N, T(5, N, expected)))
        expect(getTargetCopy(o, c, target)).to.equal(expected).and.not.equal(target)
    })

    it('tree = 10000 nodes in left branch, target = bottom leaf', () => {
        const target = T(1)
        const expected = T(1)
        let o = target
        let c = expected
        while (o.val < 10000) {
            o = T(o.val + 1, o)
            c = T(c.val + 1, c)
        }
        expect(getTargetCopy(target, expected, target)).to.equal(expected).and.not.equal(target)
    })
})