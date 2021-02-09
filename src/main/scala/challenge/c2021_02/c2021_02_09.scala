package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3634/]]
 */
//noinspection DuplicatedCode
class c2021_02_09 extends AnyWordSpec with Matchers {
  /**
   * === Convert BST to Greater Tree ===
   *
   * Given the `root` of a Binary Search Tree (BST), convert it to a Greater Tree such that every key of the original
   * BST is changed to the original key plus sum of all keys greater than the original key in BST.
   *
   * As a reminder, a ''binary search tree'' is a tree that satisfies these constraints:
   *  - The left subtree of a node contains only nodes with keys '''less than''' the node's key.
   *  - The right subtree of a node contains only nodes with keys '''greater than''' the node's key.
   *  - Both the left and right subtrees must also be binary search trees.
   *
   * '''Note:''' This question is the same as
   * [[https://leetcode.com/problems/binary-search-tree-to-greater-sum-tree/ 1038]]
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[0, 10_000]`.
   *  - `-10_000 <= Node.val <= 10_000`
   *  - All the values in the tree are '''unique'''.
   *  - `root` is guaranteed to be a valid binary search tree.
   */
  object Solution {
    def convertBST(root: TreeNode): TreeNode = {
      var sum = 0
      def rec(n: TreeNode): Unit = {
        if (n != null) {
          rec(n.right)
          sum += n.value
          n.value = sum
          rec(n.left)
        }
      }
      rec(root)
      root
    }
  }
  object SolutionMutState {
    def convertBST(root: TreeNode): TreeNode = {
      var sum = 0
      def rec(n: TreeNode): TreeNode = {
        if (n == null) null
        else {
          val r = rec(n.right)
          sum += n.value
          new TreeNode(sum, rec(n.left), r)
        }
      }
      rec(root)
    }
  }
  object SolutionMy {
    def convertBST(root: TreeNode): TreeNode = {
      def rec(n: TreeNode, sum: Int): (TreeNode, Int) = {
        if (n == null) (null, sum)
        else {
          val (r, rSum) = rec(n.right, sum)
          val nSum = rSum + n.value
          val (l, lSum) = rec(n.left, nSum)
          (new TreeNode(nSum, l, r), lSum)
        }
      }
      rec(root, 0)._1
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String = s"$value,$left,$right"
  }

  import Solution.convertBST

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]) -> " +
    "[30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]" in {
    val root = n(4, n(1, n(0), n(2, r = n(3))), n(6, n(5), n(7, r = n(8))))
    val e =
      n(30,
        n(36,
          n(36),
          n(35,
            r = n(33))),
        n(21,
          n(26),
          n(15,
            r = n(8))))
    convertBST(root).toString shouldBe e.toString
    //       4                  30
    //     /   \              /    \
    //   1       6          36      21
    //  / \     / \    =>  /  \    /  \
    // 0   2   5   7      36  35  26  15
    //      \       \           \       \
    //       3       8          33       8
  }
  "Example 2: (root = [0,null,1]) -> [1,null,1]" in {
    val root = n(0, r = n(1))
    val e = n(1, r = n(1))
    convertBST(root).toString shouldBe e.toString
    // 0      1
    //  \  =>  \
    //   1      1
  }
  "Example 3: (root = [1,0,2]) -> [3,3,2]" in {
    val root = n(1, n(0), n(2))
    val e = n(3, n(3), n(2))
    convertBST(root).toString shouldBe e.toString
    //   1        3
    //  / \  =>  / \
    // 0   2    3   2
  }
  "Example 4: (root = [3,2,4,1]) -> [7,9,4,10]" in {
    val root = n(3, n(2, n(1)), n(4))
    val e = n(7, n(9, n(10)), n(4))
    convertBST(root).toString shouldBe e.toString
    //     3          7
    //    / \        / \
    //   2   4 =>   9   4
    //  /          /
    // 1          10
  }

  "(root = [1 to 10000]) -> ???" in {
    def buildBst(l: Int, r: Int): TreeNode = {
      if (l > r) null
      else {
        val mid = l + (r - l) / 2
        n(mid, buildBst(l, mid - 1), buildBst(mid + 1, r))
      }
    }
    def buildE(l: Int, r: Int): TreeNode = {
      if (l > r) null
      else {
        val mid = l + (r - l) / 2
        n((mid to 10000).sum, buildE(l, mid - 1), buildE(mid + 1, r))
      }
    }
    val root = buildBst(1, 10000)
    val e = buildE(1, 10000)
    convertBST(root).toString shouldBe e.toString
  }

}
