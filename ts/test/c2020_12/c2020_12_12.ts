import {expect} from 'chai'

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


function subtreeWithAllDeepest(root: TreeNode | null): TreeNode | null {
    class NodeWithDepth {
        node: TreeNode | null
        depth: number

        constructor(node: TreeNode | null, depth: number) {
            this.node = node
            this.depth = depth
        }
    }

    const dfs = (node: TreeNode | null): NodeWithDepth => {
        if (node == null) return new NodeWithDepth(null, 0)
        else {
            const l = dfs(node.left)
            const r = dfs(node.right)
            if (l.depth > r.depth)
                return new NodeWithDepth(l.node, l.depth + 1)
            else if (l.depth < r.depth)
                return new NodeWithDepth(r.node, r.depth + 1)
            else
                return new NodeWithDepth(node, l.depth + 1)
        }
    }

    return dfs(root).node
}


const N = (v: number, l?: TreeNode | null, r?: TreeNode | null): TreeNode => new TreeNode(v, l, r)

describe('c2020_12_12 Smallest Subtree with all the Deepest Nodes', () => {
    it('Example 1: (root = [3,5,1,6,2,0,8,null,null,7,4]) -> [2,7,4]', () => {
        const expected = N(2, N(7), N(4))
        const root = N(3, N(5, N(6), expected), N(1, N(0), N(8)))

        expect(subtreeWithAllDeepest(root)).to.eql(expected)
        //Explanation: We return the node with value 2, nodes 7 and 4 are the deepest.
        // Notice that nodes 5, 3 and 2 contain the deepest nodes in the tree but node 2 is the smallest subtree among them,
        // so we return it.
    })
    it('Example 2: (root = [1]) -> [1]', () => {
        const root = N(1)
        expect(subtreeWithAllDeepest(root)).to.eql(root)
        //Explanation: The root is the deepest node in the tree.
    })
    it('Example 3: (root = [0,1,3,null,2]) -> [2]', () => {
        const expected = N(2)
        const root = N(0, N(1, null, expected), N(3))

        expect(subtreeWithAllDeepest(root)).to.eql(expected)
        //Explanation: The deepest node in the tree is 2,
        // the valid subtrees are the subtrees of nodes 2, 1 and 0 but the subtree of node 2 is the smallest.
    })

    it('Test 13: (root = [0,1,null,3,2]) -> [1,3,2]', () => {
        const expected = N(1, N(3), N(2))
        const root = N(0, expected)
        expect(subtreeWithAllDeepest(root)).to.eql(expected)
    })

    it('(root = []) -> root', () => {
        expect(subtreeWithAllDeepest(null)).to.eql(null)
    })

    it('(root = [511 balanced nodes (2^9)]) -> root', () => {
        const buildTree = (depth: number): TreeNode | null => {
            if (depth == 0) return null
            else return N(depth, buildTree(depth - 1), buildTree(depth - 1))
        }

        const root = buildTree(9)

        expect(subtreeWithAllDeepest(root)).to.eql(root)
    })
    it('(root = [500 nodes in left tree] -> deepest leaf in left tree', () => {
        const buildTree = (nodes: number): TreeNode | null => {
            if (nodes == 0) return null
            else return N(nodes, buildTree(nodes - 1))
        }

        const root = buildTree(500)
        const expected = N(1)

        expect(subtreeWithAllDeepest(root)).to.eql(expected)
    })
    it('(root = [500 nodes in right tree] -> deepest leaf in right tree', () => {
        const buildTree = (nodes: number): TreeNode | null => {
            if (nodes == 0) return null
            else return N(nodes, null, buildTree(nodes - 1))
        }

        const root = buildTree(500)
        const expected = N(1)

        expect(subtreeWithAllDeepest(root)).to.eql(expected)
    })

    it('(root = [5000 nodes in left tree] -> deepest leaf in left tree', () => {
        const buildTree = (nodes: number): TreeNode | null => {
            if (nodes == 0) return null
            else return N(nodes, buildTree(nodes - 1))
        }

        const root = buildTree(5000)
        const expected = N(1)

        expect(subtreeWithAllDeepest(root)).to.eql(expected)
    })
})
