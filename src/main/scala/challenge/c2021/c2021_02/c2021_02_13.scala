package challenge.c2021.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3638/]]
 */
class c2021_02_13 extends AnyWordSpec with Matchers {
  /**
   * === Shortest Path in Binary Matrix ===
   *
   * In an N by N square grid, each cell is either empty (0) or blocked (1).
   *
   * A ''clear path from top-left to bottom-right'' has length `k`
   * if and only if it is composed of cells `C_1, C_2, ..., C_k` such that:
   *  - Adjacent cells `C_i` and `C_{i+1}` are connected 8-directionally
   *    (ie., they are different and share an edge or corner)
   *  - `C_1` is at location `(0, 0)` (ie. has value `grid[0][0]`)
   *  - `C_k` is at location `(N-1, N-1)` (ie. has value `grid[N-1][N-1]`)
   *  - If `C_i` is located at `(r, c)`, then `grid[r][c]` is empty (ie. `grid[r][c] == 0`).
   *
   * Return the length of the shortest such clear path from top-left to bottom-right.
   * If such a path does not exist, return -1.
   *
   * '''Note:'''
   *  - `1 <= grid.length == grid[0].length <= 100`
   *  - `grid[r][c]` is `0` or `1`
   */
  object Solution {
    def shortestPathBinaryMatrix(grid: Array[Array[Int]]): Int = {
      val R = grid.length - 1
      val C = grid(0).length - 1

      if (grid(0)(0) == 1 || grid(R)(C) == 1) -1
      else {
        def nextSteps(r: Int, c: Int, seen: Set[(Int, Int)]): Seq[(Int, Int)] =
          for {
            nr <- (r - 1).max(0) to (r + 1).min(R)
            nc <- (c - 1).max(0) to (c + 1).min(C)
            if grid(nr)(nc) == 0 && !seen.contains((nr, nc))
          } yield (nr, nc)

        @scala.annotation.tailrec
        def rec(todo: Seq[(Int, Int, Int)], seen: Set[(Int, Int)]): Int = todo match {
          case Nil                => -1
          case (R, C, steps) +: _ => steps
          case (r, c, s) +: rest  =>
            val nexts = nextSteps(r, c, seen)
            val ns = s + 1
            rec(rest ++ nexts.map { case (r, c) => (r, c, ns) }, seen ++ nexts.toSet)
        }

        rec(Seq((0, 0, 1)), Set((0, 0)))
      }
    }
  }

  import Solution.shortestPathBinaryMatrix

  "Example 1: ([[0,1],[1,0]]) -> 2" in {
    val grid = Array(
      Array(0, 1),
      Array(1, 0))
    shortestPathBinaryMatrix(grid) shouldBe 2
  }
  "Example 2: ([[0,0,0],[1,1,0],[1,1,0]]) -> 4" in {
    val grid = Array(
      Array(0, 0, 0),
      Array(1, 1, 0),
      Array(1, 1, 0))
    shortestPathBinaryMatrix(grid) shouldBe 4
  }

  "([[0]]) -> 1" in (shortestPathBinaryMatrix(Array(Array(0))) shouldBe 1)
  "([[1]]) -> -1" in (shortestPathBinaryMatrix(Array(Array(1))) shouldBe -1)
  "([[0,0,0],[0,0,0],[0,0,1]]) -> -1" in {
    val grid = Array(
      Array(0, 0, 0),
      Array(0, 0, 0),
      Array(0, 0, 1))
    shortestPathBinaryMatrix(grid) shouldBe -1
  }
  "([[1,0,0],[0,0,0],[0,0,0]]) -> -1" in {
    val grid = Array(
      Array(1, 0, 0),
      Array(0, 0, 0),
      Array(0, 0, 0))
    shortestPathBinaryMatrix(grid) shouldBe -1
  }
  "([[0,0,0],[0,0,0],[0,0,0]]) -> 3" in {
    val grid = Array(
      Array(0, 0, 0),
      Array(0, 0, 0),
      Array(0, 0, 0))
    shortestPathBinaryMatrix(grid) shouldBe 3
  }
  "([100 x 100 0s]) -> 100" in {
    val grid = Array.fill(100, 100)(0)
    shortestPathBinaryMatrix(grid) shouldBe 100
  }

}
