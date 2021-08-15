package challenge.c2021_08

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/explore/challenge/card/august-leetcoding-challenge-2021/615/week-3-august-15th-august-21st/3890/]] */
class c2021_08_w3 extends AnyWordSpec with Matchers {
  /**
   * == Paint House II ==
   *
   * There are a row of `n` houses, each house can be painted with one of the `k` colors.
   * The cost of painting each house with a certain color is different.
   * You have to paint all the houses such that no two adjacent houses have the same color.
   *
   * The cost of painting each house with a certain color is represented by an `n x k` cost matrix costs.
   *
   *  - For example, `costs[0][0]` is the cost of painting house `0` with color `0`;
   *    `costs[1][2]` is the cost of painting house `1` with color `2`, and so on...
   *
   * Return ''the minimum cost to paint all houses''.
   *
   * '''Constraints:'''
   *  - `costs.length == n`
   *  - `costs[i].length == k`
   *  - `1 <= n <= 100`
   *  - `1 <= k <= 20`
   *  - `1 <= costs[i][j] <= 20`
   *
   * '''Follow up:''' Could you solve it in `O(nk)` runtime?
   */
  object Solution {
    /** [[https://leetcode.com/problems/paint-house-ii/solution/]] */
    def minCostII(costs: Array[Array[Int]]): Int =
      if (costs.length == 1) costs(0).min
      else if (costs(0).length == 1) Int.MaxValue
      else {
        import scala.util.chaining.scalaUtilChainingOps

        @scala.annotation.tailrec
        def rec(i: Int, p: (Int, Int, Int)): Int =
          if (i == costs.length) p._1
          else rec(i + 1, p.pipe { case (p1, pj, p2) =>
            costs(i).iterator.zipWithIndex
              .map { case (c, j) => (c + (if (j != pj) p1 else p2), j) }
              .foldLeft((Int.MaxValue, -1, Int.MaxValue)) {
                case ((a1, _, _), (c, j)) if c < a1   => (c, j, a1)
                case ((a1, aj, a2), (c, j)) if c < a2 => (a1, aj, c)
                case (a, _)                           => a
              }
          })

        val (p1, pj) = costs(0).iterator.zipWithIndex.min
        val (p2, _) = costs(0).iterator.zipWithIndex.filter(_._2 != pj).min
        rec(1, (p1, pj, p2))
      }

    def minCostIIMy(costs: Array[Array[Int]]): Int =
      if (costs.length == 1) costs(0).min
      else if (costs(0).length == 1) Int.MaxValue
      else {
        @scala.annotation.tailrec
        def rec(i: Int, rsf: Seq[Int]): Seq[Int] =
          if (i == costs.length) rsf
          else {
            val (p1, pj) = rsf.iterator.zipWithIndex.min
            val (p2, _) = rsf.iterator.zipWithIndex.filter(_._2 != pj).min
            rec(i + 1, costs(i).zipWithIndex.map { case (c, j) => c + (if (j != pj) p1 else p2) })
          }

        rec(1, costs(0)).min
      }
  }

  "Example 1: (costs = [[1,5,3],[2,9,4]]) -> 5" in {
    Solution.minCostII(Array(Array(1, 5, 3), Array(2, 9, 4))) shouldBe 5
    // Explanation:
    // Paint house 0 into color 0, paint house 1 into color 2. Minimum cost: 1 + 4 = 5;
    // Or paint house 0 into color 2, paint house 1 into color 0. Minimum cost: 3 + 2 = 5.
  }
  "Example 2: (costs = [[1,3],[2,4]]) -> 5" in {
    Solution.minCostII(Array(Array(1, 3), Array(2, 4))) shouldBe 5
  }

  "(costs = [[1,2]]) -> 1" in (Solution.minCostII(Array(Array(1, 2))) shouldBe 1)
  "(costs = [[2,1]]) -> 1" in (Solution.minCostII(Array(Array(2, 1))) shouldBe 1)

  "(costs = [[1]]) -> 1" in (Solution.minCostII(Array(Array(1))) shouldBe 1)
  "(costs = [[3]]) -> 3" in (Solution.minCostII(Array(Array(3))) shouldBe 3)
  "(costs = [[1],[1]]) -> Int.MaxValue" in (Solution.minCostII(Array(Array(1), Array(1))) shouldBe Int.MaxValue)

  "(costs = [[2,3],[2,4]]) -> 5" in (Solution.minCostII(Array(Array(2, 3), Array(2, 4))) shouldBe 5)
  "(costs = [[1,3],[3,4]]) -> 5" in (Solution.minCostII(Array(Array(1, 3), Array(3, 4))) shouldBe 5)

  "(costs = [[1,2,3],[1,2,3],[1,2,3]]) -> 4" in {
    Solution.minCostII(Array(Array(1, 2, 3), Array(1, 2, 3), Array(1, 2, 3))) shouldBe 4
  }
}
