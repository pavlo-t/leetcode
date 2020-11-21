package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/566/week-3-november-15th-november-21st/3537/]]
 */
//noinspection DuplicatedCode
class d2020_11_20 extends AnyWordSpec with Matchers {

  /**
   * === Search in Rotated Sorted Array II ===
   *
   * Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand.
   *
   * (i.e., `[0,0,1,2,2,5,6]` might become `[2,5,6,0,0,1,2]`).
   *
   * You are given a target value to search.
   * If found in the array return `true`, otherwise return `false`.
   *
   * '''Follow up:'''
   *  - This is a follow up problem to
   *    [[https://leetcode.com/problems/search-in-rotated-sorted-array/description/ Search in Rotated Sorted Array]],
   *    where `nums` may contain duplicates.
   *  - Would this affect the run-time complexity? How and why?
   */
  object Solution {
    def search(nums: Array[Int], target: Int): Boolean = {
      @scala.annotation.tailrec
      def bs(l: Int, r: Int): Boolean =
        if (l > r) false
        else {
          val m = l + (r - l) / 2

          if (nums(m) == target) true
          else if (nums(m) == nums(l)) bs(l + 1, r)
          else {
            val mInLeft = nums(l) <= nums(m)
            val tInLeft = nums(l) <= target

            if (mInLeft && !tInLeft) bs(m + 1, r)
            else if (!mInLeft && tInLeft) bs(l, m - 1)
            else if (nums(m) < target) bs(m + 1, r)
            else bs(l, m - 1)
          }
        }

      bs(0, nums.length - 1)
    }
  }

  object SolutionLinear {
    def search(nums: Array[Int], target: Int): Boolean = {
      @scala.annotation.tailrec
      def loop(i: Int): Boolean =
        if (i >= nums.length) false
        else if (nums(i) == target) true
        else loop(i + 1)
      loop(0)
    }
  }
  object SolutionBuiltin {
    def search(nums: Array[Int], target: Int): Boolean = nums.contains(target)
  }

  import Solution.search

  "Example 1: (nums = [2,5,6,0,0,1,2], target = 0) -> true" in {
    search(Array(2, 5, 6, 0, 0, 1, 2), 0) shouldBe true
  }
  "Example 2: (nums = [2,5,6,0,0,1,2], target = 3) -> false" in {
    search(Array(2, 5, 6, 0, 0, 1, 2), 3) shouldBe false
  }

  "Test 257: (nums = [1,3,5], target = 5) -> true" in {
    search(Array(1, 3, 5), 5) shouldBe true
  }
  "Test 266: (nums = [4,5,6,7,0,1,2], target = 0) -> true" in {
    search(Array(4, 5, 6, 7, 0, 1, 2), 0) shouldBe true
  }
  "Test 267: (nums = [3,1,1,1], target = 3) -> true" in {
    search(Array(3, 1, 1, 1), 3) shouldBe true
  }
  "Test 270: (nums = [2,2,2,0,2,2], target = 0) -> true" in {
    search(Array(2, 2, 2, 0, 2, 2), 0) shouldBe true
  }
  "Test 271: (nums = [1,2,2,2,0,1,1], target = 0) -> true" in {
    search(Array(1, 2, 2, 2, 0, 1, 1), 0) shouldBe true
  }

  "(nums = [], target = 0) -> false" in {
    search(Array(), 0) shouldBe false
  }
  "(nums = [1], target = 0) -> false" in {
    search(Array(1), 0) shouldBe false
  }
  "(nums = [1], target = 1) -> true" in {
    search(Array(1), 1) shouldBe true
  }

  "(nums = [1,1], target = 0) -> false" in {
    search(Array(1, 1), 0) shouldBe false
  }
  "(nums = [1,1], target = 1) -> true" in {
    search(Array(1, 1), 1) shouldBe true
  }

  "(nums = [2,0,1], target = 0) -> true" in {
    search(Array(2, 0, 1), 0) shouldBe true
  }
  "(nums = [2,0,1], target = 1) -> true" in {
    search(Array(2, 0, 1), 1) shouldBe true
  }
  "(nums = [2,0,1], target = 2) -> true" in {
    search(Array(2, 0, 1), 2) shouldBe true
  }
  "(nums = [2,0,1], target = 3) -> false" in {
    search(Array(2, 0, 1), 3) shouldBe false
  }
  "(nums = [2,0,1], target = -1) -> false" in {
    search(Array(2, 0, 1), -1) shouldBe false
  }

  "(nums = [3,0,1,2], target = 0) -> true" in {
    search(Array(3, 0, 1, 2), 0) shouldBe true
  }
  "(nums = [3,0,1,2], target = 1) -> true" in {
    search(Array(3, 0, 1, 2), 1) shouldBe true
  }
  "(nums = [3,0,1,2], target = 2) -> true" in {
    search(Array(3, 0, 1, 2), 2) shouldBe true
  }
  "(nums = [3,0,1,2], target = 3) -> true" in {
    search(Array(3, 0, 1, 2), 3) shouldBe true
  }
  "(nums = [3,0,1,2], target = 4) -> false" in {
    search(Array(3, 0, 1, 2), 4) shouldBe false
  }
  "(nums = [3,0,1,2], target = -1) -> false" in {
    search(Array(3, 0, 1, 2), -1) shouldBe false
  }

  "(nums = [very large arr], target = near the end of the arr) -> true" in {
    val pivot = 10_000_000
    val nums = (1 to 20_000_000).map(i => if (i <= pivot) i + pivot else i - pivot).toArray

    search(nums, pivot - 1) shouldBe true
  }
}
