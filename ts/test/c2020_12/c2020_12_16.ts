import {expect} from 'chai';


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

// inorder iteration
function isValidBST(root: TreeNode | null): boolean {
    const stack: TreeNode[] = [];
    let prev = -2147483649; // -2^31 - 1

    while (stack.length > 0 || root != null) {
        while (root != null) {
            stack.push(root);
            root = root.left;
        }

        const curr: TreeNode = stack.pop()!;
        if (curr.val <= prev) {
            return false;
        }
        prev = curr.val;
        root = curr.right;
    }

    return true;
}

const N = (v: number, l?: TreeNode | null, r?: TreeNode | null): TreeNode => new TreeNode(v, l, r);

describe('c2020_12_16 Validate Binary Search Tree', () => {
    it('Example 1: (root = [2,1,3]) -> true', () => {
        const root = N(2, N(1), N(3));
        expect(isValidBST(root)).to.eql(true);
    });
    it('Example 2: (root = [5,1,4,null,null,3,6]) -> false', () => {
        const root = N(5, N(1), N(4, N(3), N(6)));
        expect(isValidBST(root)).to.eql(false);
        //Explanation: The root node's value is 5 but its right child's value is 4.
    });

    it('(root = [1]) -> true', () => {
        expect(isValidBST(N(1))).to.eql(true);
    });

    it('(root = [2147483647]) -> true', () => {
        expect(isValidBST(N(2147483647))).to.eql(true);
    });
    it('(root = [-2147483648]) -> true', () => {
        expect(isValidBST(N(-2147483648))).to.eql(true);
    });
    it('(root = [2147483647,1,null]) -> true', () => {
        expect(isValidBST(N(2147483647, N(1)))).to.eql(true);
    });
    it('(root = [-2147483648,null,1]) -> true', () => {
        expect(isValidBST(N(-2147483648, null, N(1)))).to.eql(true);
    });

    it('(root = [5,1,7,null,null,3,8]) -> false', () => {
        const root = N(5, N(1), N(7, N(3), N(8)));
        expect(isValidBST(root)).to.eql(false);
    });
    it('(root = [4,2,6,1,3,5,7]) -> true', () => {
        const root = N(4, N(2, N(1), N(3)), N(6, N(5), N(7)));
        expect(isValidBST(root)).to.eql(true);
    });

    it('(root = [10000 nodes]) -> true', () => {
        const buildValidBST = (left: number, right: number): TreeNode | null => {
            if (left > right) return null;
            else {
                const mid = left + (right - left) / 2;
                const l = buildValidBST(left, mid - 1);
                const r = buildValidBST(mid + 1, right);
                return N(mid, l, r);
            }
        };

        const root = buildValidBST(1, 10000);

        expect(isValidBST(root)).to.eql(true);
    });
    it('(root = [10000 nodes]) -> false', () => {
        const buildInvalidBST = (left: number, right: number): TreeNode | null => {
            if (left > right) return N(left - 1);
            else {
                const mid = left + (right - left) / 2;
                const l = buildInvalidBST(left, mid - 1);
                const r = buildInvalidBST(mid + 1, right);
                return N(mid, l, r);
            }
        };

        const root = buildInvalidBST(1, 10000);

        expect(isValidBST(root)).to.eql(false);
    });

    it('get maximum recursion stack', () => {
        // noinspection InfiniteRecursionJS
        function computeMaxCallStackSize(): number {
            try {
                return 1 + computeMaxCallStackSize();
            } catch (e) {
                // Call stack overflow
                return 1;
            }
        }

        // noinspection InfiniteRecursionJS
        function computeMaxCallStackSizeTailRec(size: number): number {
            if (size > 100_000) return size;
            try {
                return computeMaxCallStackSizeTailRec(size + 1);
            } catch (e) {
                return size;
            }
        }

        console.log('computeMaxCallStackSize: ' + computeMaxCallStackSize());
        console.log('computeMaxCallStackSizeTailRec: ' + computeMaxCallStackSizeTailRec(1));
    });
});
