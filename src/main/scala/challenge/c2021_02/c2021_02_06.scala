package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/584/week-1-february-1st-february-7th/3630/]]
 */
//noinspection DuplicatedCode
class c2021_02_06 extends AnyWordSpec with Matchers {
  /**
   * === Binary Tree Right Side View ===
   *
   * Given a binary tree, imagine yourself standing on the right side of it,
   * return the values of the nodes you can see ordered from top to bottom.
   *
   *
   */
  object Solution {
    def rightSideView(root: TreeNode): List[Int] = {
      def appendChildren(n: TreeNode, todo: Seq[TreeNode]): Seq[TreeNode] = (n.left, n.right) match {
        case (null, null) => todo
        case (null, r)    => todo :+ r
        case (l, null)    => todo :+ l
        case (l, r)       => todo :+ l :+ r
      }
      @scala.annotation.tailrec
      def bfs(todo: Seq[TreeNode], rsf: List[Int]): List[Int] = todo match {
        case null +: _         => rsf.reverse
        case n +: null +: rest => bfs(appendChildren(n, rest) :+ null, n.value :: rsf)
        case n +: rest         => bfs(appendChildren(n, rest), rsf)
      }

      bfs(Seq(root, null), List())
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right
  }

  import Solution.rightSideView

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example: ([1,2,3,null,5,null,4]) -> [1, 3, 4]" in {
    val root = n(1, n(2, r = n(5)), n(3, r = n(4)))
    rightSideView(root) shouldBe List(1, 3, 4)
    //Explanation:
    //
    //    1       <---
    //  /   \
    // 2     3    <---
    //  \     \
    //   5     4  <---
  }
  "([1,2,3,null,5,4,null]) -> [1, 3, 4]" in {
    val root = n(1, n(2, r = n(5)), n(3, n(4)))
    rightSideView(root) shouldBe List(1, 3, 4)
    //Explanation:
    //
    //    1       <---
    //  /   \
    // 2     3    <---
    //  \   /
    //   5 4      <---
  }
  "([]) -> []" in {
    rightSideView(null) shouldBe Nil
  }

}
