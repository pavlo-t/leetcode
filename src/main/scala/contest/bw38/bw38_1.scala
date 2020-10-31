package contest.bw38

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class bw38_1 extends AnyWordSpec with Matchers {

  /**
   * <h3>5539. Sort Array by Increasing Frequency</h3>
   *
   * Given an array of integers `nums`, sort the array in <b>increasing</b> order based on the frequency of the values.
   * If multiple values have the same frequency, sort them in <b>decreasing</b> order.
   *
   * Return the <em>sorted array</em>.
   *
   * <b>Constraints:</b><ul>
   * <li> 1 <= nums.length <= 100
   * <li> -100 <= nums[i] <= 100
   * </ul>
   */
  object Solution {
    def frequencySort(nums: Array[Int]): Array[Int] = {
      val counts = collection.mutable.Map[Int, Int]().withDefaultValue(0)
      for (n <- nums) counts(n) += 1
      counts
        .toArray
        .sortBy(-_._1)
        .sortBy(_._2)
        .flatMap { case (n, f) => Array.fill(f)(n) }
    }
  }

  import Solution.frequencySort

  "Example 1: ([1,1,2,2,2,3]) -> [3,1,1,2,2,2]" in {
    val nums = Array(1, 1, 2, 2, 2, 3)
    val expected = Array(3, 1, 1, 2, 2, 2)

    frequencySort(nums) shouldBe expected
    // Explanation: '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.
  }
  "Example 2: ([2,3,1,3,2]) -> [1,3,3,2,2]" in {
    val nums = Array(2, 3, 1, 3, 2)
    val expected = Array(1, 3, 3, 2, 2)

    frequencySort(nums) shouldBe expected
    // Explanation: '2' and '3' both have a frequency of 2, so they are sorted in decreasing order.
  }
  "Example 3: ([-1,1,-6,4,5,-6,1,4,1]) -> [5,-1,4,4,-6,-6,1,1,1]" in {
    val nums = Array(-1, 1, -6, 4, 5, -6, 1, 4, 1)
    val expected = Array(5, -1, 4, 4, -6, -6, 1, 1, 1)

    frequencySort(nums) shouldBe expected
  }
}
