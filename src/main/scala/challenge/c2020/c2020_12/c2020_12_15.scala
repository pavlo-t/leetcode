package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3567/]]
 */
//noinspection DuplicatedCode
class c2020_12_15 extends AnyWordSpec with Matchers {

  /**
   * === Squares of a Sorted Array ===
   *
   * Given an integer array `nums` sorted in '''non-decreasing''' order,
   * return ''an array of '''the squares of each number''' sorted in non-decreasing order''.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 10000`
   *  - `-10000 <= nums[i] <= 10000`
   *  - `nums` is sorted in '''non-decreasing''' order.
   */
  object Solution {
    // time O(n); memory O(n)
    def sortedSquares(nums: Array[Int]): Array[Int] = {
      if (nums.length == 1 || nums(0) > 0) nums.map(n => n * n)
      else {
        val result = Array.ofDim[Int](nums.length)

        @scala.annotation.tailrec
        def loop(l: Int, r: Int): Array[Int] = {
          if (l > r) result
          else if (-nums(l) >= nums(r)) {
            result(r - l) = nums(l) * nums(l)
            loop(l + 1, r)
          } else {
            result(r - l) = nums(r) * nums(r)
            loop(l, r - 1)
          }
        }

        loop(0, nums.length - 1)
      }
    }
  }

  // time O(n log n); memory O(n)
  object SolutionScalaBuiltinsBruteForce {
    def sortedSquares(nums: Array[Int]): Array[Int] =
      nums.map(n => n * n).sorted
  }

  import Solution.sortedSquares

  "Example 1: (nums = [-4,-1,0,3,10]) -> [0,1,9,16,100]" in {
    sortedSquares(Array(-4, -1, 0, 3, 10)) shouldBe Array(0, 1, 9, 16, 100)
    //Explanation: After squaring, the array becomes [16,1,0,9,100].
    //After sorting, it becomes [0,1,9,16,100].
  }
  "Example 2: (nums = [-7,-3,2,3,11]) -> [4,9,9,49,121]" in {
    sortedSquares(Array(-7, -3, 2, 3, 11)) shouldBe Array(4, 9, 9, 49, 121)
  }

  "(nums = [-10000,-9998,...,-2,1,3,...,9997,9999]) -> [1,4,9,...,100_000_000]" in {
    val nums = (1 to 10000).map(n => if (n % 2 == 0) -n else n).toArray.sorted
    val expected = (1 to 10000).map(n => n * n).toArray

    sortedSquares(nums) shouldBe expected
  }

  "(nums = [-10000,-9998,...,-2,1,3,...,9997,9999]*100) -> [1,4,9,...,100_000_000]*100" in {
    val repeat = 100

    val nums =
      (1 to 10000)
        .map(n => if (n % 2 == 0) -n else n)
        .flatMap(Array.fill(repeat)(_))
        .toArray
        .sorted
    val expected =
      (1 to 10000)
        .map(n => n * n)
        .flatMap(Array.fill(repeat)(_))
        .toArray

    sortedSquares(nums) shouldBe expected
  }

}
