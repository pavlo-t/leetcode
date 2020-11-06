package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class d2020_11_06 extends AnyWordSpec with Matchers {

  /**
   * <h3>Find the Smallest Divisor Given a Threshold</h3>
   *
   * Given an array of integers `nums` and an integer `threshold`,
   * we will choose a positive integer divisor and divide all the array by it and sum the result of the division.
   * Find the <b>smallest</b> divisor such that the result mentioned above is less than or equal to `threshold`.
   *
   * Each result of division is rounded to the nearest integer greater than or equal to that element.
   * (For example: 7/3 = 3 and 10/2 = 5).
   *
   * It is guaranteed that there will be an answer.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= nums.length <= 50_000`
   * <li> `1 <= nums[i] <= 1_000_000`
   * <li> `nums.length <= threshold <= 1_000_000`
   * </ul>
   */
  object Solution {
    def smallestDivisor(nums: Array[Int], threshold: Int): Int = {
      @scala.annotation.tailrec
      def binarySearch(l: Int, r: Int): Int = {
        if (l == r) r
        else {
          val m = l + (r - l) / 2
          if (nums.foldLeft(0)((acc, i) => acc + i /^ m) <= threshold)
            binarySearch(l, m)
          else binarySearch(m + 1, r)
        }
      }
      binarySearch(1, nums.max)
    }

    private implicit class DivideRoundUp(i: Int) {
      def /^(d: Int): Int = if (i % d == 0) i / d else i / d + 1
    }
  }

  object SolutionBruteForce {
    def smallestDivisor(nums: Array[Int], threshold: Int): Int = {
      @scala.annotation.tailrec
      def loop(d: Int): Int = {
        if (nums.foldLeft(0)((acc, v) => acc + v /^ d) <= threshold) d
        else loop(d + 1)
      }
      loop(1)
    }

    private implicit class DivideRoundUp(i: Int) {
      def /^(d: Int): Int = if (i % d == 0) i / d else i / d + 1
    }
  }

  import Solution.smallestDivisor

  "Example 1: (nums = [1,2,5,9], threshold = 6) -> 5" in {
    smallestDivisor(Array(1, 2, 5, 9), 6) shouldBe 5
    // Explanation: We can get a sum to 17 (1+2+5+9) if the divisor is 1.
    //   If the divisor is 4 we can get a sum to 7 (1+1+2+3)
    //   and if the divisor is 5 the sum will be 5 (1+1+1+2).
  }
  "Example 2: (nums = [2,3,5,7,11], threshold = 11) -> 3" in {
    smallestDivisor(Array(2, 3, 5, 7, 11), 11) shouldBe 3
  }
  "Example 3: (nums = [19], threshold = 5) -> 4" in {
    smallestDivisor(Array(19), 5) shouldBe 4
  }

  "Test 33: (nums = [962551,933661,905225,923035,990560], threshold = 10) -> 495280" in {
    smallestDivisor(Array(962551, 933661, 905225, 923035, 990560), 10) shouldBe 495280
  }

  "(nums = [50_000 elements, threshold = 50_000]) -> 1" in {
    val nums = Array.fill(50_000)(1)
    smallestDivisor(nums, 50_000) shouldBe 1
  }
  "(nums = [50_000 elements, threshold = 50_001]) -> 49_999" in {
    val nums = (1 to 50_000).toArray
    smallestDivisor(nums, 50_001) shouldBe 49_999
  }
  "(nums = [50_000 elements, threshold = 50_000]) -> 50_000" in {
    val nums = (1 to 50_000).toArray
    smallestDivisor(nums, 50_000) shouldBe 50_000
  }
}
