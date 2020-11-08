package contest.w213

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class w213_1 extends AnyWordSpec with Matchers {

  /**
   * <h3>5554. Check Array Formation Through Concatenation</h3>
   *
   * You are given an array of <b>distinct</b> integers `arr` and an array of integer arrays `pieces`,
   * where the integers in `pieces` are <b>distinct</b>.
   * Your goal is to form `arr` by concatenating the arrays in `pieces` <b>in any order</b>.
   * However, you are <b>not</b> allowed to reorder the integers in each array `pieces[i]`.
   *
   * Return `true` <em>if it is possible to form the array `arr` from `pieces`</em>.
   * Otherwise, return `false`.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= pieces.length <= arr.length <= 100`
   * <li> `sum(pieces[i].length) == arr.length`
   * <li> `1 <= pieces[i].length <= arr.length`
   * <li> `1 <= arr[i], pieces[i][j] <= 100`
   * <li> The integers in `arr` are <b>distinct</b>.
   * <li> The integers in `pieces` are <b>distinct</b>
   * (i.e., If we flatten pieces in a 1D array, all the integers in this array are distinct).
   * </ul>
   */
  object Solution {
    def canFormArray(arr: Array[Int], pieces: Array[Array[Int]]): Boolean = {
      @scala.annotation.tailrec
      def loop(a: Array[Int], ps: Array[Array[Int]]): Boolean = {
        if (a.length < 1) true
        else ps.indexWhere(p => p.nonEmpty && a.startsWith(p)) match {
          case -1 => false
          case p  => loop(a.drop(ps(p).length), ps.updated(p, Array()))
        }
      }

      loop(arr, pieces)
    }
  }

  import Solution.canFormArray

  "Example 1: (arr = [85], pieces = [[85]]) -> true" in {
    canFormArray(Array(85), Array(Array(85))) shouldBe true
  }
  "Example 2: (arr = [15,88], pieces = [[88],[15]]) -> true" in {
    canFormArray(Array(15, 88), Array(Array(88), Array(15))) shouldBe true
    // Explanation: Concatenate [15] then [88]
  }
  "Example 3: (arr = [49,18,16], pieces = [[16,18,49]]) -> false" in {
    canFormArray(Array(49, 18, 16), Array(Array(16, 18, 49))) shouldBe false
    // Explanation: Even though the numbers match, we cannot reorder pieces[0].
  }
  "Example 4: (arr = [91,4,64,78], pieces = [[78],[4,64],[91]]) -> true" in {
    canFormArray(Array(91), Array(Array(78), Array(4, 64), Array(91))) shouldBe true
    // Explanation: Concatenate [91] then [4,64] then [78]
  }
  "Example 5: (arr = [1,3,5,7], pieces = [[2,4,6,8]]) -> false" in {
    canFormArray(Array(1, 3, 5, 7), Array(Array(2, 4, 6, 8))) shouldBe false
  }
}
