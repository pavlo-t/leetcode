import {expect} from "chai";

/** Definition for a binary tree node. */
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


// noinspection JSUnusedLocalSymbols,JSUnusedGlobalSymbols
class BSTIterator {
    private stack: TreeNode[] = []

    constructor(root: TreeNode | null) {
        this.pushLeftOnTheStack(root)
    }

    private pushLeftOnTheStack(root: TreeNode | null) {
        let current = root
        while (current != null) {
            this.stack.push(current)
            current = current.left
        }
    }

    next(): number {
        const n = this.stack.pop()
        if (typeof n == "undefined")
            throw new Error("Called .next() on an empty BSTIterator")
        this.pushLeftOnTheStack(n.right)
        return n.val
    }

    hasNext(): boolean {
        return this.stack.length > 0
    }
}

// noinspection JSUnusedLocalSymbols,JSUnusedGlobalSymbols
class BSTIteratorEnqueueAllInConstructor {
    private stack: number[] = []

    constructor(root: TreeNode | null) {
        this.fillStack(root)
    }

    private fillStack(node: TreeNode | null) {
        if (node != null) {
            this.fillStack(node.left)
            this.stack.push(node.val)
            this.fillStack(node.right)
        }
    }

    next(): number {
        const n = this.stack.shift()
        if (typeof n == "undefined")
            throw new Error("Called .next() on an empty BSTIterator")
        return n
    }

    hasNext(): boolean {
        return this.stack.length > 0
    }
}


/**
 * Your BSTIterator object will be instantiated and called as such:
 * var obj = new BSTIterator(root)
 * var param_1 = obj.next()
 * var param_2 = obj.hasNext()
 */
describe("c2020_12_09 Binary Search Tree Iterator", () => {
    function n(v: number, l?: TreeNode | null, r?: TreeNode | null): TreeNode {
        return new TreeNode(v, l, r)
    }

    it("Example 1: (root = [7,3,15,n,n,9,20])", () => {
        const root = n(7, n(3), n(15, n(9), n(20)))
        const bstIterator = new BSTIterator(root)
        expect(bstIterator.next()).to.eql(3)
        expect(bstIterator.next()).to.eql(7)
        expect(bstIterator.hasNext()).to.eql(true)
        expect(bstIterator.next()).to.eql(9)
        expect(bstIterator.hasNext()).to.eql(true)
        expect(bstIterator.next()).to.eql(15)
        expect(bstIterator.hasNext()).to.eql(true)
        expect(bstIterator.next()).to.eql(20)
        expect(bstIterator.hasNext()).to.eql(false)
    })
    it("(root = null", () => {
        const bstIterator = new BSTIterator(null)
        expect(bstIterator.hasNext()).to.eql(false)
    })

    function buildTree(depth: number): TreeNode | null {
        let value = 1

        function loop(depth: number): TreeNode | null {
            if (depth == 0) return null
            else {
                const l = loop(depth - 1)
                const rootValue = value
                value++
                const r = loop(depth - 1)
                return n(rootValue, l, r)
            }
        }

        return loop(depth)
    }

    it("(root = 65535 nodes", () => {
        const root = buildTree(16)
        const bst = new BSTIterator(root)

        let expected = 1
        while (bst.hasNext()) {
            expect(bst.next()).to.eql(expected)
            expected++
        }
        expect(expected).to.eql(65536)
    })
    it("(root = 131071 nodes", () => {
        const root = buildTree(17)
        const bst = new BSTIterator(root)

        let expected = 1
        while (bst.hasNext()) {
            expect(bst.next()).to.eql(expected)
            expected++
        }
        expect(expected).to.eql(131072)
    })
})
