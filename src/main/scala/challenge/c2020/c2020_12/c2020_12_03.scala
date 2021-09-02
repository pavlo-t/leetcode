package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/569/week-1-december-1st-december-7th/3553/]]
 */
//noinspection DuplicatedCode
class c2020_12_03 extends AnyWordSpec with Matchers {

  /**
   * === Increasing Order Search Tree ===
   *
   * Given the `root` of a binary search tree,
   * rearrange the tree in '''in-order''' so that the leftmost node in the tree is now the root of the tree,
   * and every node has no left child and only one right child.
   *
   * '''Constraints:'''
   *  - The number of nodes in the given tree will be in the range `[1, 100]`.
   *  - `0 <= Node.val <= 1000`
   */
  object SolutionMutatingInput {
    def increasingBST(root: TreeNode): TreeNode =
      if (root == null) null
      else {
        def inOrder(n: TreeNode, next: TreeNode): TreeNode =
          if (n == null) next
          else {
            val res = inOrder(n.left, n)
            n.left = null
            n.right = inOrder(n.right, next)
            res
          }

        inOrder(root, null)
      }
  }

  object Solution {
    def increasingBST(root: TreeNode): TreeNode =
      if (root == null) null
      else {
        def toList(n: TreeNode): List[Int] =
          if (n == null) Nil
          else toList(n.right) ++ List(n.value) ++ toList(n.left)

        toList(root).foldLeft(null: TreeNode) {
          case (null, v) => new TreeNode(v)
          case (r, v)    => new TreeNode(v, null, r)
        }
      }
  }
  object SolutionMyInitial {
    def increasingBST(root: TreeNode): TreeNode =
      if (root == null) null
      else {
        def toList(n: TreeNode): List[TreeNode] =
          if (n == null) Nil
          else toList(n.left) ++ List(new TreeNode(n.value)) ++ toList(n.right)

        val list = toList(root)
        if (list.tail != Nil)
          list.sliding(2).foreach { case List(l, r) => l.right = r }

        list.head
      }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String = s"$value,${childToString(left)},${childToString(right)}"
    private def childToString(n: TreeNode): String = if (n == null) "n" else n.toString
  }

  import Solution.increasingBST

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)
  private def r(v: Int, r: TreeNode = null): TreeNode = new TreeNode(v, _right = r)

  "Example 1: (root = [5,3,6,2,4,null,8,1,null,null,null,7,9]) -> " +
    "[1,null,2,null,3,null,4,null,5,null,6,null,7,null,8,null,9]" in {
    val root = n(5, n(3, n(2, n(1)), n(4)), n(6, r = n(8, n(7), n(9))))
    val expected = r(1, r(2, r(3, r(4, r(5, r(6, r(7, r(8, r(9)))))))))

    increasingBST(root).toString shouldBe expected.toString
  }
  "Example 2: (root = [5,1,7]) -> [1,null,5,null,7]" in {
    val root = n(5, n(1), n(7))
    val expected = r(1, r(5, r(7)))

    increasingBST(root).toString shouldBe expected.toString
  }
  "test 3: ([217]) -> [217]" in {
    val root = n(217)
    val expected = r(217)

    increasingBST(root).toString shouldBe expected.toString
  }

  "([1 to 100 in the left branch]) -> [1 to 100]" in {
    val root = n(100)
    var current = root
    for (i <- 99 to 1 by -1) {
      val l = n(i)
      current.left = l
      current = l
    }

    val expected = n(1)
    current = expected
    for (i <- 2 to 100) {
      val r = n(i)
      current.right = r
      current = r
    }

    increasingBST(root).toString shouldBe expected.toString
  }

}
