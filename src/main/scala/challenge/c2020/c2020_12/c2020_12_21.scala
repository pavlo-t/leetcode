package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/571/week-3-december-15th-december-21st/3573/]]
 */
//noinspection DuplicatedCode
class c2020_12_21 extends AnyWordSpec with Matchers {
  /**
   * === Smallest Range II ===
   *
   * Given an array `A` of integers,
   * for each integer `A[i]` we need to choose '''either''' `x += -K` '''or''' `x += K`,
   * and add `x` to `A[i]` '''(only once)'''.
   *
   * After this process, we have some array `B`.
   *
   * Return the smallest possible difference between the maximum value of `B` and the minimum value of `B`.
   *
   * '''Note:'''
   *  1. `1 <= A.length <= 10000`
   *  1. `0 <= A[i] <= 10000`
   *  1. `0 <= K <= 10000`
   */
  object Solution {
    def smallestRangeII(A: Array[Int], K: Int): Int = {
      val a = A.sorted

      val last = a.last - K
      val head = a.head + K

      @scala.annotation.tailrec
      def rec(i: Int, rsf: Int): Int = {
        if (i == a.length) rsf
        else rec(i + 1, rsf.min(last.max(a(i - 1) + K) - head.min(a(i) - K)))
      }

      rec(1, a.last - a.head)
    }
  }

  object SolutionIterative {
    def smallestRangeII(nums: Array[Int], K: Int): Int = {
      val A = nums.sorted
      var result = A.last - A.head

      for (j <- 1 until A.length; i = j - 1) {
        val max = (A.last - K).max(A(i) + K)
        val min = (A.head + K).min(A(j) - K)
        result = result.min(max - min)
      }

      result
    }
  }

  object SolutionMutateSortInput {
    def smallestRangeII(A: Array[Int], K: Int): Int = {
      A.sortInPlace()
      var result = A.last - A.head

      for (j <- 1 until A.length; i = j - 1) {
        val max = (A.last - K).max(A(i) + K)
        val min = (A.head + K).min(A(j) - K)
        result = result.min(max - min)
      }

      result
    }
  }

  import Solution.smallestRangeII

  "Example 1: (A = [1], K = 0) -> 0" in {
    smallestRangeII(Array(1), 0) shouldBe 0
    //Explanation: B = [1]
  }
  "Example 2: (A = [0,10], K = 2) -> 6" in {
    smallestRangeII(Array(0, 10), 2) shouldBe 6
    //Explanation: B = [2,8]
  }
  "Example 3: (A = [1,3,6], K = 3) -> 3" in {
    smallestRangeII(Array(1, 3, 6), 3) shouldBe 3
    //Explanation: B = [4,6,3]
  }

  "(A = [1,2], K = 3) -> 1" in (smallestRangeII(Array(1, 2), 3) shouldBe 1)
  "(A = [1,2], K = 2) -> 1" in (smallestRangeII(Array(1, 2), 2) shouldBe 1)
  "(A = [1,2], K = 1) -> 1" in (smallestRangeII(Array(1, 2), 1) shouldBe 1)

  "(A = [1,3], K = 1) -> 0" in (smallestRangeII(Array(1, 3), 1) shouldBe 0)
  "(A = [1,3], K = 2) -> 2" in (smallestRangeII(Array(1, 3), 2) shouldBe 2)
  "(A = [1,3], K = 3) -> 2" in (smallestRangeII(Array(1, 3), 3) shouldBe 2)

  "(A = [1,4], K = 1) -> 1" in (smallestRangeII(Array(1, 4), 1) shouldBe 1)
  "(A = [1,4], K = 2) -> 1" in (smallestRangeII(Array(1, 4), 2) shouldBe 1)
  "(A = [1,4], K = 3) -> 3" in (smallestRangeII(Array(1, 4), 3) shouldBe 3)

  "(A = [1,5], K = 1) -> 2" in (smallestRangeII(Array(1, 5), 1) shouldBe 2)
  "(A = [1,5], K = 2) -> 0" in (smallestRangeII(Array(1, 5), 2) shouldBe 0)
  "(A = [1,5], K = 3) -> 2" in (smallestRangeII(Array(1, 5), 3) shouldBe 2)
  "(A = [1,5], K = 4) -> 4" in (smallestRangeII(Array(1, 5), 4) shouldBe 4)

  "(A = [1,6], K = 1) -> 3" in (smallestRangeII(Array(1, 6), 1) shouldBe 3 /*[2,5]*/)
  "(A = [1,6], K = 2) -> 1" in (smallestRangeII(Array(1, 6), 2) shouldBe 1 /*[3,4]*/)
  "(A = [1,6], K = 3) -> 1" in (smallestRangeII(Array(1, 6), 3) shouldBe 1 /*[4,3]*/)
  "(A = [1,6], K = 4) -> 3" in (smallestRangeII(Array(1, 6), 4) shouldBe 3 /*[5,2]*/)
  "(A = [1,6], K = 5) -> 5" in (smallestRangeII(Array(1, 6), 5) shouldBe 5 /*[6,11]*/)

  "(A = [1,2,6], K = 4) -> 4" in (smallestRangeII(Array(1, 2, 6), 4) shouldBe 4 /*[5,6,2]*/)
  "(A = [1,3,6], K = 4) -> 5" in (smallestRangeII(Array(1, 3, 6), 4) shouldBe 5 /*[5,7,10|2]*/)
  "(A = [1,4,6], K = 4) -> 5" in (smallestRangeII(Array(1, 4, 6), 4) shouldBe 5 /*[5,8,10]*/)
  "(A = [1,5,6], K = 4) -> 5" in (smallestRangeII(Array(1, 5, 6), 4) shouldBe 4 /*[5,1,2]*/)

  "(A = [1,2,3,4,5], K = 4) -> 4" in (smallestRangeII(Array(1, 2, 3, 4, 5), 4) shouldBe 4 /*[5,6,7,8,9]*/)

  "(A = [1,2,3,4,5,6], K = 4) -> 5" in (smallestRangeII(Array(1, 2, 3, 4, 5, 6), 4) shouldBe 5 /*[5,6,7,8,9,10]*/)
  "(A = [1,2,3,4,5,6], K = 3) -> 5" in (smallestRangeII(Array(1, 2, 3, 4, 5, 6), 3) shouldBe 5 /*[4,5,6,7,8,9]*/)
  "(A = [1,2,3,4,5,6], K = 2) -> 3" in (smallestRangeII(Array(1, 2, 3, 4, 5, 6), 2) shouldBe 3 /*[3,4,5,6,3,4]*/)
  "(A = [1,2,3,4,5,6], K = 1) -> 3" in (smallestRangeII(Array(1, 2, 3, 4, 5, 6), 1) shouldBe 3 /*[2,3,4,5,4,5]*/)
}
