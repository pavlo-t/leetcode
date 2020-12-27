package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3580/]]
 */
//noinspection DuplicatedCode,NameBooleanParameters
class c2020_12_25 extends AnyWordSpec with Matchers {
  /**
   * === Diagonal Traverse ===
   *
   * Given a matrix of `M x N` elements (`M` rows, `N` columns),
   * return all elements of the matrix in diagonal order.
   *
   * '''Note:'''
   *  - The total number of elements of the given matrix will not exceed `10,000`.
   */
  object SolutionFold {
    private sealed trait Dir
    private case object U extends Dir
    private case object D extends Dir

    def findDiagonalOrder(M: Array[Array[Int]]): Array[Int] = {
      if (M.isEmpty || M(0).isEmpty) Array()
      else {
        val rows = M.length
        val cols = M(0).length

        val LR = rows - 1
        val LC = cols - 1

        (0 until (rows * cols)).foldLeft((Seq[Int](), U: Dir, 0, 0)) { case ((rsf, dir, r, c), _) =>
          val nRsf = rsf.appended(M(r)(c))
          (dir, r, c) match {
            case (U, r, LC) => (nRsf, D, r + 1, c)
            case (U, 0, c)  => (nRsf, D, r, c + 1)
            case (U, r, c)  => (nRsf, U, r - 1, c + 1)
            case (D, LR, c) => (nRsf, U, r, c + 1)
            case (D, r, 0)  => (nRsf, U, r + 1, c)
            case (D, r, c)  => (nRsf, D, r + 1, c - 1)
          }
        }._1.toArray
      }
    }
  }

  object Solution {
    private sealed trait Dir
    private case object U extends Dir
    private case object D extends Dir

    def findDiagonalOrder(M: Array[Array[Int]]): Array[Int] = {
      if (M.isEmpty || M(0).isEmpty) Array()
      else {
        val rows = M.length
        val cols = M(0).length
        val result = Array.fill(rows * cols)(0)

        val LR = rows - 1
        val LC = cols - 1

        @scala.annotation.tailrec
        def rec(i: Int, dir: Dir, r: Int, c: Int): Array[Int] = {
          result(i) = M(r)(c)
          val ni = i + 1

          if (ni == result.length) result
          else (dir, r, c) match {
            case (U, r, LC) => rec(ni, D, r + 1, c)
            case (U, 0, c)  => rec(ni, D, r, c + 1)
            case (U, r, c)  => rec(ni, U, r - 1, c + 1)
            case (D, LR, c) => rec(ni, U, r, c + 1)
            case (D, r, 0)  => rec(ni, U, r + 1, c)
            case (D, r, c)  => rec(ni, D, r + 1, c - 1)
          }
        }
        rec(0, U, 0, 0)
      }
    }
  }

  object SolutionMyRecInitial {
    def findDiagonalOrder(M: Array[Array[Int]]): Array[Int] = {
      if (M.isEmpty || M(0).isEmpty) Array()
      else {
        val rows = M.length
        val cols = M(0).length
        val result = Array.fill(rows * cols)(0)

        @scala.annotation.tailrec
        def rec(i: Int, up: Boolean, r: Int, c: Int): Array[Int] = {
          result(i) = M(r)(c)
          if (i == result.length - 1) result
          else if (up) {
            if (c == cols - 1) rec(i + 1, false, r + 1, c)
            else if (r == 0) rec(i + 1, false, r, c + 1)
            else rec(i + 1, true, r - 1, c + 1)
          } else {
            if (r == rows - 1) rec(i + 1, true, r, c + 1)
            else if (c == 0) rec(i + 1, true, r + 1, c)
            else rec(i + 1, false, r + 1, c - 1)
          }
        }
        rec(0, true, 0, 0)
      }
    }
  }

  import Solution.findDiagonalOrder

  "Example: ([[1,2,3],[4,5,6],[7,8,9]]) -> [1,2,4,7,5,3,6,8,9]" in {
    val matrix = Array(
      Array(1, 2, 3),
      Array(4, 5, 6),
      Array(7, 8, 9))
    val expected = Array(1, 2, 4, 7, 5, 3, 6, 8, 9)

    findDiagonalOrder(matrix) shouldBe expected
  }

  "([]) -> []" in {
    findDiagonalOrder(Array()) shouldBe Array()
  }
  "([[1]]) -> [1]" in {
    val matrix = Array(Array(1))
    val expected = Array(1)

    findDiagonalOrder(matrix) shouldBe expected
  }
  "([[1],[2],[3]]) -> [1,2,3]" in {
    val matrix = Array(
      Array(1),
      Array(2),
      Array(3))
    val expected = Array(1, 2, 3)

    findDiagonalOrder(matrix) shouldBe expected
  }
  "([[1,2,3]]) -> [1,2,3]" in {
    val matrix = Array(Array(1, 2, 3))
    val expected = Array(1, 2, 3)

    findDiagonalOrder(matrix) shouldBe expected
  }
  "([[1,2],[3,4],[5,6]]) -> [1,2,3,5,4,6]" in {
    val matrix = Array(
      Array(1, 2),
      Array(3, 4),
      Array(5, 6))
    val expected = Array(1, 2, 3, 5, 4, 6)

    findDiagonalOrder(matrix) shouldBe expected
  }
  "Example: ([[1,2,3,4],[5,6,7,8],[9,10,11,12],[13,14,15,16]]) -> " +
    "[1,5,2,3,6,9,13,10,7,4,8,11,14,15,12,16]" in {
    val matrix = Array(
      Array(1, 2, 3, 4),
      Array(5, 6, 7, 8),
      Array(9, 10, 11, 12),
      Array(13, 14, 15, 16))
    val expected = Array(1, 2, 5, 9, 6, 3, 4, 7, 10, 13, 14, 11, 8, 12, 15, 16)

    findDiagonalOrder(matrix) shouldBe expected
  }
  "Example: ([[11,12,13,14],[21,22,23,24],[31,32,33,34],[41,42,43,44]]) -> " +
    "[11,12,21,31,22,13,14,23,32,41,42,33,24,34,43,44]" in {
    val matrix = Array(
      Array(11, 12, 13, 14),
      Array(21, 22, 23, 24),
      Array(31, 32, 33, 34),
      Array(41, 42, 43, 44))
    val expected = Array(11, 12, 21, 31, 22, 13, 14, 23, 32, 41, 42, 33, 24, 34, 43, 44)

    findDiagonalOrder(matrix) shouldBe expected
  }
  "Example: ([[0,1,5,6],[2,4,7,12],[3,8,11,13],[9,10,14,15]]) -> " +
    "[0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]" in {
    val matrix = Array(
      Array(0, 1, 5, 6),
      Array(2, 4, 7, 12),
      Array(3, 8, 11, 13),
      Array(9, 10, 14, 15))
    val expected = Array(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15)

    findDiagonalOrder(matrix) shouldBe expected
  }

}
