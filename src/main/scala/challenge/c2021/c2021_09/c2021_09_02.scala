package challenge.c2021.c2021_09

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/636/week-1-september-1st-september-7th/3961/]]
 */
class c2021_09_02 extends AnyWordSpec with Matchers {
  /**
   * === Unique Binary Search Trees II ===
   */
  object Solution {
    def generateTrees(i: Int): List[TreeNode] = {
      def rec(l: Int, r: Int): Seq[TreeNode] =
        if (l == r) Seq(new TreeNode(l))
        else (l to r).flatMap { m =>
          if (m == l) rec(m + 1, r).map(n => new TreeNode(m, null, n))
          else if (m == r) rec(l, m - 1).map(n => new TreeNode(m, n))
          else for (lc <- rec(l, m - 1); rc <- rec(m + 1, r)) yield new TreeNode(m, lc, rc)
        }

      rec(1, i).toList
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String = s"{${if (left == null) "n" else left},$value,${if (right == null) "n" else right}}"
  }

  private def n(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (n = 3) -> [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]" in {
    val result = Solution.generateTrees(3)
    result.map(_.toString).toSet shouldBe Set(
      n(1, r = n(3, n(2))),
      n(1, r = n(2, r = n(3))),
      n(2, n(1), n(3)),
      n(3, n(2, n(1))),
      n(3, n(1, r = n(2)))).map(_.toString)
    result.size shouldBe 5
    // 1   1       2       3   3
    //  \   \     / \     /   /
    //   3   2   1   3   2   1
    //  /     \         /     \
    // 2       3       1       2
  }
  "Example 2: (n = 1) -> [[1]]" in {
    val result = Solution.generateTrees(1)
    result.map(_.toString).toSet shouldBe Set(n(1)).map(_.toString)
    result.size shouldBe 1
  }

  "(n = 2) -> [[1,2],[2,1]]" in {
    val result = Solution.generateTrees(2)
    result.map(_.toString).toSet shouldBe Set(n(1, r = n(2)), n(2, n(1))).map(_.toString)
    result.size shouldBe 2
    // 1     2
    //  \   /
    //   2 1
  }

  "(n = 8) -> 1430 trees" in {
    val result = Solution.generateTrees(8)
    result.size shouldBe 1430
  }
}
