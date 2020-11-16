package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/566/week-3-november-15th-november-21st/3532/]]
 */
//noinspection DuplicatedCode
class d2020_11_15 extends AnyWordSpec with Matchers {

  /**
   * === Range Sum of BST ===
   *
   * Given the `root` node of a binary search tree,
   * return ''the sum of values of all nodes with a value in the range'' `[low, high]`.
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[1, 20_000]`.
   *  - `1 <= Node.val <= 100_000`
   *  - `1 <= low <= high <= 100_000`
   *  - All `Node.val` are '''unique'''.
   */
  object Solution {
    def rangeSumBST(root: TreeNode, low: Int, high: Int): Int =
      if (root == null) 0
      else if (root.value < low)
        rangeSumBST(root.right, low, high)
      else if (root.value > high)
        rangeSumBST(root.left, low, high)
      else
        root.value + rangeSumBST(root.left, low, high) + rangeSumBST(root.right, low, high)
  }
  object SolutionBruteForce {
    def rangeSumBST(root: TreeNode, low: Int, high: Int): Int =
      if (root == null) 0
      else if (root.value < low || root.value > high)
        rangeSumBST(root.left, low, high) + rangeSumBST(root.right, low, high)
      else
        root.value + rangeSumBST(root.left, low, high) + rangeSumBST(root.right, low, high)
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right
  }

  import Solution.rangeSumBST

  private def T(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [10,5,15,3,7,null,18], low = 7, high = 15) -> 32" in {
    val root = T(10, T(5, T(3), T(7)), T(15, null, T(18)))
    rangeSumBST(root, 7, 15) shouldBe 32
  }
  "Example 2: (root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10) -> 23" in {
    val root = T(10, T(5, T(3, T(1)), T(7, T(6))), T(15, T(13), T(18)))
    rangeSumBST(root, 6, 10) shouldBe 23
  }

  "Test 13: (root = [18,9,27,6,15,24,30,3,null,12,null,21], low = 18, high = 24) -> 63" in {
    val root = T(18, T(9, T(6, T(3)), T(15, T(12))), T(27, T(24, T(21)), T(30)))
    rangeSumBST(root, 18, 24) shouldBe 63
  }

  private def buildTree(size: Int): TreeNode = {
    if (size == 0) null
    else T(size, buildTree(size / 2), buildTree(size / 2))
  }
  private def size(n: TreeNode): Int =
    if (n == null) 0 else 1 + size(n.left) + size(n.right)

  "(root = [tree max size+], low = 1, high = 100_000) -> 286912" in {
    val root = buildTree(20_000)

    size(root) shouldBe 32767
    rangeSumBST(root, 1, 100_000) shouldBe 286912
  }
  "(root = [chain tree 19_577], low = 1, high = 100_000) -> 191_639_253" in {
    val treeSize = 19_577
    var root = T(1)
    for (i <- 2 to treeSize) root = T(i, root)

    size(root) shouldBe treeSize
    rangeSumBST(root, 1, 100_000) shouldBe 191_639_253
  }
}
