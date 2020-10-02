package c2020_04.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d03 extends AnyWordSpec with Matchers {

object Solution {
  def maxSubArrayFold(nums: Array[Int]): Int = {
    var current = 0
    nums.foldLeft(Int.MinValue) { (acc, n) =>
      current = math.max(current + n, n)
      math.max(current, acc)
    }
  }

  def maxSubArray(nums: Array[Int]): Int = {
    var max = Int.MinValue
    var current = 0
    for (n <- nums) {
      current = math.max(current + n, n)
      if (current > max) max = current
    }
    max
  }
}

  "Solution" should {
    "Array(-2, 1, -3, 4, -1, 2, 1, -5, 4) shouldBe 6" in {
      Solution.maxSubArray(Array(-2, 1, -3, 4, -1, 2, 1, -5, 4)) shouldBe 6
    }

    "Array(2, -1, 2) shouldBe 3" in {
      Solution.maxSubArray(Array(2, -1, 2)) shouldBe 3
    }

    "Array(1) shouldBe 1" in {
      Solution.maxSubArray(Array(1)) shouldBe 1
    }

    "Array(0) shouldBe 0" in {
      Solution.maxSubArray(Array(0)) shouldBe 0
    }

    "Array(-1) shouldBe -1" in {
      Solution.maxSubArray(Array(-1)) shouldBe -1
    }

    "Array(-2147483647) shouldBe -2147483647" in {
      Solution.maxSubArray(Array(-2147483647)) shouldBe -2147483647
    }

    "Array(lots of elements) shouldBe Int.MaxValue" in {
      val length = 10000000
      val arr: Array[Int] = Array.ofDim[Int](length)
      arr(0) = Int.MaxValue
      for (i <- 1 until length) arr(i) = i

      Solution.maxSubArray(arr) shouldBe Int.MaxValue
    }
  }
}
