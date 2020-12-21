package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3571/]]
 */
//noinspection DuplicatedCode
class c2020_12_19 extends AnyWordSpec with Matchers {

  /**
   * === Cherry Pickup II ===
   *
   * Given a `rows x cols` matrix `grid` representing a field of cherries.
   * Each cell in `grid` represents the number of cherries that you can collect.
   *
   * You have two robots that can collect cherries for you, Robot #1 is located at the top-left corner `(0,0)`,
   * and Robot #2 is located at the top-right corner `(0, cols-1)` of the grid.
   *
   * Return ''the maximum number of cherries collection using both robots'' by following the rules below:
   *  - From a cell `(i,j)`, robots can move to cell `(i+1, j-1)`, `(i+1, j)` or `(i+1, j+1)`.
   *  - When any robot is passing through a cell, it picks up all cherries, and the cell becomes an empty cell `(0)`.
   *  - When both robots stay on the same cell, only one of them takes the cherries.
   *  - Both robots cannot move outside of the `grid` at any moment.
   *  - Both robots should reach the bottom row in the `grid`.
   *
   * '''Constraints:'''
   *  - `2 <= grid.length, grid[i].length <= 70`
   *  - `0 <= grid[i][j] <= 100 `
   */
  object Solution {
    def cherryPickup(G: Array[Array[Int]]): Int = {
      val rows = G.length
      val cols = G(0).length
      val dp = Array.fill(rows + 1, cols + 2, cols + 2)(0)

      for (i <- rows - 1 to 0 by -1; j <- 1 to cols; k <- 1 to cols if j != k) {
        val nMax =
          Seq(
            (j - 1, k - 1), (j - 1, k), (j - 1, k + 1),
            (j, k - 1), (j, k), (j, k + 1),
            (j + 1, k - 1), (j + 1, k), (j + 1, k + 1)
          ).map { case (j, k) => dp(i + 1)(j)(k) }
            .max
        dp(i)(j)(k) = G(i)(j - 1) + G(i)(k - 1) + nMax
      }

      dp(0)(1)(cols)
    }
  }

  import Solution.cherryPickup

  "Example 1: (grid = [[3,1,1],[2,5,1],[1,5,5],[2,1,1]]) -> 24" in {
    val grid = Array(
      Array(3, 1, 1),
      Array(2, 5, 1),
      Array(1, 5, 5),
      Array(2, 1, 1))
    cherryPickup(grid) shouldBe 24
    //Explanation:
    // Cherries taken by Robot #1, (3 + 2 + 5 + 2) = 12.
    // Cherries taken by Robot #2, (1 + 5 + 5 + 1) = 12.
    // Total of cherries: 12 + 12 = 24.
  }
  "Example 2: (grid = [[1,0,0,0,0,0,1],[2,0,0,0,0,3,0],[2,0,9,0,0,0,0],[0,3,0,5,4,0,0],[1,0,2,3,0,0,6]]) -> 28" in {
    val grid = Array(
      Array(1, 0, 0, 0, 0, 0, 1),
      Array(2, 0, 0, 0, 0, 3, 0),
      Array(2, 0, 9, 0, 0, 0, 0),
      Array(0, 3, 0, 5, 4, 0, 0),
      Array(1, 0, 2, 3, 0, 0, 6))
    cherryPickup(grid) shouldBe 28
    //Explanation: Path of robot #1 and #2 are described in color green and blue respectively.
    // Cherries taken by Robot #1, (1 + 0 + 9 + 5 + 2) = 17.
    // Cherries taken by Robot #2, (1 + 3 + 0 + 4 + 3) = 11.
    // Total of cherries: 17 + 11 = 28.
  }
  "Example 3: (grid = [[1,0,0,3],[0,0,0,3],[0,0,3,3],[9,0,3,3]]) -> 22" in {
    val grid = Array(
      Array(1, 0, 0, 3),
      Array(0, 0, 0, 3),
      Array(0, 0, 3, 3),
      Array(9, 0, 3, 3))
    cherryPickup(grid) shouldBe 22
  }
  "Example 4: (grid = [[1,1],[1,1]]) -> 4" in {
    val grid = Array(
      Array(1, 1),
      Array(1, 1))
    cherryPickup(grid) shouldBe 4
  }

  "(grid = [[1,5],[1,5]]) -> 12" in {
    val grid = Array(
      Array(1, 5),
      Array(1, 5))
    cherryPickup(grid) shouldBe 12
  }
  "(grid = [[5,1],[5,1]]) -> 12" in {
    val grid = Array(
      Array(5, 1),
      Array(5, 1))
    cherryPickup(grid) shouldBe 12
  }
  "(grid = [[1,1,1],[1,1,1],[1,1,1]]) -> 6" in {
    val grid = Array(
      Array(1, 1, 1),
      Array(1, 1, 1),
      Array(1, 1, 1))
    cherryPickup(grid) shouldBe 6
  }
  "(grid = [[1,2,3],[4,5,6],[7,8,9]]) -> 32" in {
    val grid = Array(
      Array(1, 2, 3),
      Array(4, 5, 6),
      Array(7, 8, 9))
    cherryPickup(grid) shouldBe 32
  }

  "(grid = [70 x 70 filled with 0]) -> 0" in {
    val grid = Array.fill(70, 70)(0)
    cherryPickup(grid) shouldBe 0
  }
  "(grid = [70 x 70 filled with 1]) -> 140" in {
    val grid = Array.fill(70, 70)(1)
    cherryPickup(grid) shouldBe 140
  }

}
