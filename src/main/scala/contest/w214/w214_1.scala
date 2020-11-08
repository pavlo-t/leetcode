package contest.w214

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class w214_1 extends AnyWordSpec with Matchers {

  /**
   * <h3>5561. Get Maximum in Generated Array</h3>
   *
   * You are given an integer `n`.
   * An array `nums` of length `n + 1` is generated in the following way:<ul>
   * <li> `nums[0] = 0`
   * <li> `nums[1] = 1`
   * <li> `nums[2 * i] = nums[i]` when `2 <= 2 * i <= n`
   * <li> `nums[2 * i + 1] = nums[i] + nums[i + 1]` when `2 <= 2 * i + 1 <= n`
   * </ul>
   *
   * Return <em>the <b>maximum</b> integer in the array `nums`</em>.
   *
   * <b>Constraints:</b><ul>
   * <li> `0 <= n <= 100`
   * </ul>
   */
  object Solution {
    def getMaximumGenerated(n: Int): Int = {
      val nums = Array.ofDim[Int](n + 1)
      for (i <- nums.indices) {
        nums(i) =
          if (i == 0) 0
          else if (i == 1) 1
          else if (i % 2 == 0) nums(i / 2)
          else nums(i / 2) + nums(i / 2 + 1)
      }
      nums.max
    }
  }

  import Solution.getMaximumGenerated

  "Example 1: (n = 7) -> 3" in {
    getMaximumGenerated(7) shouldBe 3
    //Explanation: According to the given rules:
    //  nums[0] = 0
    //  nums[1] = 1
    //  nums[(1 * 2) = 2] = nums[1] = 1
    //  nums[(1 * 2) + 1 = 3] = nums[1] + nums[2] = 1 + 1 = 2
    //  nums[(2 * 2) = 4] = nums[2] = 1
    //  nums[(2 * 2) + 1 = 5] = nums[2] + nums[3] = 1 + 2 = 3
    //  nums[(3 * 2) = 6] = nums[3] = 2
    //  nums[(3 * 2) + 1 = 7] = nums[3] + nums[4] = 2 + 1 = 3
    //Hence, nums = [0,1,1,2,1,3,2,3], and the maximum is 3.
  }
  "Example 2: (n = 2) -> 1" in {
    getMaximumGenerated(2) shouldBe 1
    //Explanation: According to the given rules, the maximum between nums[0], nums[1], and nums[2] is 1.
  }
  "Example 3: (n = 3) -> 2" in {
    getMaximumGenerated(3) shouldBe 2
    //Explanation: According to the given rules, the maximum between nums[0], nums[1], nums[2], and nums[3] is 2.
  }
  "(n = 0) -> 0" in {
    getMaximumGenerated(0) shouldBe 0
  }
  "(n = 100) -> 21" in {
    getMaximumGenerated(100) shouldBe 21
  }
}
