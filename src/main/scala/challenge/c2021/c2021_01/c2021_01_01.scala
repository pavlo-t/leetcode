package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3589/]]
 */
//noinspection DuplicatedCode
class c2021_01_01 extends AnyWordSpec with Matchers {
  /**
   * === Check Array Formation Through Concatenation ===
   *
   * You are given an array of '''distinct''' integers `arr` and an array of integer arrays `pieces`,
   * where the integers in `pieces` are '''distinct'''.
   * Your goal is to form `arr` by concatenating the arrays in `pieces` '''in any order'''.
   * However, you are '''not''' allowed to reorder the integers in each array `pieces[i]`.
   *
   * Return `true` ''if it is possible to form the array'' `arr` ''from'' `pieces`. Otherwise, return `false`.
   *
   * '''Constraints:'''
   *  - `1 <= pieces.length <= arr.length <= 100`
   *  - `sum(pieces[i].length) == arr.length`
   *  - `1 <= pieces[i].length <= arr.length`
   *  - `1 <= arr[i], pieces[i][j] <= 100`
   *  - The integers in `arr` are '''distinct'''.
   *  - The integers in `pieces` are '''distinct'''
   *    (i.e., If we flatten pieces in a 1D array, all the integers in this array are distinct).
   */
  object Solution {
    def canFormArray(arr: Array[Int], pieces: Array[Array[Int]]): Boolean = {
      val map = pieces.map(p => (p.head, p)).toMap

      @scala.annotation.tailrec
      def loop(i: Int, j: Int, piece: Array[Int]): Boolean = {
        if (i == arr.length) true
        else if (j == piece.length) map.get(arr(i)) match {
          case None        => false
          case Some(piece) => loop(i + 1, 1, piece)
        }
        else if (arr(i) != piece(j)) false
        else loop(i + 1, j + 1, piece)
      }

      loop(0, 0, Array())
    }
  }

  object SolutionOneByOne {
    def canFormArray(arr: Array[Int], pieces: Array[Array[Int]]): Boolean = {
      @scala.annotation.tailrec
      def rec(arr: Array[Int], pieces: Array[Array[Int]]): Boolean = {
        if (arr.isEmpty) true
        else pieces.indexWhere(arr.startsWith(_)) match {
          case -1 => false
          case i  => rec(arr.drop(pieces(i).length), pieces.patch(i, Nil, 1))
        }
      }

      rec(arr, pieces)
    }
  }

  import Solution.canFormArray

  "Example 1: (arr = [85], pieces = [[85]]) -> true" in {
    canFormArray(Array(85), Array(Array(85))) shouldBe true
  }
  "Example 2: (arr = [15,88], pieces = [[88],[15]]) -> true" in {
    canFormArray(Array(15, 88), Array(Array(88), Array(15))) shouldBe true
    //Explanation: Concatenate [15] then [88]
  }
  "Example 3: (arr = [49,18,16], pieces = [[16,18,49]]) -> false" in {
    canFormArray(Array(49, 18, 16), Array(Array(16, 18, 49))) shouldBe false
    //Explanation: Even though the numbers match, we cannot reorder pieces[0].
  }
  "Example 4: (arr = [91,4,64,78], pieces = [[78],[4,64],[91]]) -> true" in {
    canFormArray(Array(91, 4, 64, 78), Array(Array(78), Array(4, 64), Array(91))) shouldBe true
    //Explanation: Concatenate [91] then [4,64] then [78]
  }
  "Example 5: (arr = [1,3,5,7], pieces = [[2,4,6,8]]) -> false" in {
    canFormArray(Array(1, 3, 5, 7), Array(Array(2, 4, 6, 8))) shouldBe false
  }

  "Test 9: (arr = [1,2,3], pieces = [[2],[1,3]]) -> false" in {
    canFormArray(Array(1, 2, 3), Array(Array(2), Array(1, 3))) shouldBe false
  }

  "(arr = 1 to 100, pieces = (100 to 1).map(Array(_)) -> true" in {
    val arr = (1 to 100).toArray
    val pieces = (100 to 1 by -1).map(Array(_)).toArray
    canFormArray(arr, pieces) shouldBe true
  }

  "(arr = 1 to 10000, pieces = (10000 to 1 by -1).map(Array(_)) -> true" in {
    val size = 10000
    val arr = (1 to size).toArray
    val pieces = (size to 1 by -1).map(Array(_)).toArray
    canFormArray(arr, pieces) shouldBe true
  }
  "(arr = 1 to 10000, pieces = (10000 to 10 by -10).map(i => (i-9 to i)) -> true" in {
    val size = 10000
    val arr = (1 to size).toArray
    val pieces = (size to 10 by -10).map(i => (i - 9 to i).toArray).toArray
    canFormArray(arr, pieces) shouldBe true
  }

}
