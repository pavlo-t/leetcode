package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/569/week-1-december-1st-december-7th/3551/]]
 */
//noinspection DuplicatedCode
class c2020_12_01 extends AnyWordSpec with Matchers {

  /**
   * === Maximum Depth of Binary Tree ===
   *
   * Given the `root` of a binary tree, return ''its maximum depth''.
   *
   * A binary tree's '''maximum depth''' is the number of nodes along the longest path from the root node
   * down to the farthest leaf node.
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[0, 10000]`.
   *  - `-100 <= Node.val <= 100`
   */
  object SolutionIterative {
    import collection.mutable

    def maxDepth(root: TreeNode): Int =
      if (root == null) 0
      else {
        val stack = mutable.Stack[(TreeNode, Int)]((root, 1))
        var maxDepth = 0
        while (stack.nonEmpty) {
          val (node, depth) = stack.pop()
          val nDepth = depth + 1
          if (node.left != null) stack.push((node.left, nDepth))
          if (node.right != null) stack.push((node.right, nDepth))
          maxDepth = maxDepth max depth
        }
        maxDepth
      }
  }
  object SolutionTailrecBfsMutable {
    import collection.mutable

    def maxDepth(root: TreeNode): Int = {
      if (root == null) 0
      else {
        val todo = mutable.Queue[(TreeNode, Int)]((root, 1))
        @scala.annotation.tailrec
        def maxDepthTailrec(rsf: Int): Int =
          if (todo.isEmpty) rsf
          else {
            val (node, depth) = todo.dequeue
            val nDepth = depth + 1
            if (node.left != null) todo.enqueue((node.left, nDepth))
            if (node.right != null) todo.enqueue((node.right, nDepth))
            maxDepthTailrec(rsf max depth)
          }

        maxDepthTailrec(0)
      }
    }
  }
  object Solution {
    import collection.immutable.Queue
    import scala.util.chaining.scalaUtilChainingOps

    def maxDepth(root: TreeNode): Int =
      if (root == null) 0
      else maxDepthTailrec(Queue((root, 1)), 0)

    @scala.annotation.tailrec
    private def maxDepthTailrec(todo: Queue[(TreeNode, Int)], rsf: Int): Int =
      if (todo.isEmpty) rsf
      else {
        val ((node, depth), rest) = todo.dequeue
        maxDepthTailrec(enqueueChildren(rest, node, depth), rsf max depth)
      }

    private def enqueueChildren(queue: Queue[(TreeNode, Int)], node: TreeNode, depth: Int): Queue[(TreeNode, Int)] = {
      val nDepth = depth + 1
      def enqueueIfNotNull(queue: Queue[(TreeNode, Int)], node: TreeNode): Queue[(TreeNode, Int)] =
        if (node == null) queue else queue.enqueue((node, nDepth))
      queue
        .pipe(enqueueIfNotNull(_, node.left))
        .pipe(enqueueIfNotNull(_, node.right))
    }
  }
  object SolutionTailrecBfsImmutableSimple {
    import collection.immutable.Queue

    def maxDepth(root: TreeNode): Int =
      maxDepthTailrec(Queue((root, 1)), 0)

    @scala.annotation.tailrec
    private def maxDepthTailrec(todo: Queue[(TreeNode, Int)], rsf: Int): Int =
      if (todo.isEmpty) rsf
      else {
        val ((node, depth), rest) = todo.dequeue
        if (node == null) maxDepthTailrec(rest, rsf)
        else maxDepthTailrec(rest.enqueueAll(Iterable((node.left, depth + 1), (node.right, depth + 1))), rsf max depth)
      }
  }

  object SolutionRecursion {
    def maxDepth(root: TreeNode): Int =
      if (root == null) 0
      else 1 + (maxDepth(root.left) max maxDepth(root.right))
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right
  }

  import Solution.maxDepth

  private def N(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [3,9,20,null,null,15,7]) -> 3" in {
    val root = N(3, N(9), N(20, N(15), N(7)))
    maxDepth(root) shouldBe 3
  }
  "Example 2: (root = [1,null,2]) -> 2" in {
    val root = N(1, r = N(2))
    maxDepth(root) shouldBe 2
  }
  "Example 3: (root = []) -> 0" in {
    maxDepth(null) shouldBe 0
  }
  "Example 4: (root = [0]) -> 1" in {
    val root = N(0)
    maxDepth(root) shouldBe 1
  }

  private def buildTree(nodes: Int): TreeNode = {
    if (nodes < 2) null
    else {
      val n = N(nodes)
      n.left = buildTree(nodes / 2)
      n.right = buildTree(nodes / 2)
      n
    }
  }
  "(root = 10000+ nodes in a balanced tree) -> 13" in {
    val root = buildTree(10000)
    maxDepth(root) shouldBe 13
  }
  "(root = 10000 nodes in the left branch) -> 10000" in {
    val root = N(1)
    var current = root
    for (i <- 2 to 10000) {
      val n = N(i)
      current.left = n
      current = n
    }

    maxDepth(root) shouldBe 10000
  }
  "(root = 10000 nodes in the right branch) -> 10000" in {
    val root = N(1)
    var current = root
    for (i <- 2 to 10000) {
      val n = N(i)
      current.right = n
      current = n
    }

    maxDepth(root) shouldBe 10000
  }

  "(root = 100_000+ nodes in a balanced tree) -> 16" in {
    val root = buildTree(100000)
    maxDepth(root) shouldBe 16
  }
  "(root = 100_000 nodes in the right branch) -> 100_000" in {
    val root = N(1)
    var current = root
    for (i <- 2 to 100000) {
      val n = N(i)
      current.right = n
      current = n
    }

    maxDepth(root) shouldBe 100000
  }
}
