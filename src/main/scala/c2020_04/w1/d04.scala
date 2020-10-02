package c2020_04.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


// https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/528/week-1/3286/
class d04 extends AnyWordSpec with Matchers {

  object Solution {
    def moveZeroes(nums: Array[Int]): Unit = {
      nums.sortInPlaceWith((_, r) => r == 0)
    }
  }

  "Solution" should {
    "Array(0, 1, 0, 3, 12) shouldBe Array(1, 3, 12, 0, 0)" in {
      val arr = Array(0, 1, 0, 3, 12)
      Solution.moveZeroes(arr)

      arr shouldBe Array(1, 3, 12, 0, 0)
    }
  }
}
