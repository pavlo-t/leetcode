package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3577/]]
 */
//noinspection DuplicatedCode
class c2020_12_22 extends AnyWordSpec with Matchers {
  /**
   * ===  Balanced Binary Tree ===
   *
   * Given a binary tree, determine if it is height-balanced.
   *
   * For this problem, a height-balanced binary tree is defined as:
   *
   * {{{ a binary tree in which the left and right subtrees of every node differ in height by no more than 1. }}}
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[0, 5000]`.
   *  - `-10_000 <= Node.val <= 10_000`
   */
  object Solution {
    def isBalanced(root: TreeNode): Boolean = {
      def rec(node: TreeNode): Int = {
        if (node == null) 0
        else {
          val l = rec(node.left)
          if (l < 0) -1
          else {
            val r = rec(node.right)
            if (r < 0 || (l - r).abs > 1) -1
            else (l max r) + 1
          }
        }
      }
      rec(root) >= 0
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String =
      if (left == null && right == null) value.toString
      else s"${if (left == null) "n" else left},$value,${if (right == null) "n" else right}"
  }

  import Solution.isBalanced

  private def N(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [3,9,20,null,null,15,7]) -> true" in {
    val root = N(3, N(9), N(20, N(15), N(7)))
    isBalanced(root) shouldBe true
  }
  "Example 2: (root = [1,2,2,3,3,null,null,4,4]) -> false" in {
    val root =
      N(1,
        N(2, N(3, N(4), N(4)), N(3)),
        N(2))
    isBalanced(root) shouldBe false
  }
  "Example 3: (root = []) -> true" in {
    isBalanced(null) shouldBe true
  }

  "Test 134: (root = [1,2,2,3,null,null,3,4,null,null,4]) -> false" in {
    val root = N(1, N(2, N(3, N(4))), N(2, r = N(3, r = N(4))))
    isBalanced(root) shouldBe false
  }

  "(root = 10_000 balanced nodes) -> true" in {
    def buildTree(from: Int, to: Int): TreeNode = {
      if (from > to) null
      else {
        val mid = from + (to - from) / 2
        val l = buildTree(from, mid - 1)
        val r = buildTree(mid + 1, to)
        N(mid, l, r)
      }
    }

    val root = buildTree(1, 10000)
    isBalanced(root) shouldBe true
  }
}
