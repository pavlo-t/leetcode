package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_11_10 extends AnyWordSpec with Matchers {

  /**
   * <h3>Flipping an Image</h3>
   *
   * Given a binary matrix `A`, we want to flip the image horizontally, then invert it, and return the resulting image.
   *
   * To flip an image horizontally means that each row of the image is reversed.
   * For example, flipping `[1, 1, 0]` horizontally results in `[0, 1, 1]`.
   *
   * To invert an image means that each `0` is replaced by `1`, and each `1` is replaced by `0`.
   * For example, inverting `[0, 1, 1]` results in `[1, 0, 0]`.
   *
   * <b>Notes:</b><ul>
   * <li> `1 <= A.length = A[0].length <= 20`
   * <li> `0 <= A[i][j] <= 1`
   * </ul>
   */
  object Solution {
    def flipAndInvertImage(A: Array[Array[Int]]): Array[Array[Int]] = {
      val len = A(0).length
      for (row <- A; i <- 0 until (len + 1) / 2) {
        val tmp = row(i) ^ 1
        row(i) = row(len - i - 1) ^ 1
        row(len - i - 1) = tmp
      }
      A
    }
  }
  object SolutionBuiltins1BinaryOr_ {
    def flipAndInvertImage(A: Array[Array[Int]]): Array[Array[Int]] =
      A.map(_.reverse.map(1 ^ _))
  }
  object SolutionBuiltins1Minus_ {
    def flipAndInvertImage(A: Array[Array[Int]]): Array[Array[Int]] =
      A.map(_.reverse.map(1 - _))
  }
  object SolutionBuiltinsMy {
    def flipAndInvertImage(A: Array[Array[Int]]): Array[Array[Int]] =
      A.map(_.reverse.map(i => if (i == 0) 1 else 0))
  }

  import Solution.flipAndInvertImage

  "Example 1: ([[1,1,0],[1,0,1],[0,0,0]]) -> [[1,0,0],[0,1,0],[1,1,1]]" in {
    val A = Array(Array(1, 1, 0), Array(1, 0, 1), Array(0, 0, 0))
    val expected = Array(Array(1, 0, 0), Array(0, 1, 0), Array(1, 1, 1))

    flipAndInvertImage(A) shouldBe expected
    //Explanation: First reverse each row: [[0,1,1],[1,0,1],[0,0,0]].
    //Then, invert the image: [[1,0,0],[0,1,0],[1,1,1]]
  }
  "Example 2: ([[1,1,0,0],[1,0,0,1],[0,1,1,1],[1,0,1,0]]) -> [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]" in {
    val A = Array(Array(1, 1, 0, 0), Array(1, 0, 0, 1), Array(0, 1, 1, 1), Array(1, 0, 1, 0))
    val expected = Array(Array(1, 1, 0, 0), Array(0, 1, 1, 0), Array(0, 0, 0, 1), Array(1, 0, 1, 0))

    flipAndInvertImage(A) shouldBe expected
    //Explanation: First reverse each row: [[0,0,1,1],[1,0,0,1],[1,1,1,0],[0,1,0,1]].
    //Then invert the image: [[1,1,0,0],[0,1,1,0],[0,0,0,1],[1,0,1,0]]
  }

}
