package challenge.c2021.c2021_07

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/608/week-1-july-1st-july-7th/3798/]] */
class c2021_07_w1 extends AnyWordSpec with Matchers {
  /**
   * == Find Leaves of Binary Tree ==
   *
   * Given the `root` of a binary tree, collect a tree's nodes as if you were doing this:
   *  - Collect all the leaf nodes.
   *  - Remove all the leaf nodes.
   *  - Repeat until the tree is empty.
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[1, 100]`.
   *  - `-100 <= Node.val <= 100`
   */
  object Solution {
    /** Approach 2: DFS (Depth-First Search) without sorting
     *
     * [[https://leetcode.com/problems/find-leaves-of-binary-tree/solution/]] */
    def findLeaves(root: TreeNode): List[List[Int]] = {
      val result = collection.mutable.ArrayBuffer[List[Int]]()

      def getHeight(n: TreeNode): Int =
        if (n == null) -1
        else {
          val ch = 1 + getHeight(n.left) max getHeight(n.right)
          if (result.length == ch) result.addOne(Nil)
          result(ch) = n.value :: result(ch)

          ch
        }

      getHeight(root)

      result.toList
    }

    def findLeavesMy(root: TreeNode): List[List[Int]] = {
      def isLeaf(n: TreeNode, ls: Set[TreeNode]): Boolean =
        !ls.contains(n) &&
          (n.left == null || ls.contains(n.left)) &&
          (n.right == null || ls.contains(n.right))

      def takeLeaves(n: TreeNode, ls: Set[TreeNode]): Seq[TreeNode] =
        if (n == null) Nil
        else if (isLeaf(n, ls)) Seq(n)
        else takeLeaves(n.left, ls) ++ takeLeaves(n.right, ls)

      def rec(ls: Set[TreeNode]): List[List[Int]] = {
        if (ls.contains(root)) Nil
        else {
          val leaves = takeLeaves(root, ls)
          leaves.map(_.value).toList :: rec(ls ++ leaves)
        }
      }

      rec(Set())
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right
  }

  import Solution.findLeaves

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [1,2,3,4,5]) -> [[4,5,3],[2],[1]]" in {
    val root = n(1, n(2, n(4), n(5)), n(3))
    findLeaves(root) shouldBe List(List(4, 5, 3), List(2), List(1))
    //     1
    //    / \       1
    //   2   3 =>  /  => 1
    //  / \       2
    // 4   5
    //Explanation:
    // [[3,5,4],[2],[1]] and [[3,4,5],[2],[1]] are also considered correct answers
    // since per each level it does not matter the order on which elements are returned.
  }
  "Example 2: (root = [1]) -> [[1]]" in (findLeaves(n(1)) shouldBe List(List(1)))

  "(root = [3,2,1,1,1]) -> [[1,1,1],[2],[3]]" in {
    val root = n(3, n(2, n(1), n(1)), n(1))
    findLeaves(root) shouldBe List(List(1, 1, 1), List(2), List(3))
    //     3
    //    / \       3
    //   2   1 =>  /  => 3
    //  / \       2
    // 1   1
  }
  "(root = [1,1,1,1,1]) -> [[1,1,1],[1],[1]]" in {
    val root = n(1, n(1, n(1), n(1)), n(1))
    findLeaves(root) shouldBe List(List(1, 1, 1), List(1), List(1))
    //     1
    //    / \       1
    //   1   1 =>  /  => 1
    //  / \       1
    // 1   1
  }
}
