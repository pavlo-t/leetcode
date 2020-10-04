package c2020_04.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


// https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/528/week-1/3286/
class d04 extends AnyWordSpec with Matchers {

  /**
   * Move Zeroes
   *
   * Given an array `nums`, write a function to move all `0`'s to the end of it
   * while maintaining the relative order of the non-zero elements.
   *
   * Note:<ol>
   * <li> You must do this <b>in-place</b> without making a copy of the array.
   * <li> Minimize the total number of operations.
   */
  object Solution {
    def moveZeroes1(nums: Array[Int]): Unit =
      nums.sortInPlaceWith((_, r) => r == 0)

    import scala.annotation.tailrec

    def moveZeroes2(nums: Array[Int]): Unit = {
      @tailrec
      def nextNonZero(i: Int): Int = {
        if (i < nums.length) {
          if (nums(i) != 0) i
          else nextNonZero(i + 1)
        }
        else -1
      }

      @tailrec
      def loop(i: Int): Unit =
        if (i < nums.length - 1) {
          if (nums(i) == 0) {
            val j = nextNonZero(i + 1)
            if (j > 0) {
              val t = nums(i)
              nums(i) = nums(j)
              nums(j) = t
            }
          }
          loop(i + 1)
        }

      loop(0)
    }

    def moveZeroes(nums: Array[Int]): Unit = {
      var lastNonZeroFoundAt = 0
      var cur = 0
      var t = 0

      while (cur < nums.length) {
        if (nums(cur) != 0) {
          t = nums(cur)
          nums(cur) = nums(lastNonZeroFoundAt)
          nums(lastNonZeroFoundAt) = t

          lastNonZeroFoundAt += 1
        }
        cur += 1
      }
    }
  }

  "Example 1" in {
    val arr = Array(0, 1, 0, 3, 12)
    Solution.moveZeroes(arr)

    arr shouldBe Array(1, 3, 12, 0, 0)
  }

  "Test 1" in {
    val arr = Array(0, 0, 0, 0, 42)
    Solution.moveZeroes(arr)

    arr shouldBe Array(42, 0, 0, 0, 0)
  }

  "Test 10" in {
    val arr = Array(1)
    Solution.moveZeroes(arr)

    arr shouldBe Array(1)
  }
}
