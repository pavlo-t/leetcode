package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3617/]]
 */
//noinspection DuplicatedCode
class c2021_01_26 extends AnyWordSpec with Matchers {
  /**
   * === Path With Minimum Effort ===
   *
   * You are a hiker preparing for an upcoming hike.
   * You are given `heights`, a 2D array of size `rows x columns`,
   * where `heights[row][col]` represents the height of cell `(row, col)`.
   * You are situated in the top-left cell, `(0, 0)`,
   * and you hope to travel to the bottom-right cell, `(rows-1, columns-1)` (i.e., '''0-indexed''').
   * You can move '''up''', '''down''', '''left''', or '''right''',
   * and you wish to find a route that requires the minimum '''effort'''.
   *
   * A route's '''effort''' is the '''maximum absolute difference''' in heights between two consecutive cells of the route.
   *
   * Return ''the minimum '''effort''' required to travel from the top-left cell to the bottom-right cell''.
   *
   * '''Constraints:'''
   *  - `1 <= heights.length, heights[i].length <= 100`
   *  - `1 <= heights[i][j] <= 1_000_000`
   */
  object Solution {
    def minimumEffortPath(heights: Array[Array[Int]]): Int = {
      val Lr = heights.length - 1
      val Lc = heights(0).length - 1

      val seen = Array.fill(Lr + 1, Lc + 1)(false)

      def dfs(r: Int, c: Int, limit: Int): Boolean = {
        if (r == Lr && c == Lc) true
        else {
          Seq((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1))
            .filter { case (nr, nc) =>
              nr >= 0 && nr <= Lr &&
                nc >= 0 && nc <= Lc &&
                !seen(nr)(nc) &&
                (heights(r)(c) - heights(nr)(nc)).abs <= limit
            }
            .map({ case (nr, nc) =>
              seen(nr)(nc) = true
              dfs(nr, nc, limit)
            }).exists(identity)
        }
      }

      @scala.annotation.tailrec
      def bs(l: Int, r: Int): Int = {
        if (l >= r) l
        else {
          val mid = l + (r - l) / 2
          seen.foreach(_.mapInPlace(_ => false))
          if (dfs(0, 0, mid))
            bs(l, mid)
          else
            bs(mid + 1, r)
        }
      }

      bs(0, 1_000_000)
    }
  }

  import Solution.minimumEffortPath

  "Example 1: (heights = [[1,2,2],[3,8,2],[5,3,5]]) -> 2" in {
    minimumEffortPath(Array(
      Array(1, 2, 2),
      Array(3, 8, 2),
      Array(5, 3, 5))) shouldBe 2
    //Explanation:
    // The route of [1,3,5,3,5] has a maximum absolute difference of 2 in consecutive cells.
    // This is better than the route of [1,2,2,2,5], where the maximum absolute difference is 3.
  }
  "Example 2: (heights = [[1,2,3],[3,8,4],[5,3,5]]) -> 1" in {
    minimumEffortPath(Array(
      Array(1, 2, 3),
      Array(3, 8, 4),
      Array(5, 3, 5))) shouldBe 1
    //Explanation:
    // The route of [1,2,3,4,5] has a maximum absolute difference of 1 in consecutive cells,
    // which is better than route [1,3,5,3,5].
  }
  "Example 3: (heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]) -> 0" in {
    minimumEffortPath(Array(
      Array(1, 2, 1, 1, 1),
      Array(1, 2, 1, 2, 1),
      Array(1, 2, 1, 2, 1),
      Array(1, 2, 1, 2, 1),
      Array(1, 1, 1, 2, 1))) shouldBe 0
  }

  "Test 9(heights = [" +
    "[8,3,2,5,2,10,7,1,8,9]," +
    "[1,4,9,1,10,2,4,10,3,5]," +
    "[4,10,10,3,6,1,3,9,8,8]," +
    "[4,4,6,10,10,10,2,10,8,8]," +
    "[9,10,2,4,1,2,2,6,5,7]," +
    "[2,9,2,6,1,4,7,6,10,9]," +
    "[8,8,2,10,8,2,3,9,5,3]," +
    "[2,10,9,3,5,1,7,4,5,6]," +
    "[2,3,9,2,5,10,2,7,1,8]," +
    "[9,10,4,10,7,4,9,3,1,6]]) -> 5" in {
    minimumEffortPath(Array(
      Array(8, 3, 2, 5, 2, 10, 7, 1, 8, 9),
      Array(1, 4, 9, 1, 10, 2, 4, 10, 3, 5),
      Array(4, 10, 10, 3, 6, 1, 3, 9, 8, 8),
      Array(4, 4, 6, 10, 10, 10, 2, 10, 8, 8),
      Array(9, 10, 2, 4, 1, 2, 2, 6, 5, 7),
      Array(2, 9, 2, 6, 1, 4, 7, 6, 10, 9),
      Array(8, 8, 2, 10, 8, 2, 3, 9, 5, 3),
      Array(2, 10, 9, 3, 5, 1, 7, 4, 5, 6),
      Array(2, 3, 9, 2, 5, 10, 2, 7, 1, 8),
      Array(9, 10, 4, 10, 7, 4, 9, 3, 1, 6))) shouldBe 5
  }

  "(heights = [[1,2,1,1,1,1],[1,2,1,2,2,1],[1,2,1,1,2,1],[1,2,2,1,2,1],[1,1,1,1,2,1]]) -> 0" in {
    minimumEffortPath(Array(
      Array(1, 2, 1, 1, 1, 1),
      Array(1, 2, 1, 2, 2, 1),
      Array(1, 2, 1, 1, 2, 1),
      Array(1, 2, 2, 1, 2, 1),
      Array(1, 1, 1, 1, 2, 1))) shouldBe 0
  }
  "(heights = [[1,2],[3,5]]) -> 2" in {
    minimumEffortPath(Array(
      Array(1, 2),
      Array(3, 5))) shouldBe 2
  }
  "(heights = [[1,2]]) -> 1" in {
    minimumEffortPath(Array(Array(1, 2))) shouldBe 1
  }
  "(heights = [[1],[3]]) -> 2" in {
    minimumEffortPath(Array(Array(1), Array(3))) shouldBe 2
  }
  "(heights = Array.fill(10)(Array.fill(10)(1))) -> 0" in {
    minimumEffortPath(Array.fill(10)(Array.fill(10)(1))) shouldBe 0
  }
  "(heights = Array.fill(100)(Array.fill(100)(1))) -> 0" in {
    minimumEffortPath(Array.fill(100)(Array.fill(100)(1))) shouldBe 0
  }
}
