import {expect} from 'chai';

/**
 * ### Find a Corresponding Node of a Binary Tree in a Clone of That Tree
 *
 * {@link https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3590/}
 */
function getTargetCopy(original: TreeNode | null, cloned: TreeNode | null, target: TreeNode | null): TreeNode | null {
    const stack = [[original, cloned]];

    while (true) {
        const [n, c] = stack.pop()!;
        if (n) {
            if (n == target) {
                return c;
            } else {
                stack.push([n.left, c!.left]);
                stack.push([n.right, c!.right]);
            }
        }
    }
}

// noinspection JSUnusedLocalSymbols
function getTargetCopyBuildPath(original: TreeNode | null, cloned: TreeNode | null, target: TreeNode | null): TreeNode | null {
    type Path = ('l' | 'r')[];
    type Node = TreeNode | null;

    const stack: ([Node, Path])[] = [[original, []]];
    let path: Path;
    let found = false;

    while (!found) {
        const [n, p] = stack.pop()!;
        if (n) {
            if (n == target) {
                path = p;
                found = true;
            } else {
                stack.push([n.left, [...p, 'l']]);
                stack.push([n.right, [...p, 'r']]);
            }
        }
    }

    return path!.reduce((n, c) => {
        if (c === 'l') return n!.left;
        else return n!.right;
    }, cloned);
}

// noinspection JSUnusedLocalSymbols
function getTargetCopyUniqueValues(original: TreeNode | null, cloned: TreeNode | null, target: TreeNode | null): TreeNode | null {
    const stack = [cloned];
    while (true) {
        const n = stack.pop();
        if (n) {
            if (n.val == target!.val) {
                return n;
            } else {
                stack.push(n.left);
                stack.push(n.right);
            }
        }
    }
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

describe('c2021_01_02 Find a Corresponding Node of a Binary Tree in a Clone of That Tree', () => {
    const N = (v: number, l?: TreeNode | null, r?: TreeNode | null): TreeNode | null => new TreeNode(v, l, r);

    it('Example 1: (tree = [7,4,3,null,null,6,19], target = 3) -> 3', () => {
        const target = N(3, N(6), N(19));
        const original = N(7, N(4), target);
        const expected = N(3, N(6), N(19));
        const cloned = N(7, N(4), expected);

        expect(getTargetCopy(original, cloned, target)).to.equal(expected);
    });
    it('Example 2: (tree = [7], target =  7) -> 7', () => {
        const original = N(7);
        const cloned = N(7);

        expect(getTargetCopy(original, cloned, original)).to.equal(cloned);
    });
    it('Example 3: (tree = [8,null,6,null,5,null,4,null,3,null,2,null,1], target = 4) -> 4', () => {
        const target = N(4, null, N(3, null, N(2, null, N(1))));
        const original = N(8, null, N(6, null, N(5, null, target)));
        const expected = N(4, null, N(3, null, N(2, null, N(1))));
        const cloned = N(8, null, N(6, null, N(5, null, expected)));

        expect(getTargetCopy(original, cloned, target)).to.equal(expected);
    });
    it('Example 4: (tree = [1,2,3,4,5,6,7,8,9,10], target = 5) -> 5', () => {
        const target = N(5, N(10));
        const original = N(1, N(2, N(4, N(8), N(9)), target), N(3, N(6), N(7)));
        const expected = N(5, N(10));
        const cloned = N(1, N(2, N(4, N(8), N(9)), expected), N(3, N(6), N(7)));

        expect(getTargetCopy(original, cloned, target)).to.equal(expected);
    });
    it('Example 5: (tree = [1,2,null,3], target = 2) -> 2', () => {
        const target = N(2, N(3));
        const original = N(1, target);
        const expected = N(2, N(3));
        const cloned = N(1, expected);

        expect(getTargetCopy(original, cloned, target)).to.equal(expected);
    });

    it('Work with repeated values: (tree = [1,1,1,1,1,1,1,1,1,1], target = 1) -> 1', () => {
        const target = N(1, N(1));
        const original = N(1, N(1, N(1, N(1), N(1)), target), N(1, N(1), N(1)));
        const expected = N(1, N(1));
        const cloned = N(1, N(1, N(1, N(1), N(1)), expected), N(1, N(1), N(1)));

        expect(getTargetCopy(original, cloned, target)).to.equal(expected);
    });
});