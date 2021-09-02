package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3568/]]
 */
//noinspection DuplicatedCode
class c2020_12_16 extends AnyWordSpec with Matchers {

  /**
   * === Validate Binary Search Tree ===
   *
   * Given the `root` of a binary tree, ''determine if it is a valid binary search tree (BST)''.
   *
   * A '''valid BST''' is defined as follows:
   *  - The left subtree of a node contains only nodes with keys '''less than''' the node's key.
   *  - The right subtree of a node contains only nodes with keys '''greater than''' the node's key.
   *  - Both the left and right subtrees must also be binary search trees.
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[1, 10000]`.
   *  - `-2^31 <= Node.val <= 2^31 - 1`
   */
  object Solution {
    def isValidBST(root: TreeNode): Boolean = {
      def rec(n: TreeNode, min: Long, max: Long): Boolean =
        if (n == null) true
        else if (n.value <= min || n.value >= max) false
        else rec(n.left, min, n.value) && rec(n.right, n.value, max)

      rec(root, Long.MinValue, Long.MaxValue)
    }
  }

  object Solution1 {
    def isValidBST(root: TreeNode): Boolean = {
      def rec(n: TreeNode, min: Long, max: Long): Boolean =
        if (n == null) true
        else if (n.value > min && n.value < max)
          rec(n.left, min, n.value) && rec(n.right, n.value, max)
        else false

      rec(root, Long.MinValue, Long.MaxValue)
    }
  }

  object SolutionMyUgly {
    def isValidBST(root: TreeNode): Boolean = {
      val F = (false, 0, 0)

      def rec(n: TreeNode): (Boolean, Int, Int) =
        (n.left, n.right) match {
          case (null, null) => (true, n.value, n.value)
          case (l, null)    =>
            val (lr, lMin, lMax) = rec(l)
            if (lr && lMax < n.value) (true, lMin, n.value)
            else F
          case (null, r)    =>
            val (rr, rMin, rMax) = rec(r)
            if (rr && n.value < rMin) (true, n.value, rMax)
            else F
          case (l, r)       =>
            val (lr, lMin, lMax) = rec(l)
            if (lr && lMax < n.value) {
              val (rr, rMin, rMax) = rec(r)
              if (rr && n.value < rMin) (true, lMin, rMax)
              else F
            } else F
        }

      rec(root)._1
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

  import Solution.isValidBST

  private def N(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [2,1,3]) -> true" in {
    val root = N(2, N(1), N(3))
    isValidBST(root) shouldBe true
  }
  "Example 2: (root = [5,1,4,null,null,3,6]) -> false" in {
    val root = N(5, N(1), N(4, N(3), N(6)))
    isValidBST(root) shouldBe false
    //Explanation: The root node's value is 5 but its right child's value is 4.
  }

  "(root = [1]) -> true" in {
    val root = N(1)
    isValidBST(root) shouldBe true
  }

  "(root = [Int.MaxValue]) -> true" in {
    isValidBST(N(Int.MaxValue)) shouldBe true
  }
  "(root = [Int.MinValue]) -> true" in {
    isValidBST(N(Int.MinValue)) shouldBe true
  }
  "(root = [Int.MaxValue,1,null]) -> true" in {
    isValidBST(N(Int.MaxValue, N(1))) shouldBe true
  }
  "(root = [Int.MinValue,null,1]) -> true" in {
    isValidBST(N(Int.MinValue, r = N(1))) shouldBe true
  }
  "(root = [5,1,7,null,null,3,8]) -> false" in {
    val root = N(5, N(1), N(7, N(3), N(8)))
    isValidBST(root) shouldBe false
  }

  "(root = [4,2,6,1,3,5,7]) -> true" in {
    val root = N(4, N(2, N(1), N(3)), N(6, N(5), N(7)))
    isValidBST(root) shouldBe true
  }
  "(root = [10000+ nodes]) -> true" in {
    def buildValidBST(left: Int, right: Int): TreeNode = {
      if (left > right) null
      else {
        val mid = left + (right - left) / 2
        val l = buildValidBST(left, mid - 1)
        val r = buildValidBST(mid + 1, right)
        N(mid, l, r)
      }
    }

    val root = buildValidBST(1, 10000)

    isValidBST(root) shouldBe true
  }
  "(root = [10000+ nodes]) -> false" in {
    def buildInvalidBST(left: Int, right: Int): TreeNode = {
      if (left > right) N(left - 1)
      else {
        val mid = left + (right - left) / 2
        val l = buildInvalidBST(left, mid - 1)
        val r = buildInvalidBST(mid + 1, right)
        N(mid, l, r)
      }
    }

    val root = buildInvalidBST(1, 10000)

    isValidBST(root) shouldBe false
  }

}
