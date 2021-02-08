package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3632/]]
 */
//noinspection DuplicatedCode
class c2021_02_w2 extends AnyWordSpec with Matchers {
  /**
   * === Number of Distinct Islands ===
   *
   * Given a non-empty 2D array `grid` of `0`'s and `1`'s, an '''island''' is a group of `1`'s (representing land)
   * connected 4-directionally (horizontal or vertical.)
   * You may assume all four edges of the grid are surrounded by water.
   *
   * Count the number of '''distinct''' islands.
   * An island is considered to be the same as another if and only if one island can be translated
   * (and not rotated or reflected) to equal the other.
   *
   * '''Example 1:'''
   * {{{
   * 11000
   * 11000
   * 00011
   * 00011
   * }}}
   * Given the above grid map, return `1`.
   *
   * '''Example 2:'''
   * {{{
   * 11011
   * 10000
   * 00001
   * 11011
   * }}}
   * Given the above grid map, return `3`.
   *
   * Notice that:
   * {{{
   * 11
   * 1
   * }}}
   * and
   * {{{
   *  1
   * 11
   * }}}
   * are considered different island shapes, because we do not consider reflection / rotation.
   *
   *
   * '''Note:''' The length of each dimension in the given `grid` does not exceed `50`.
   */
  object Solution {
    def numDistinctIslands(grid: Array[Array[Int]]): Int =
      if (grid.length == 0 || grid(0).length == 0) 0
      else {
        val Rows = grid.length
        val Cols = grid(0).length

        def nexts(r: Int, c: Int): Seq[(Int, Int)] =
          Seq((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1))
            .filter { case cell@(r, c) => r >= 0 && r < Rows && c >= 0 && c < Cols }

        @scala.annotation.tailrec
        def see(r0: Int, c0: Int,
                todo: Seq[(Int, Int)],
                seen: Set[(Int, Int)],
                rsf: Set[(Int, Int)]): (Set[(Int, Int)], Set[(Int, Int)]) = todo match {
          case Nil                                 => (seen, rsf)
          case cell +: rest if seen.contains(cell) => see(r0, c0, rest, seen, rsf)
          case (r, c) +: rest if grid(r)(c) == 0   => see(r0, c0, rest, seen + (r -> c), rsf)

          case (r, c) +: rest => see(r0, c0, rest ++ nexts(r, c), seen + (r -> c), rsf + ((r - r0) -> (c - c0)))
        }

        @scala.annotation.tailrec
        def loop(r: Int, c: Int, seen: Set[(Int, Int)], rsf: Set[Set[(Int, Int)]]): Int = {
          if (r == Rows) rsf.size
          else if (c == Cols) loop(r + 1, 0, seen, rsf)
          else if (seen.contains((r, c))) loop(r, c + 1, seen, rsf)
          else if (grid(r)(c) == 0) loop(r, c + 1, seen + (r -> c), rsf)
          else {
            val (ns, nr) = see(r, c, Seq(r -> c), seen, Set())
            loop(r, c + 1, ns, rsf + nr)
          }
        }

        loop(0, 0, Set(), Set())
      }
  }

  object SolutionNumberOfIslands {
    def numDistinctIslands(grid: Array[Array[Int]]): Int =
      if (grid.length == 0 || grid(0).length == 0) 0
      else {
        val Rows = grid.length
        val Cols = grid(0).length

        def nexts(r: Int, c: Int): Seq[(Int, Int)] =
          Seq((r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1))
            .filter { case cell@(r, c) => r >= 0 && r < Rows && c >= 0 && c < Cols }

        @scala.annotation.tailrec
        def see(todo: Seq[(Int, Int)], seen: Set[(Int, Int)]): Set[(Int, Int)] = todo match {
          case Nil                                 => seen
          case cell +: rest if seen.contains(cell) => see(rest, seen)
          case (-1 | Rows, _) +: rest              => see(rest, seen)
          case (_, -1 | Cols) +: rest              => see(rest, seen)
          case (r, c) +: rest if grid(r)(c) == 0   => see(rest, seen + (r -> c))

          case (r, c) +: rest => see(rest ++ nexts(r, c), seen + (r -> c))
        }

        @scala.annotation.tailrec
        def loop(r: Int, c: Int, seen: Set[(Int, Int)], rsf: Int): Int = {
          if (r == Rows) rsf
          else if (c == Cols) loop(r + 1, 0, seen, rsf)
          else if (seen.contains((r, c))) loop(r, c + 1, seen, rsf)
          else if (grid(r)(c) == 0) loop(r, c + 1, seen + (r -> c), rsf)
          else loop(r, c + 1, see(Seq(r -> c), seen), rsf + 1)
        }

        loop(0, 0, Set(), 0)
      }
  }

  import Solution.numDistinctIslands

  "Example 1: ([[1,1,0,0,0],[1,1,0,0,0],[0,0,0,1,1],[0,0,0,1,1]]) -> 1" in {
    val grid = Array(
      Array(1, 1, 0, 0, 0),
      Array(1, 1, 0, 0, 0),
      Array(0, 0, 0, 1, 1),
      Array(0, 0, 0, 1, 1))
    numDistinctIslands(grid) shouldBe 1
  }
  "Example 2: ([[1,1,0,1,1],[1,0,0,0,0],[0,0,0,0,1],[1,1,0,1,1]]) -> 3" in {
    val grid = Array(
      Array(1, 1, 0, 1, 1),
      Array(1, 0, 0, 0, 0),
      Array(0, 0, 0, 0, 1),
      Array(1, 1, 0, 1, 1))
    numDistinctIslands(grid) shouldBe 3
    // 11
    // 1
    // and
    //  1
    // 11
    // are considered different island shapes, because we do not consider reflection / rotation.
  }

  "([[1,1,0,0],[1,1,0,0],[0,0,1,1],[0,0,1,0]]) -> 2" in {
    val grid = Array(
      Array(1, 1, 0, 0),
      Array(1, 1, 0, 0),
      Array(0, 0, 1, 1),
      Array(0, 0, 1, 0))
    numDistinctIslands(grid) shouldBe 2
  }

  "([50x50 1s grid]) -> 1" in {
    val grid = Array.fill(50, 50)(1)
    numDistinctIslands(grid) shouldBe 1
  }
  "([50x50 0s grid]) -> 0" in {
    val grid = Array.fill(50, 50)(0)
    numDistinctIslands(grid) shouldBe 0
  }
  "([50x50 1s and 0s grid]) -> 0" in {
    val grid = Array.ofDim[Int](50, 50)
    for (r <- 0 until 50; c <- 0 until 50)
      grid(r)(c) = if (r % 2 == 0) (c + 1) % 2 else c % 2
    //println(grid.map(_.mkString("[", ",", "]")).mkString("\n"))
    numDistinctIslands(grid) shouldBe 1
  }
}
