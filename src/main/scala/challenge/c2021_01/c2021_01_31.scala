package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/583/week-5-january-29th-january-31st/3623/]]
 */
//noinspection DuplicatedCode
class c2021_01_31 extends AnyWordSpec with Matchers {
  /**
   * === Next Permutation ===
   *
   * Implement '''next permutation''', which rearranges numbers into the lexicographically next greater permutation of numbers.
   *
   * If such an arrangement is not possible, it must rearrange it as the lowest possible order (i.e., sorted in ascending order).
   *
   * The replacement must be [[http://en.wikipedia.org/wiki/In-place_algorithm in place]] and use only constant extra memory.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100`
   *  - `0 <= nums[i] <= 100`
   */
  object Solution {
    def nextPermutation(nums: Array[Int]): Unit = {
      if (nums.length > 1) {
        var i = nums.length - 2
        while (i >= 0 && nums(i) >= nums(i + 1)) i -= 1

        if (i < 0) nums.sortInPlace()
        else {
          val tmp = nums(i)
          val j = nums.lastIndexWhere(_ > tmp)
          nums(i) = nums(j)
          nums(j) = tmp
          reverseInPlaceSuffix(nums, i + 1, nums.length - 1)
        }
      }
    }

    @scala.annotation.tailrec
    private def reverseInPlaceSuffix(nums: Array[Int], l: Int, r: Int): Unit = {
      if (l < r) {
        val tmp = nums(l)
        nums(l) = nums(r)
        nums(r) = tmp
        reverseInPlaceSuffix(nums, l + 1, r - 1)
      }
    }
  }

  import Solution.nextPermutation

  "Example 1: (nums = [1,2,3]) -> [1,3,2]" in {
    val nums = Array(1, 2, 3)
    nextPermutation(nums)
    nums shouldBe Array(1, 3, 2)
  }
  "Example 2: (nums = [3,2,1]) -> [1,2,3]" in {
    val nums = Array(3, 2, 1)
    nextPermutation(nums)
    nums shouldBe Array(1, 2, 3)
  }
  "Example 3: (nums = [1,1,5]) -> [1,5,1]" in {
    val nums = Array(1, 1, 5)
    nextPermutation(nums)
    nums shouldBe Array(1, 5, 1)
  }
  "Example 4: (nums = [1]) -> [1]" in {
    val nums = Array(1)
    nextPermutation(nums)
    nums shouldBe Array(1)
  }

  "(nums = [1,5,1]) -> [5,1,1]" in {
    val nums = Array(1, 5, 1)
    nextPermutation(nums)
    nums shouldBe Array(5, 1, 1)
  }
  "(nums = [1,2,3,4]) -> 24 permutations -> [1,2,3,4]" in {
    val nums = Array(1, 2, 3, 4)
    nextPermutation(nums)
    nums shouldBe Array(1, 2, 4, 3)
    nextPermutation(nums)
    nums shouldBe Array(1, 3, 2, 4)
    nextPermutation(nums)
    nums shouldBe Array(1, 3, 4, 2)
    nextPermutation(nums)
    // Array(1, 4, 3, 3)
    nums shouldBe Array(1, 4, 2, 3)
    nextPermutation(nums)
    nums shouldBe Array(1, 4, 3, 2)
    nextPermutation(nums)
    nums shouldBe Array(2, 1, 3, 4)
    nextPermutation(nums)
    nums shouldBe Array(2, 1, 4, 3)
    nextPermutation(nums)
    nums shouldBe Array(2, 3, 1, 4)
    nextPermutation(nums)
    nums shouldBe Array(2, 3, 4, 1)
    nextPermutation(nums)
    nums shouldBe Array(2, 4, 1, 3)
    nextPermutation(nums)
    nums shouldBe Array(2, 4, 3, 1)
    nextPermutation(nums)
    nums shouldBe Array(3, 1, 2, 4)
    nextPermutation(nums)
    nums shouldBe Array(3, 1, 4, 2)
    nextPermutation(nums)
    nums shouldBe Array(3, 2, 1, 4)
    nextPermutation(nums)
    nums shouldBe Array(3, 2, 4, 1)
    nextPermutation(nums)
    nums shouldBe Array(3, 4, 1, 2)
    nextPermutation(nums)
    nums shouldBe Array(3, 4, 2, 1)
    nextPermutation(nums)
    nums shouldBe Array(4, 1, 2, 3)
    nextPermutation(nums)
    nums shouldBe Array(4, 1, 3, 2)
    nextPermutation(nums)
    nums shouldBe Array(4, 2, 1, 3)
    nextPermutation(nums)
    nums shouldBe Array(4, 2, 3, 1)
    nextPermutation(nums)
    nums shouldBe Array(4, 3, 1, 2)
    nextPermutation(nums)
    nums shouldBe Array(4, 3, 2, 1)
    nextPermutation(nums)
    nums shouldBe Array(1, 2, 3, 4)
  }

}
