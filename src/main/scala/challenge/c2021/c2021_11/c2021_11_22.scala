package challenge.c2021.c2021_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/problems/delete-node-in-a-bst/]] */
//noinspection DuplicatedCode
class c2021_11_22 extends AnyWordSpec with Matchers {
  /**
   * = 450. Delete Node in a BST =
   */
  object Solution {
    def deleteNode(root: TreeNode, key: Int): TreeNode = {
      Thread.sleep(100)
      println(s"root: $root; k: $key")

      val dummy = new TreeNode(Int.MinValue, null, root)
      var prev = dummy
      var curr = root
      while (curr != null && curr.value != key) {
        prev = curr
        curr = if (curr.value < key) curr.right else curr.left
      }
      println(s"  prev: $prev")
      println(s"  curr: $curr")

      def setCurr(prev: TreeNode, curr: TreeNode, newCurr: TreeNode): Unit =
        if (prev.left == curr) prev.left = newCurr else prev.right = newCurr

      if (curr != null) {
        (curr.left, curr.right) match {
          case (null, null) => setCurr(prev, curr, null)
          case (null, r) => setCurr(prev, curr, r)
          case (l, null) => setCurr(prev, curr, l)
          case (l, r) =>
            if (r.left != null) {
              var parent = r
              var leftMostChild = r.left
              while (leftMostChild.left != null) {
                parent = leftMostChild
                leftMostChild = leftMostChild.left
              }
              parent.left = leftMostChild.right
              curr.value = leftMostChild.value
            } else {
              r.left = l
              setCurr(prev, curr, r)
            }
        }
      }

      dummy.right
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String =
      s"($value,${if (left == null) "n" else left},${if (right == null) "n" else right})"
  }

  private val N: TreeNode = null

  private def n(v: Int, l: TreeNode = N, r: TreeNode = N): TreeNode = new TreeNode(v, l, r)

  "root = [], key = 0" in {
    Solution.deleteNode(N, 0) shouldBe null
  }
  "[5,3,6,2,4,null,7], key = 3" in {
    val r = n(5, n(3, n(2), n(4)), n(6, N, n(7)))
    val k = 3
    val e = n(5, n(4, n(2), N), n(6, N, n(7)))
    Solution.deleteNode(r, k).toString shouldBe e.toString
    //     5          5         5
    //    / \        / \       / \
    //   3   6  =>  4   6  OR 2   6
    //  / \   \    /     \     \   \
    // 2   4   7  2       7     4   7
    // Explanation: Given key to delete is 3. So we find the node with value 3 and delete it.
    // One valid answer is [5,4,6,2,null,null,7], shown in the above BST.
    // Please notice that another valid answer is [5,2,6,null,4,null,7] and it's also accepted.
  }
  "root = [5,3,6,2,4,null,7], key = 0" in {
    val r = n(5, n(3, n(2), n(4)), n(6, N, n(7)));
    val k = 0;
    val e = n(5, n(3, n(2), n(4)), n(6, N, n(7)));
    Solution.deleteNode(r, k).toString shouldBe e.toString
  }
  "root = [4,2,6,1,3,5,7], key = 4" in {
    //      4           5
    //    /   \        / \
    //   2     6  =>  2   6
    //  / \   / \    / \   \
    // 1   3 5   7  1   3   7
    val r = n(4, n(2, n(1), n(3)), n(6, n(5), n(7)))
    val k = 4
    val e = n(5, n(2, n(1), n(3)), n(6, N, n(7)))
    Solution.deleteNode(r, k).toString shouldBe e.toString
  }
  "root = [4,2,7,1,3,5,8,n,n,n,n,n,6,n,n], key = 4" in {
    // [5,2,7,1,3,6,8]
    //      4            5
    //    /   \        /   \
    //   2     7  =>  2     7
    //  / \   / \    / \   / \
    // 1   3 5   8  1   3 6   8
    //        \
    //         6
    val r = n(4, n(2, n(1), n(3)), n(7, n(5, N, n(6)), n(8)))
    val k = 4
    val e = n(5, n(2, n(1), n(3)), n(7, n(6), n(8)))
    Solution.deleteNode(r, k).toString shouldBe e.toString
  }
}
