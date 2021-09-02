package challenge.c2020.c2020_10.w5

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_31 extends AnyWordSpec with Matchers {

  /**
   * <h3>Recover Binary Search Tree</h3>
   *
   * You are given the `root` of a binary search tree (BST),
   * where exactly two nodes of the tree were swapped by mistake.
   * <em>Recover the tree without changing its structure</em>.
   *
   * <b>Follow up:</b>
   * A solution using `O(n)` space is pretty straight forward.
   * Could you devise a constant space solution?
   *
   * <b>Constraints:</b><ul>
   * <li> The number of nodes in the tree is in the range `[2, 1000]`.
   * <li> `-2**31 <= Node.val <= 2**31 - 1`
   * </ul>
   *
   * Solution: [[https://www.geeksforgeeks.org/fix-two-swapped-nodes-of-bst/]]
   */
  object Solution {
    def recoverTree(root: TreeNode): Unit = {
      var first: Option[TreeNode] = None
      var last: Option[TreeNode] = None

      var prev: Option[TreeNode] = None

      def findErrors(n: TreeNode): Unit = {
        if (n != null) {
          findErrors(n.left)

          if (prev.nonEmpty) {
            if (prev.get.value > n.value) {
              if (first.isEmpty) {
                first = prev
                last = Some(n)
              } else {
                last = Some(n)
              }
            }
          }
          prev = Some(n)

          findErrors(n.right)
        }
      }

      findErrors(root)

      val tmp = first.get.value
      first.get.value = last.get.value
      last.get.value = tmp
    }
  }

  object SolutionONSpace {
    def recoverTree(root: TreeNode): Unit = {
      def toSeq(n: TreeNode): Seq[Int] =
        if (n == null) Seq()
        else toSeq(n.left) ++ Seq(n.value) ++ toSeq(n.right)

      val seq = toSeq(root).sorted

      var i = 0
      def fromSeq(n: TreeNode): Unit =
        if (n != null) {
          fromSeq(n.left)
          n.value = seq(i)
          i += 1
          fromSeq(n.right)
        }

      fromSeq(root)
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right
    override def toString: String = s"{$left,$value,$right}"
  }
  private def tn(v: Int = 0, l: TreeNode = null, r: TreeNode = null): TreeNode =
    new TreeNode(v, l, r)

  import Solution.recoverTree

  "Example 1: ([1,3,null,null,2]) -> [3,1,null,null,2]" in {
    val root = tn(1, l = tn(3, r = tn(2)))
    val expected = tn(3, l = tn(1, r = tn(2)))

    recoverTree(root)

    root.toString shouldBe expected.toString
    // Explanation: 3 cannot be a left child of 1 because 3 > 1. Swapping 1 and 3 makes the BST valid.
  }
  "Example 2: ([3,1,4,null,null,2]) -> [2,1,4,null,null,3]" in {
    val root = tn(3, tn(1), tn(4, tn(2)))
    val expected = tn(2, tn(1), tn(4, tn(3)))

    recoverTree(root)

    root.toString shouldBe expected.toString
    // Explanation: 2 cannot be in the right subtree of 3 because 2 < 3. Swapping 2 and 3 makes the BST valid.
  }

  "([1,2,null]) -> [2,1,null]" in {
    val root = tn(1, tn(2))
    val expected = tn(2, tn(1))

    recoverTree(root)

    root.toString shouldBe expected.toString
  }
  "([5,3,8,1,4,6,9,null,2,null,null,null,7,10,null]) -> [5,3,8,1,4,6,10,null,2,null,null,null,7,9,null]" in {
    val root =
      tn(5,
        tn(3, tn(1, r = tn(2)), tn(4)),
        tn(8, tn(6, r = tn(7)), tn(9, tn(10))))
    val expected =
      tn(5,
        tn(3, tn(1, r = tn(2)), tn(4)),
        tn(8, tn(6, r = tn(7)), tn(10, tn(9))))

    recoverTree(root)

    root.toString shouldBe expected.toString
  }
  "([9,3,8,1,4,6,10,null,2,null,null,null,7,5,null]) -> [5,3,8,1,4,6,10,null,2,null,null,null,7,9,null]" in {
    val root =
      tn(9,
        tn(3, tn(1, r = tn(2)), tn(4)),
        tn(8, tn(6, r = tn(7)), tn(10, tn(5))))
    val expected =
      tn(5,
        tn(3, tn(1, r = tn(2)), tn(4)),
        tn(8, tn(6, r = tn(7)), tn(10, tn(9))))

    recoverTree(root)

    root.toString shouldBe expected.toString
  }
  "([10,5,8,2,20,null,null]) -> [10,5,20,2,8,null,null]" in {
    val root =
      tn(10,
        tn(5, tn(2), tn(20)),
        tn(8))
    val expected =
      tn(10,
        tn(5, tn(2), tn(8)),
        tn(20))

    recoverTree(root)

    root.toString shouldBe expected.toString
  }
  "(1_000 nodes tree) -> 1_000" in {
    val size = 1_000
    var root: TreeNode = tn(1)
    for (i <- 2 to size) root = tn(i, root)

    root.value = size - 1
    root.left.value = size

    recoverTree(root)

    root.value shouldBe size
  }

}
