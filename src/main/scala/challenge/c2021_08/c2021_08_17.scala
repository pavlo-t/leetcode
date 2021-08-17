package challenge.c2021_08

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

import scala.annotation.tailrec

/** [[https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/615/week-3-august-15th-august-21st/3899/]] */
class c2021_08_17 extends AnyWordSpec with Matchers {
  /**
   * == Count Good Nodes in Binary Tree ==
   *
   * Given a binary tree `root`, a node ''X'' in the tree is named '''good''' if in the path from root to ''X''
   * there are no nodes with a value ''greater than X''.
   *
   * Return the number of '''good''' nodes in the binary tree.
   *
   * '''Constraints:'''
   *  - The number of nodes in the binary tree is in the range `[1, 100_000]`.
   *  - Each node's value is between `[-10_000, 10_000]`.
   */
  object Solution {
    def goodNodes(root: TreeNode): Int = {
      @scala.annotation.tailrec
      def rec(todo: Seq[(TreeNode, Int)], rsf: Int): Int = todo match {
        case Nil                               => rsf
        case (null, _) +: rest                 => rec(rest, rsf)
        case (n, max) +: rest if n.value < max => rec((n.left, max) +: (n.right, max) +: rest, rsf)
        case (n, _) +: rest                    => rec((n.left, n.value) +: (n.right, n.value) +: rest, rsf + 1)
      }
      rec(Seq((root, Int.MinValue)), 0)
    }
    def goodNodes_BruteForce(root: TreeNode): Int = {
      def rec(n: TreeNode, max: Int): Int =
        if (n == null) 0
        else if (n.value < max) rec(n.left, max) + rec(n.right, max)
        else 1 + rec(n.left, n.value) + rec(n.right, n.value)
      rec(root, Int.MinValue)
    }
  }
  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String = s"{$left,$value,$right}"
  }

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [3,1,4,3,null,1,5]) -> 4" in {
    val root = n(3, n(1, n(3)), n(4, n(1), n(5)))
    Solution.goodNodes(root) shouldBe 4
    // Explanation: Nodes in blue are good.
    // Root Node (3) is always a good node.
    // Node 4 -> (3,4) is the maximum value in the path starting from the root.
    // Node 5 -> (3,4,5) is the maximum value in the path
    // Node 3 -> (3,1,3) is the maximum value in the path.
  }
  "Example 2: (root = [3,3,null,4,2]) -> 3" in {
    val root = n(3, n(3, n(4), n(2)))
    Solution.goodNodes(root) shouldBe 3
    // Explanation: Node 2 -> (3, 3, 2) is not good, because "3" is higher than it.
  }
  "Example 3: (root = [1]) -> 1" in {
    Solution.goodNodes(n(1)) shouldBe 1
    // Explanation: Root is considered as good.
  }

  "(100_000 nodes in left branch) -> 100_000" in {
    @tailrec def build(v: Int, rsf: TreeNode = null): TreeNode =
      if (v == 0) rsf
      else build(v - 1, n(v, rsf))

    val root = build(100_000)
    Solution.goodNodes(root) shouldBe 100_000
  }


}
