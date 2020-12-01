package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/568/week-5-november-29th-november-30th/3547/]]
 */
//noinspection DuplicatedCode
class c2020_11_w5 extends AnyWordSpec with Matchers {

  /**
   * === Maximum Average Subarray II ===
   *
   * You are given an integer array `nums` consisting of `n` elements, and an integer `k`.
   *
   * Find a contiguous subarray whose '''length is greater than or equal''' to `k`
   * that has the maximum average value and return ''this value''.
   * Any answer with a calculation error less than `0.00001` will be accepted.
   *
   * '''Constraints:'''
   *  - `1 <= k <= nums.length <= 10_000`
   *  - `-10_000 <= nums[i] <= 10_000`
   */
  object Solution {
    def findMaxAverage(nums: Array[Int], k: Int): Double = {
      val ErrorTolerance = 0.00001

      def canGetHigherAvg(avg: Double): Boolean = {
        @scala.annotation.tailrec
        def loop(i: Int, rSum: Double, lSum: Double, minLSum: Double): Boolean = {
          if (i < k) loop(i + 1, rSum + nums(i) - avg, lSum, minLSum)
          else if (rSum >= minLSum) true
          else if (i == nums.length) false
          else {
            val nLSum = lSum + nums(i - k) - avg
            loop(i + 1, rSum + nums(i) - avg, nLSum, minLSum min nLSum)
          }
        }
        loop(0, 0, 0, 0)
      }

      @scala.annotation.tailrec
      def bs(l: Double, r: Double): Double = {
        if (r - l < ErrorTolerance) l
        else {
          val mid = (l + r) / 2
          if (canGetHigherAvg(mid)) bs(mid, r)
          else bs(l, mid)
        }
      }

      val (min, max) = nums.foldLeft((Int.MaxValue, Int.MinValue))((rsf, i) => (rsf._1 min i, rsf._2 max i))
      bs(min, max)
    }
  }

  // https://leetcode.com/problems/maximum-average-subarray-ii/solution/
  object SolutionBinarySearch {
    def findMaxAverage(nums: Array[Int], k: Int): Double = {
      var max_val: Double = Integer.MIN_VALUE
      var min_val: Double = Integer.MAX_VALUE

      for (n <- nums) {
        max_val = Math.max(max_val, n)
        min_val = Math.min(min_val, n)
      }

      var prev_mid = max_val
      var error: Double = Integer.MAX_VALUE
      while (error > 0.00001) {
        val mid = (max_val + min_val) * 0.5
        if (check(nums, mid, k)) min_val = mid
        else max_val = mid
        error = Math.abs(prev_mid - mid)
        prev_mid = mid
      }

      println(s"min=$min_val,max=$max_val,prev_mid=$prev_mid,error=$error")

      min_val
    }

    def check(nums: Array[Int], mid: Double, k: Int): Boolean = {
      var sum = 0.0
      var prev = 0.0
      var min_sum = 0.0
      for (i <- 0 until k) {
        sum += nums(i) - mid
      }
      if (sum >= 0) return true
      for (i <- k until nums.length) {
        sum += nums(i) - mid
        prev += nums(i - k) - mid
        min_sum = Math.min(prev, min_sum)
        if (sum >= min_sum) return true
      }
      false
    }
  }
  object SolutionIterative {
    def findMaxAverage(nums: Array[Int], k: Int): Double = {
      var result = Double.MinValue

      for (l <- 0 to (nums.length - k)) {
        var sum = 0
        for (r <- l until nums.length) {
          sum += nums(r)
          if (r - l + 1 >= k)
            result = result max (sum.toDouble / (r - l + 1))
        }
      }

      result
    }
  }

  object SolutionMyDp {
    def findMaxAverage(nums: Array[Int], k: Int): Double = {
      val dp = nums.clone()
      for (i <- 1 until dp.length) dp(i) += dp(i - 1)

      var result = Double.MinValue
      for (l <- k to nums.length) {
        var sum = Int.MinValue
        for (i <- (l - 1) until nums.length) {
          sum = sum max (if (i < l) dp(i) else dp(i) - dp(i - l))
        }
        result = result max (sum.toDouble / l)
      }
      result
    }
  }

  object SolutionMyScalaBuiltins {
    def findMaxAverage(nums: Array[Int], k: Int): Double = {
      Thread.sleep(20)
      if (nums.length <= 30) println(s"(${nums.mkString("[", ",", "]")},k=$k)")
      else println(s"(${nums.take(12).mkString("[", ",", ",..")}${nums.takeRight(12).mkString(",", ",", "]")},k=$k)")

      (k to nums.length).foldLeft(Double.MinValue) { (rsf, k) =>
        if (k % 100 == 0) println(s" k=$k,rsf=$rsf")
        rsf max (nums.sliding(k).foldLeft(0.0)(_ max _.sum) / k)
      }
    }
  }
  object SolutionMyBruteForceRecursion {
    def findMaxAverage(nums: Array[Int], k: Int): Double = {
      Thread.sleep(20)
      if (nums.length <= 30) println(s"(${nums.mkString("[", ",", "]")},k=$k)")
      else println(s"(${nums.take(12).mkString("[", ",", ",..")}${nums.takeRight(12).mkString(",", ",", "]")},k=$k)")

      if (k == 1) return nums.max
      def loop(i: Int, start: Int): Double = {

        println(s" loop($i, $start)")
        if (i >= nums.length) {
          Double.MinValue
        } else if (start == -1)
          loop(i + 1, i) max loop(i + 1, start)
        else if (i - start + 1 >= k) {
          var sum = 0.0
          for (j <- start to i) sum += nums(j)
          val avg = sum / (i - start + 1)

          println(s"  sum=$sum,avg=$avg")
          avg max loop(i + 1, start)
        } else
          loop(i + 1, start)
      }

      loop(0, -1)
    }
  }

  import Solution.findMaxAverage

  "Example 1: (nums = [1,12,-5,-6,50,3], k = 4) -> 12.75000" in {
    findMaxAverage(Array(1, 12, -5, -6, 50, 3), 4) shouldBe 12.75 +- 0.00001
    //Explanation:
    // - When the length is 4, averages are [0.5, 12.75, 10.5] and the maximum average is 12.75
    // - When the length is 5, averages are [10.4, 10.8] and the maximum average is 10.8
    // - When the length is 6, averages are [9.16667] and the maximum average is 9.16667
    // The maximum average is when we choose a subarray of length 4 (i.e., the sub array [12, -5, -6, 50])
    // which has the max average 12.75, so we return 12.75
    // Note that we do not consider the subarrays of length < 4.
  }
  "Example 2: (nums = [5], k = 1) -> 5.00000" in {
    findMaxAverage(Array(5), 1) shouldBe 5.0 +- 0.00001
  }

  "(nums = [50,12,-5,-6,50,3], k = 4) -> 20.2" in {
    findMaxAverage(Array(50, 12, -5, -6, 50, 3), 4) shouldBe 20.2 +- 0.00001
  }
  "(nums = [1,20,-5,-6,50,20], k = 4) -> 15.8" in {
    findMaxAverage(Array(1, 20, -5, -6, 50, 20), 4) shouldBe 15.8 +- 0.00001
  }
  "([1..=10000], 1) -> 10000.0" in {
    findMaxAverage((1 to 10000).toArray, 1) shouldBe 10000.0 +- 0.00001
  }
  "([-10000..=10000], 1) -> 10000.0" in {
    findMaxAverage((-10000 to 10000).toArray, 1) shouldBe 10000.0 +- 0.00001
  }

  "([1..=10000], 2) -> 9999.5" in {
    findMaxAverage((1 to 10000).toArray, 2) shouldBe 9999.5 +- 0.00001
  }
  "([1;10000], 2) -> 1.0" in {
    findMaxAverage(Array.fill(10000)(1), 2) shouldBe 1.0 +- 0.00001
  }
}
