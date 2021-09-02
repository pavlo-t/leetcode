package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/583/week-5-january-29th-january-31st/3621/]]
 */
//noinspection DuplicatedCode
class c2021_01_29 extends AnyWordSpec with Matchers {
  /**
   * === Vertical Order Traversal of a Binary Tree ===
   *
   * Given the `root` of a binary tree, calculate the '''vertical order traversal''' of the binary tree.
   *
   * For each node at position `(x, y)`, its left and right children will be at positions
   * `(x - 1, y - 1)` and `(x + 1, y - 1)` respectively.
   *
   * The '''vertical order traversal''' of a binary tree is a list of non-empty '''reports''' for each unique
   * x-coordinate from left to right.
   * Each '''report''' is a list of all nodes at a given x-coordinate.
   * The '''report''' should be primarily sorted by y-coordinate from highest y-coordinate to lowest.
   * If any two nodes have the same y-coordinate in the '''report''',
   * the node with the smaller value should appear earlier.
   *
   * Return ''the '''vertical order traversal''' of the binary tree''.
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[1, 1000]`.
   *  - `0 <= Node.val <= 1000`
   */
  object Solution {
    def verticalTraversal(root: TreeNode): List[List[Int]] = {
      def rec(n: TreeNode, x: Int, d: Int): Seq[(Int, Int, Int)] = {
        if (n == null) Nil
        else (x, d, n.value) +: rec(n.left, x - 1, d + 1) :++ rec(n.right, x + 1, d + 1)
      }
      rec(root, 0, 0).groupBy(_._1).toList.sortBy(_._1).map(_._2.sorted.map(_._3).toList)
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right
  }

  import Solution.verticalTraversal

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [3,9,20,null,null,15,7]) -> [[9],[3,15],[20],[7]]" in {
    val root = n(3, n(9), n(20, n(15), n(7)))
    val e = List(List(9), List(3, 15), List(20), List(7))
    verticalTraversal(root) shouldBe e
    //Explanation: Without loss of generality, we can assume the root node is at position (0, 0):
    //  y|
    // -----------------------------------
    //  0|   3     |      (0,0)
    // -1| 9   20  |(-1,-1)    (1,-1)
    // -2|   15  7 |      (0,-2)    (2,-2)
    // -----------------------------------
    //  x|-1|0|1|2 | -1   |0   |1   |2
    //The node with value 9 occurs at position (-1, -1).
    //The nodes with values 3 and 15 occur at positions (0, 0) and (0, -2).
    //The node with value 20 occurs at position (1, -1).
    //The node with value 7 occurs at position (2, -2).
  }
  "Example 2: (root = [1,2,3,4,5,6,7]) -> [[4],[2],[1,5,6],[3],[7]]" in {
    val root = n(1, n(2, n(4), n(5)), n(3, n(6), n(7)))
    val e = List(List(4), List(2), List(1, 5, 6), List(3), List(7))
    verticalTraversal(root) shouldBe e
    //Explanation: The node with value 5 and the node with value 6 have the same position according to the given scheme.
    //  y|
    // -----------------
    //  0|       1
    // -1|    2     3
    // -2| 4    5 6    7
    // -----------------
    //  x|-2 -1  0  1  2
    //However, in the report [1,5,6], the node with value 5 comes first since 5 is smaller than 6.
  }

  "(root = 1023 nodes) -> [[4],[2],[1,5,6],[3],[7]]" in {
    def buildTree(depth: Int): TreeNode = if (depth > 0) n(depth, buildTree(depth - 1), buildTree(depth - 1)) else null
    val root = buildTree(10)
    val result = verticalTraversal(root)

    result.flatten.size shouldBe 1023
    result shouldBe List()
  }


}
