package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3626/]]
 */
//noinspection DuplicatedCode
class c2021_02_02 extends AnyWordSpec with Matchers {
  /**
   * === Trim a Binary Search Tree ===
   *
   * Given the `root` of a binary search tree and the lowest and highest boundaries as `low` and `high`,
   * trim the tree so that all its elements lies in `[low, high]`.
   * Trimming the tree should '''not''' change the relative structure of the elements that will remain in the tree
   * (i.e., any node's descendant should remain a descendant).
   * It can be proven that there is a '''unique answer'''.
   *
   * Return ''the root of the trimmed binary search tree''.
   * Note that the root may change depending on the given bounds.
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree in the range `[1, 10_000]`.
   *  - `0 <= Node.val <= 10_000`
   *  - The value of each node in the tree is '''unique'''.
   *  - `root` is guaranteed to be a valid binary search tree.
   *  - `0 <= low <= high <= 10_000`
   */
  object Solution {
    def trimBST(root: TreeNode, low: Int, high: Int): TreeNode = {
      if (root == null) root
      else if (root.value < low) trimBST(root.right, low, high)
      else if (root.value > high) trimBST(root.left, low, high)
      else new TreeNode(root.value, trimBST(root.left, low, high), trimBST(root.right, low, high))
    }
  }

  object SolutionMy {
    def trimBST(root: TreeNode, low: Int, high: Int): TreeNode = {
      def trimLower(n: TreeNode): TreeNode = {
        if (n == null) null
        else if (n.value >= low)
          new TreeNode(n.value, trimLower(n.left), n.right)
        else
          trimLower(n.right)
      }
      def trimHigher(n: TreeNode): TreeNode = {
        if (n == null) null
        else if (n.value <= high)
          new TreeNode(n.value, n.left, trimHigher(n.right))
        else
          trimHigher(n.left)
      }

      trimHigher(trimLower(root))
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String =
      s"${if (left == null) "" else s"$left,"}$value${if (right == null) "" else s",$right"}"
  }

  import Solution.trimBST

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [1,0,2], low = 1, high = 2) -> [1,null,2]" in {
    val root = n(1, n(0), n(2))
    val e = n(1, null, n(2))
    trimBST(root, 1, 2).toString shouldBe e.toString
    //  1  => 1
    // 0 2     2
  }
  "Example 2: (root = [3,0,4,null,2,null,null,1], low = 1, high = 3) -> [3,2,null,1]" in {
    val root = n(3, n(0, null, n(2, n(1))), n(4))
    val e = n(3, n(2, n(1)))
    trimBST(root, 1, 3).toString shouldBe e.toString
    //    3    =>   3
    // 0     4     2
    //   2        1
    //  1
  }
  "Example 3: (root = [1], low = 1, high = 2) -> [1]" in {
    trimBST(n(1), 1, 2).toString shouldBe n(1).toString
  }
  "Example 4: (root = [1,null,2], low = 1, high = 3) -> [1,null,2]" in {
    val root = n(1, null, n(2))
    trimBST(root, 1, 3).toString shouldBe root.toString
  }
  "Example 5: (root = [1,null,2], low = 2, high = 4) -> [2]" in {
    val root = n(1, null, n(2))
    trimBST(root, 2, 4).toString shouldBe n(2).toString
    // 1  => 2
    //  2
  }

  "(root = [1 to 10_000 balanced], low = 1986, high = 1986) -> [1986]" in {
    def buildBst(l: Int, r: Int): TreeNode = {
      if (l > r) null
      else {
        val mid = l + (r - l) / 2
        n(mid, buildBst(l, mid - 1), buildBst(mid + 1, r))
      }
    }
    val root = buildBst(1, 10000)
    //Thread.sleep(30)
    //println(s"Root: $root")
    trimBST(root, 1986, 1986).toString shouldBe n(1986).toString
  }
  "(root = [1 to 10_000 in left branch], low = 1986, high = 1986) -> [1986]" in {
    @scala.annotation.tailrec
    def buildBst(l: Int, r: Int, root: TreeNode, curr: TreeNode): TreeNode = {
      if (l > r) root
      else {
        curr.left = n(r)
        buildBst(l, r - 1, root, curr.left)
      }
    }
    val dummy = n(0)
    val root = buildBst(1, 10000, dummy, dummy).left
    //Thread.sleep(30)
    //println(s"Root: $root")
    trimBST(root, 1986, 1986).toString shouldBe n(1986).toString
  }

}
