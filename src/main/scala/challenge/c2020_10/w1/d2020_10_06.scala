package challenge.c2020_10.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d2020_10_06 extends AnyWordSpec with Matchers {

  /**
   * Insert into a Binary Search Tree
   *
   * You are given the `root` node of a binary search tree (BST) and a `value` to insert into the tree.
   * Return <em>the root node of the BST after the insertion</em>.
   * It is <b>guaranteed</b> that the new value does not exist in the original BST.
   *
   * <b>Notice</b> that there may exist multiple valid ways for the insertion,
   * as long as the tree remains a BST after insertion.
   * You can return <b>any of them</b>.
   *
   *
   * <b>Constraints:</b><ul>
   * <li> The number of nodes in the tree will be in the range `[0, 104]`.
   * <li> `-108 <= Node.val <= 108`
   * <li> All the values `Node.val` are <b>unique</b>.
   * <li> `-108 <= val <= 108`
   * <li> It's <b>guaranteed</b> that `val` does not exist in the original BST.
   */
  object Solution {
    import scala.annotation.tailrec

    def insertIntoBST(root: TreeNode, v: Int): TreeNode = {
      if (root == null) new TreeNode(v)
      else {
        @tailrec
        def loop(n: TreeNode): Unit = {
          if (v < n.value)
            if (n.left == null) n.left = new TreeNode(v)
            else loop(n.left)
          else if (n.right == null)
            n.right = new TreeNode(v)
          else loop(n.right)
        }

        loop(root)

        root
      }
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String =
      s"{${if (left == null) "" else left},$value,${if (right == null) "" else right}}"
  }
  object TreeNode {
    def apply(l: TreeNode = null, v: Int = 0, r: TreeNode = null) =
      new TreeNode(v, l, r)
  }

  "Example 1" in {
    val tree =
      TreeNode(
        TreeNode(TreeNode(v = 1), 2, TreeNode(v = 3)),
        4,
        TreeNode(v = 7))
    val expected =
      TreeNode(
        TreeNode(TreeNode(v = 1), 2, TreeNode(v = 3)),
        4,
        TreeNode(TreeNode(v = 5), 7))

    Solution.insertIntoBST(tree, 5).toString shouldBe expected.toString
  }
  "Example 2" in {
    val tree =
      TreeNode(
        TreeNode(TreeNode(v = 10), 20, TreeNode(v = 30)),
        40,
        TreeNode(TreeNode(v = 50), v = 60, TreeNode(v = 70)))
    val expected =
      TreeNode(
        TreeNode(TreeNode(v = 10), 20, TreeNode(TreeNode(v = 25), 30)),
        40,
        TreeNode(TreeNode(v = 50), v = 60, TreeNode(v = 70)))

    Solution.insertIntoBST(tree, 25).toString shouldBe expected.toString
  }

  "Test 9" in {
    val expected = TreeNode(v = 5)

    Solution.insertIntoBST(null, 5).toString shouldBe expected.toString
  }
}
