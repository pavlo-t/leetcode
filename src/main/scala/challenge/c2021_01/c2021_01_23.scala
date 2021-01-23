package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/582/week-4-january-22nd-january-28th/3614/]]
 */
//noinspection DuplicatedCode
class c2021_01_23 extends AnyWordSpec with Matchers {
  /**
   * === Sort the Matrix Diagonally ===
   *
   * A '''matrix diagonal''' is a diagonal line of cells starting from some cell in either the topmost row
   * or leftmost column and going in the bottom-right direction until reaching the matrix's end.
   * For example, the '''matrix diagonal''' starting from `mat[2][0]`, where `mat` is a `6 x 3` matrix,
   * includes cells `mat[2][0]`, `mat[3][1]`, and `mat[4][2]`.
   *
   * Given an `m x n` matrix `mat` of integers,
   * sort each '''matrix diagonal''' in ascending order and return ''the resulting matrix''.
   *
   * '''Constraints:'''
   *  - `1 <= mat.length, mat[i].length <= 100`
   *  - `1 <= mat[i][j] <= 100`
   */
  object Solution {
    def diagonalSort(mat: Array[Array[Int]]): Array[Array[Int]] = {
      val result = Array.fill(mat.length, mat(0).length)(101)

      def diagonalToSeq(r: Int, c: Int): Seq[Int] =
        if (r == mat.length || c == mat(0).length) Nil
        else mat(r)(c) +: diagonalToSeq(r + 1, c + 1)

      @scala.annotation.tailrec
      def writeToResult(r: Int, c: Int, seq: Seq[Int]): Unit =
        if (r < mat.length && c < mat(0).length) {
          result(r)(c) = seq.head
          writeToResult(r + 1, c + 1, seq.tail)
        }

      def sortDiagonal(r: Int, c: Int): Unit =
        writeToResult(r, c, diagonalToSeq(r, c).sorted)

      for (r <- result.indices) sortDiagonal(r, 0)
      for (c <- result(0).indices) sortDiagonal(0, c)

      result
    }
  }

  import Solution.diagonalSort

  "Example 1: (mat = [[3,3,1,1],[2,2,1,2],[1,1,1,2]]) -> [[1,1,1,1],[1,2,2,2],[1,2,3,3]]" in {
    val mat = Array(
      Array(3, 3, 1, 1),
      Array(2, 2, 1, 2),
      Array(1, 1, 1, 2))
    val expected = Array(
      Array(1, 1, 1, 1),
      Array(1, 2, 2, 2),
      Array(1, 2, 3, 3))

    diagonalSort(mat) shouldBe expected
  }

  "(mat = 100 x 100 1s) -> 100 x 100 1s" in {
    val mat = Array.fill(100)(Array.fill(100)(1))
    val expected = Array.fill(100)(Array.fill(100)(1))
    diagonalSort(mat) shouldBe expected
  }

  //"non-ergodic game" should {
  //  def next(m: Double): Double = if (util.Random.nextBoolean()) m + m * .5 else m - m * .4
  //
  //  "ensemble average: 1,000,000 players, 20 turns" in {
  //    val players = Array.fill(1_000_000)(100.0)
  //
  //    for (_ <- 1 to 20) players.mapInPlace(next)
  //    val avg = players.sum / players.length
  //    Thread.sleep(20)
  //    println(s"ensemble average result: $avg")
  //
  //    avg should (be > 260.0 and be < 270.0)
  //  }
  //
  //  "time average: 1 player, 1,000 turns" in {
  //    var money = 100.0
  //
  //    for (_ <- 1 to 1_000) money = next(money)
  //    Thread.sleep(20)
  //    println(s"time average result: $money")
  //
  //    money should be < 1.0
  //  }
  //}

}
