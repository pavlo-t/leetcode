package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/583/week-5-january-29th-january-31st/3620/]]
 */
class c2021_01_w5 extends AnyWordSpec with Matchers {
  /**
   * === Number Of Corner Rectangles ===
   *
   * Given a grid where each entry is only 0 or 1, find the number of corner rectangles.
   *
   * A ''corner rectangle'' is 4 distinct 1s on the grid that form an axis-aligned rectangle.
   * Note that only the corners need to have the value 1.
   * Also, all four 1s used must be distinct.
   *
   * '''Note:'''
   *  - The number of rows and columns of `grid` will each be in the range `[1, 200]`.
   *  - Each `grid[i][j]` will be either `0` or `1`.
   *  - The number of `1`s in the grid will be at most `6000`.
   */
  object Solution {
    def countCornerRectangles(grid: Array[Array[Int]]): Int =
      if (grid.length < 2 || grid(0).length < 2) 0
      else {
        var result = 0
        val map = collection.mutable.Map[Int, Int]()
        for (r1 <- grid.indices; r2 <- (r1 + 1) until grid.length; c <- grid(0).indices) {
          if (grid(r1)(c) == 1 && grid(r2)(c) == 1) {
            val pos = r1 * grid(0).length + r2
            map get pos match {
              case None    => map.update(pos, 1)
              case Some(v) =>
                result += v
                map.update(pos, v + 1)
            }
          }
        }
        result
      }
  }

  import Solution.countCornerRectangles

  "Example 1: (grid = [\n" +
    "[1, 0, 0, 1, 0],\n" +
    "[0, 0, 1, 0, 1],\n" +
    "[0, 0, 0, 1, 0],\n" +
    "[1, 0, 1, 0, 1]]) -> 1" in {
    val grid = Array(
      Array(1, 0, 0, 1, 0),
      Array(0, 0, 1, 0, 1),
      Array(0, 0, 0, 1, 0),
      Array(1, 0, 1, 0, 1))
    countCornerRectangles(grid) shouldBe 1
    //Explanation: There is only one corner rectangle, with corners grid[1][2], grid[1][4], grid[3][2], grid[3][4].
  }
  "Example 2: (grid = [\n" +
    "[1, 1, 1],\n" +
    "[1, 1, 1],\n" +
    "[1, 1, 1]]) -> 9" in {
    val grid = Array(
      Array(1, 1, 1),
      Array(1, 1, 1),
      Array(1, 1, 1))
    countCornerRectangles(grid) shouldBe 9
    //Explanation: There are four 2x2 rectangles, four 2x3 and 3x2 rectangles, and one 3x3 rectangle.
  }
  "Example 3: (grid = [[1, 1, 1, 1]]) -> 0" in {
    countCornerRectangles(Array(Array(1, 1, 1, 1))) shouldBe 0
    //Explanation: Rectangles must have four distinct corners.
  }

  "(grid = 200 x 200 1s) -> 0" in {
    countCornerRectangles(Array.fill(200, 200)(1)) shouldBe 396010000
  }
}
