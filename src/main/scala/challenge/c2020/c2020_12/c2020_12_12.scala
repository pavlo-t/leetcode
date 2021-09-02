package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/570/week-2-december-8th-december-14th/3563/]]
 */
//noinspection DuplicatedCode
class c2020_12_12 extends AnyWordSpec with Matchers {

  /**
   * === Smallest Subtree with all the Deepest Nodes ===
   *
   * Given the `root` of a binary tree, the depth of each node is '''the shortest distance to the root'''.
   *
   * Return ''the smallest subtree'' such that it contains '''all the deepest nodes''' in the original tree.
   *
   * A node is called '''the deepest''' if it has the largest depth possible among any node in the entire tree.
   *
   * The '''subtree''' of a node is tree consisting of that node, plus the set of all descendants of that node.
   *
   * '''Note:''' This question is the same as [[1123 https://leetcode.com/problems/lowest-common-ancestor-of-deepest-leaves/]]
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree will be in the range `[1, 500]`.
   *  - `0 <= Node.val <= 500`
   *  - The values of the nodes in the tree are '''unique'''.
   */
  object Solution {
    private case class NodeDepth(node: TreeNode, depth: Int)

    def subtreeWithAllDeepest(root: TreeNode): TreeNode = dfs(root).node

    private def dfs(node: TreeNode): NodeDepth =
      if (node == null) NodeDepth(null, 0)
      else {
        val l = dfs(node.left)
        val r = dfs(node.right)

        if (l.depth > r.depth) NodeDepth(l.node, l.depth + 1)
        else if (l.depth < r.depth) NodeDepth(r.node, r.depth + 1)
        else NodeDepth(node, l.depth + 1)
      }
  }

  // O(n) time; O(n) memory
  object SolutionMyWithMemoization {
    def subtreeWithAllDeepest(root: TreeNode): TreeNode = {
      val cache = collection.mutable.Map[Int, Int]()

      def maxDepth(node: TreeNode): Int =
        if (node == null) 0
        else if (cache.contains(node.value)) cache(node.value)
        else {
          cache(node.value) = 1 + (maxDepth(node.left) max maxDepth(node.right))
          cache(node.value)
        }

      @scala.annotation.tailrec
      def loop(node: TreeNode): TreeNode =
        if (node == null) null
        else {
          val ld = maxDepth(node.left)
          val rd = maxDepth(node.right)
          if (ld == rd) node
          else if (ld > rd) loop(node.left)
          else loop(node.right)
        }

      loop(root)
    }
  }

  // O(n^2) time; O(log n) memory (recursion stack)
  object SolutionMyBruteForce {
    @scala.annotation.tailrec
    def subtreeWithAllDeepest(root: TreeNode): TreeNode =
      if (root == null) null
      else {
        val ld = maxDepth(root.left)
        val rd = maxDepth(root.right)
        if (ld == rd) root
        else if (ld > rd) subtreeWithAllDeepest(root.left)
        else subtreeWithAllDeepest(root.right)
      }

    private def maxDepth(node: TreeNode): Int =
      if (node == null) 0
      else {
        val ld = 1 + maxDepth(node.left)
        val rd = 1 + maxDepth(node.right)
        ld max rd
      }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String = {
      if (left == null && right == null) value.toString
      else s"$value,${if (left == null) "" else left},${if (right == null) "" else right}"
    }
  }

  import Solution.subtreeWithAllDeepest

  private def N(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [3,5,1,6,2,0,8,null,null,7,4]) -> [2,7,4]" in {
    val root = N(3, N(5, N(6), N(2, N(7), N(4))), N(1, N(0), N(8)))
    val expected = root.left.right

    subtreeWithAllDeepest(root) shouldBe expected
    //Explanation: We return the node with value 2, nodes 7 and 4 are the deepest.
    // Notice that nodes 5, 3 and 2 contain the deepest nodes in the tree but node 2 is the smallest subtree among them,
    // so we return it.
  }
  "Example 2: (root = [1]) -> [1]" in {
    val root = N(1)
    subtreeWithAllDeepest(root) shouldBe root
    //Explanation: The root is the deepest node in the tree.
  }
  "Example 3: (root = [0,1,3,null,2]) -> [2]" in {
    val root = N(0, N(1, r = N(2)), N(3))
    val expected = root.left.right

    subtreeWithAllDeepest(root) shouldBe expected
    //Explanation: The deepest node in the tree is 2,
    // the valid subtrees are the subtrees of nodes 2, 1 and 0 but the subtree of node 2 is the smallest.
  }

  "Test 13: (root = [0,1,null,3,2]) -> [1,3,2]" in {
    val root = N(0, N(1, N(3), N(2)))
    val expected = root.left

    subtreeWithAllDeepest(root) shouldBe expected
  }

  "(root = []) -> root" in {
    subtreeWithAllDeepest(null) shouldBe null
  }
  "(root = [511 balanced nodes (2^9)]) -> root" in {
    def buildTree(depth: Int): TreeNode =
      if (depth == 0) null
      else N(depth, buildTree(depth - 1), buildTree(depth - 1))

    val root = buildTree(9)

    subtreeWithAllDeepest(root) shouldBe root
  }
  "(root = [500 nodes in left tree] -> deepest leaf in left tree" in {
    def buildTree(nodes: Int): TreeNode =
      if (nodes == 0) null
      else N(nodes, buildTree(nodes - 1))

    val root = buildTree(500)
    val expected = N(1)

    subtreeWithAllDeepest(root).toString shouldBe expected.toString
  }
  "(root = [500 nodes in right tree] -> deepest leaf in right tree" in {
    def buildTree(nodes: Int): TreeNode =
      if (nodes == 0) null
      else N(nodes, r = buildTree(nodes - 1))

    val root = buildTree(500)
    val expected = N(1)

    subtreeWithAllDeepest(root).toString shouldBe expected.toString
  }
  "(root = [5000 nodes in left tree] -> deepest leaf in left tree" in {
    def buildTree(nodes: Int): TreeNode = {
      val root = N(nodes)
      var current = root
      var v = nodes - 1
      while (v > 0) {
        val next = N(v)
        current.left = next
        current = next
        v -= 1
      }
      root
    }

    val root = buildTree(5000)
    val expected = N(1)

    subtreeWithAllDeepest(root).toString shouldBe expected.toString
  }

}
