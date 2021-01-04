package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3586/]]
 */
//noinspection DuplicatedCode
class c2020_12_30 extends AnyWordSpec with Matchers {
  /**
   * === Game of Life ===
   *
   * According to [[https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life Wikipedia's article]]:
   * "The '''Game of Life''', also known simply as '''Life''',
   * is a cellular automaton devised by the British mathematician John Horton Conway in 1970."
   *
   * The board is made up of an `m x n` grid of cells,
   * where each cell has an initial state:
   * '''live''' (represented by a `1`) or '''dead''' (represented by a `0`).
   * Each cell interacts with its eight [[https://en.wikipedia.org/wiki/Moore_neighborhood neighbors]]
   * (horizontal, vertical, diagonal) using the following four rules (taken from the above Wikipedia article):
   *
   *  1. Any live cell with fewer than two live neighbors dies as if caused by under-population.
   *  1. Any live cell with two or three live neighbors lives on to the next generation.
   *  1. Any live cell with more than three live neighbors dies, as if by over-population.
   *  1. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
   *
   * The next state is created by applying the above rules simultaneously to every cell in the current state,
   * where births and deaths occur simultaneously.
   * Given the current state of the `m x n` grid `board`, return ''the next state''.
   *
   * '''Constraints:'''
   *  - `1 <= board.length, board[i].length <= 25`
   *  - `board[i][j]` is `0` or `1`.
   *
   * '''Follow up:'''
   *  - Could you solve it in-place? Remember that the board needs to be updated simultaneously:
   *    You cannot update some cells first and then use their updated values to update other cells.
   *  - In this question, we represent the board using a 2D array. In principle, the board is infinite,
   *    which would cause problems when the active area encroaches upon the border of the array
   *    (i.e., live cells reach the border). How would you address these problems?
   */
  object Solution {
    def gameOfLife(board: Array[Array[Int]]): Unit = {
      val m = board.length
      val n = board(0).length

      for (r <- 0 until m; c <- 0 until n) {
        var neighbors = 0
        for {
          nr <- r - 1 to r + 1 if nr >= 0 && nr < m
          nc <- c - 1 to c + 1 if nc >= 0 && nc < n
        } if (!(nr == r && nc == c)) neighbors += board(nr)(nc) & 1


        board(r)(c) = (board(r)(c), neighbors) match {
          case (0, 3)     => 2 // b10: was 0 becomes 1
          case (0, _)     => 0 // b00: was 0 stays 0
          case (1, 2 | 3) => 3 // b11: was 1 stays 1
          case (1, _)     => 1 // b01: was 1 becomes 0
        }
      }

      for (r <- 0 until m; c <- 0 until n) board(r)(c) >>= 1
    }
  }

  object SolutionO1Space {
    def gameOfLife(board: Array[Array[Int]]): Unit = {
      val m = board.length
      val n = board(0).length

      for (i <- 0 until m; j <- 0 until n) {
        var neighbors = 0
        for {
          ni <- i - 1 to i + 1 if ni >= 0 && ni < m
          nj <- j - 1 to j + 1 if nj >= 0 && nj < n
          if !(ni == i && nj == j)
        } neighbors += board(ni)(nj) % 2

        // 0: was 0 stays 0
        // 1: was 1 stays 1
        // 2: was 0 becomes 1
        // 3: was 1 becomes 0
        board(i)(j) = if (board(i)(j) == 0) {
          if (neighbors == 3) 2
          else 0
        } else {
          if (neighbors < 2 || neighbors > 3) 3
          else 1
        }
      }

      for (i <- 0 until m; j <- 0 until n) board(i)(j) = board(i)(j) match {
        case 3 => 0
        case 2 => 1
        case v => v
      }
    }
  }

  object SolutionOmnSpace {
    def gameOfLife(board: Array[Array[Int]]): Unit = {
      val m = board.length
      val n = board(0).length

      val cc = board.map(_.clone())

      for (i <- 0 until m; j <- 0 until n) {
        var neighbors = 0
        for {
          ni <- i - 1 to i + 1 if ni >= 0 && ni < m
          nj <- j - 1 to j + 1 if nj >= 0 && nj < n
          if !(ni == i && nj == j)
        } neighbors += cc(ni)(nj)
        if (cc(i)(j) == 0 && neighbors == 3)
          board(i)(j) = 1
        else if (cc(i)(j) == 1 && (neighbors < 2 || neighbors > 3))
          board(i)(j) = 0
      }
    }
  }

  object SolutionMyResultAndCopyBack {
    def gameOfLife(board: Array[Array[Int]]): Unit = {
      val m = board.length
      val n = board(0).length

      val result = Array.fill(m, n)(0)
      for (i <- 0 until m; j <- 0 until n) {
        var neighbors = 0
        for {
          ni <- i - 1 to i + 1 if ni >= 0 && ni < m
          nj <- j - 1 to j + 1 if nj >= 0 && nj < n
          if !(ni == i && nj == j)
        } neighbors += board(ni)(nj)
        if (neighbors == 3) result(i)(j) = 1
        else if (neighbors == 2) result(i)(j) = board(i)(j)
      }

      for (i <- 0 until m; j <- 0 until n)
        board(i)(j) = result(i)(j)
    }
  }

  import Solution.gameOfLife

  //noinspection ScalaUnusedSymbol
  private def printBoard(b: Array[Array[Int]]): Unit = println(b.map(_.mkString(",")).mkString("\n"))

  "Example 1: (board = [[0,1,0],[0,0,1],[1,1,1],[0,0,0]]) -> [[0,0,0],[1,0,1],[0,1,1],[0,1,0]]" in {
    val board = Array(
      Array(0, 1, 0),
      Array(0, 0, 1),
      Array(1, 1, 1),
      Array(0, 0, 0))

    gameOfLife(board)

    board shouldBe Array(
      Array(0, 0, 0),
      Array(1, 0, 1),
      Array(0, 1, 1),
      Array(0, 1, 0))
  }
  "Example 2: (board = [[1,1],[1,0]]) -> [[1,1],[1,1]]" in {
    val board = Array(Array(1, 1), Array(1, 0))

    gameOfLife(board)

    board shouldBe Array(Array(1, 1), Array(1, 1))
  }

  "(board = 25 * 25 filled with 0) -> all 0s" in {
    val board = Array.fill(25, 25)(0)

    gameOfLife(board)

    board shouldBe Array.fill(25, 25)(0)
  }
  "(board = 25 * 25 filled with 1) -> all 0s with 1s in the corners" in {
    val board = Array.fill(25, 25)(1)

    gameOfLife(board)

    val expected = Array.fill(25, 25)(0)
    expected(0)(0) = 1
    expected(0)(24) = 1
    expected(24)(0) = 1
    expected(24)(24) = 1
    board shouldBe expected
  }
  "(board = 1000 * 1000 filled with 0) -> all 0s" in {
    val side = 1000
    val board = Array.fill(side, side)(0)

    gameOfLife(board)

    board shouldBe Array.fill(side, side)(0)
  }
  "(board = 1000 * 1000 filled with 1) -> all 0s with 1s in the corners" in {
    val side = 1000
    val board = Array.fill(side, side)(1)

    gameOfLife(board)

    val expected = Array.fill(side, side)(0)
    expected(0)(0) = 1
    expected(0)(side - 1) = 1
    expected(side - 1)(0) = 1
    expected(side - 1)(side - 1) = 1
    board shouldBe expected
  }

  //"iter %" in {
  //  val arr = (0 to 100000).map(_ % 4)
  //  val seq = arr.indices.map(i => arr(i) % 2)
  //}
  //
  //"iter -" in {
  //  val arr = (0 to 100000).map(_ % 4)
  //  val seq = arr.indices.map(i => if (arr(i) < 2) arr(i) else arr(i) - 2)
  //}
}
