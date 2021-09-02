package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/564/week-1-november-1st-november-7th/3520/]]
 */
class d2020_11_05 extends AnyWordSpec with Matchers {

  /**
   * <h3>Minimum Cost to Move Chips to The Same Position</h3>
   *
   * We have `n` chips, where the position of the `i`th chip is `position[i]`.
   *
   * We need to move all the chips to the <b>same position</b>.
   * In one step, we can change the position of the `i`th chip from `position[i]` to:<ul>
   * <li> `position[i] + 2` or `position[i] - 2` with `cost = 0`.
   * <li> `position[i] + 1` or `position[i] - 1` with `cost = 1`.
   * </ul>
   *
   * Return the <em>minimum cost</em> needed to move all the chips to the same position.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= position.length <= 100`
   * <li> `1 <= position[i] <= 1_000_000_000`
   * </ul>
   */
  object Solution {
    import scala.util.chaining._

    def minCostToMoveChips(position: Array[Int]): Int =
      position.foldLeft((0, 0)) { case ((even, odd), p) =>
        if (p % 2 == 0) (even + 1, odd) else (even, odd + 1)
      }.pipe { case (even, odd) => even min odd }
  }

  import Solution.minCostToMoveChips

  "Example 1: (position = [1,2,3]) -> 1" in {
    minCostToMoveChips(Array(1, 2, 3)) shouldBe 1
    // Explanation:
    //   First step: Move the chip at position 3 to position 1 with cost = 0.
    //   Second step: Move the chip at position 2 to position 1 with cost = 1.
    //   Total cost is 1.
  }
  "Example 2: (position = [2,2,2,3,3]) -> 2" in {
    minCostToMoveChips(Array(2, 2, 2, 3, 3)) shouldBe 2
    // Explanation: We can move the two chips at position 3 to position 2. Each move has cost = 1. The total cost = 2.
  }
  "Example 3: (position = [1,1000000000]) -> 1" in {
    minCostToMoveChips(Array(1, 1_000_000_000)) shouldBe 1
  }

  "Test 49: (position = [2,2,2,2,2,2,2]) -> 0" in {
    minCostToMoveChips(Array(2, 2, 2, 2, 2, 2, 2)) shouldBe 0
  }

  "([1]) -> 0" in {
    minCostToMoveChips(Array(1)) shouldBe 0
  }
  "([1,2,3,..100]) -> 50" in {
    val position = Array.ofDim[Unit](100).zipWithIndex.map(_._2 + 1)

    minCostToMoveChips(position) shouldBe 50
  }
}
