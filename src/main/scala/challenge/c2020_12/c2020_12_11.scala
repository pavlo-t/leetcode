package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/570/week-2-december-8th-december-14th/3562/]]
 */
//noinspection DuplicatedCode
class c2020_12_11 extends AnyWordSpec with Matchers {

  /**
   * === Remove Duplicates from Sorted Array II ===
   *
   * Given a sorted array ''nums'', remove the duplicates [[https://en.wikipedia.org/wiki/In-place_algorithm in-place]]
   * such that duplicates appeared at most ''twice'' and return the new length.
   *
   * Do not allocate extra space for another array;
   * you must do this by '''modifying the input array''' [[https://en.wikipedia.org/wiki/In-place_algorithm in-place]]
   * with `O(1)` extra memory.
   *
   * '''Clarification:'''
   *
   * Confused why the returned value is an integer, but your answer is an array?
   *
   * Note that the input array is passed in by '''reference''',
   * which means a modification to the input array will be known to the caller.
   *
   * Internally you can think of this:
   *
   * {{{
   * // nums is passed in by reference. (i.e., without making a copy)
   * int len = removeDuplicates(nums);
   *
   * // any modification to nums in your function would be known by the caller.
   * // using the length returned by your function, it prints the first len elements.
   * for (int i = 0; i < len; i++) {
   * print(nums[i]);
   * }
   * }}}
   *
   * '''Constraints:'''
   *  - `0 <= nums.length <= 30_000`
   *  - `-10_000 <= nums[i] <= 10_000`
   *  - `nums` is sorted in ascending order.
   */
  object Solution {
    def removeDuplicates(nums: Array[Int]): Int =
      if (nums.length < 2) nums.length
      else {
        @scala.annotation.tailrec
        def loop(l: Int, r: Int): Int =
          if (r == nums.length) l + 2
          else if (nums(l) != nums(r)) {
            nums(l + 2) = nums(r)
            loop(l + 1, r + 1)
          }
          else loop(l, r + 1)

        loop(0, 2)
      }
  }

  object SolutionTwoPointersImproved {
    def removeDuplicates(nums: Array[Int]): Int =
      if (nums.length < 2) nums.length
      else {
        var j = 2
        for (i <- j until nums.length) {
          if (nums(j - 2) != nums(i)) {
            nums(j) = nums(i)
            j += 1
          }
        }
        j
      }
  }

  /** [[https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii/solution/]] */
  object SolutionTwoPointers {
    def removeDuplicates(nums: Array[Int]): Int =
      if (nums.length == 0) 0
      else {
        var j = 1
        var count = 1

        for (i <- 1 until nums.length) {
          if (nums(i - 1) == nums(i))
            count += 1
          else
            count = 1

          if (count <= 2) {
            nums(j) = nums(i)
            j += 1
          }
        }

        j
      }
  }
  object SolutionOverwriteAndSort {
    def removeDuplicates(nums: Array[Int]): Int = {
      var last = -10001
      var lastCount = 1
      var droppedCount = 0
      for (i <- nums.indices) {
        if (nums(i) == last) {
          lastCount += 1
          if (lastCount > 2) {
            droppedCount += 1
            nums(i) = Int.MaxValue
          }
        } else {
          last = nums(i)
          lastCount = 1
        }
      }

      nums.sortInPlace()
      nums.length - droppedCount
    }
  }

  import Solution.removeDuplicates

  "Example 1: (nums = [1,1,1,2,2,3]) -> Output: 5, nums = [1,1,2,2,3]" in {
    val nums = Array(1, 1, 1, 2, 2, 3)

    val newSize = removeDuplicates(nums)

    newSize shouldBe 5
    nums.take(newSize) shouldBe Array(1, 1, 2, 2, 3)
    //Explanation:
    // Your function should return length = 5,
    // with the first five elements of nums being 1, 1, 2, 2 and 3 respectively.
    // It doesn't matter what you leave beyond the returned length.
  }
  "Example 2: (nums = [0,0,1,1,1,1,2,3,3]) -> Output: 7, nums = [0,0,1,1,2,3,3]" in {
    val nums = Array(0, 0, 1, 1, 1, 1, 2, 3, 3)

    val newSize = removeDuplicates(nums)

    newSize shouldBe 7
    nums.take(newSize) shouldBe Array(0, 0, 1, 1, 2, 3, 3)
    //Explanation:
    // Your function should return length = 7,
    // with the first seven elements of nums being modified to 0, 0, 1, 1, 2, 3 and 3 respectively.
    // It doesn't matter what values are set beyond the returned length.
  }

  "(nums = []) -> Output: 0, nums = []" in {
    val nums = Array[Int]()

    val newSize = removeDuplicates(nums)

    newSize shouldBe 0
    nums.take(newSize) shouldBe Array()
  }

  "(nums = Array.fill(30_000)(1)) -> Output: 2, nums = [1,1]" in {
    val nums = Array.fill(30000)(1)

    val newSize = removeDuplicates(nums)

    newSize shouldBe 2
    nums.take(newSize) shouldBe Array(1, 1)
  }

}
