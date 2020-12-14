package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/570/week-2-december-8th-december-14th/3564/]]
 */
//noinspection DuplicatedCode
class c2020_12_13 extends AnyWordSpec with Matchers {

  /**
   * === Burst Balloons ===
   *
   * Given `n` balloons, indexed from `0` to `n-1`.
   * Each balloon is painted with a number on it represented by array `nums`.
   * You are asked to burst all the balloons.
   * If the you burst balloon `i` you will get `nums[left] * nums[i] * nums[right]` coins.
   * Here `left` and `right` are adjacent indices of `i`.
   * After the burst, the `left` and `right` then becomes adjacent.
   *
   * Find the maximum coins you can collect by bursting the balloons wisely.
   *
   * '''Note:'''
   *  - You may imagine `nums[-1] = nums[n] = 1`. They are not real therefore you can not burst them.
   *  - `0 ≤ n ≤ 500`
   *  - `0 ≤ nums[i] ≤ 100`
   */
  object Solution {
    /** [[https://www.geeksforgeeks.org/burst-balloon-to-maximize-coins/]] */
    def maxCoins(nums: Array[Int]): Int = {
      val ns = Array(1) ++ nums ++ Array(1)
      val dp = Array.fill(ns.length, ns.length)(0)

      for {
        length <- 1 to nums.length
        left <- 1 to (nums.length - length + 1)
        right = left + length - 1
        last <- left to right
      } {
        val m = dp(left)(last - 1) + ns(left - 1) * ns(last) * ns(right + 1) + dp(last + 1)(right)
        dp(left)(right) = dp(left)(right) max m
      }

      dp(1)(nums.length)
    }
  }

  object SolutionDpWithLogging {
    def maxCoins(nums: Array[Int]): Int = {
      Thread.sleep(30)
      if (nums.length > 30) println(s"maxCoins(${nums.take(30).mkString("[", ",", ",...]")}")
      else println(s"maxCoins(${nums.mkString("[", ",", "]")}")

      val ns = Array(1) ++ nums ++ Array(1)
      val dp = Array.fill(ns.length, ns.length)(0)

      for {
        length <- 1 to nums.length
        left <- 1 to (nums.length - length + 1)
        right = left + length - 1
        last <- left to right
      } {
        val m = dp(left)(last - 1) + ns(left - 1) * ns(last) * ns(right + 1) + dp(last + 1)(right)
        dp(left)(right) = dp(left)(right) max m

        Thread.sleep(20)
        println(s"  loop(len=$length,l=$left,r=$right,last=$last;dp($left)($right)=${dp(left)(right)}")
      }

      dp(1)(nums.length)
    }
  }

  object SolutionDp2 {
    /**
     * @see [[https://www.youtube.com/watch?v=IFNibRVgFBo]]
     * @see [[https://www.youtube.com/watch?v=KWPat-qNAGI]]
     */
    def maxCoins(nums: Array[Int]): Int = if (nums.isEmpty) 0 else {
      val T = Array.ofDim[Int](nums.length, nums.length)

      for {
        len <- 1 to nums.length
        i <- 0 to (nums.length - len)
        j = i + len - 1
        k <- i to j
      } {
        val leftValue = if (i == 0) 1 else nums(i - 1)
        val rightValue = if (j == nums.length - 1) 1 else nums(j + 1)

        val before = if (i == k) 0 else T(i)(k - 1)
        val after = if (j == k) 0 else T(k + 1)(j)

        T(i)(j) = T(i)(j) max (leftValue * nums(k) * rightValue + before + after)
      }

      T(0)(nums.length - 1)
    }
  }


  object SolutionMemoization {
    def maxCoins(nums: Array[Int]): Int = {

      val cache = collection.mutable.Map[Seq[Int], Int]()

      def loop(nums: Seq[Int]): Int = {
        if (nums.isEmpty) 0
        else if (cache.contains(nums)) cache(nums)
        else if (nums.length == 1) {
          cache(nums) = nums.head
          cache(nums)
        } else if (nums.length == 2) {
          cache(nums) = nums.min * nums.max + nums.max
          cache(nums)
        } else {
          cache(nums) = (for (i <- nums.indices) yield {
            if (i == 0) nums.head * nums(1) + loop(nums.drop(1))
            else if (i == nums.length - 1) nums(i - 1) * nums(i) + loop(nums.dropRight(1))
            else nums(i - 1) * nums(i) * nums(i + 1) + loop(nums.patch(i, Nil, 1))
          }).max
          cache(nums)
        }
      }

      loop(nums)
    }
  }

  object SolutionBruteForce {
    def maxCoins(nums: Array[Int]): Int = {
      if (nums.isEmpty) 0
      else if (nums.length == 1) nums.head
      else if (nums.length == 2) nums.min * nums.max + nums.max
      else {
        (for (i <- nums.indices) yield {
          if (i == 0) nums.head * nums(1) + maxCoins(nums.drop(1))
          else if (i == nums.length - 1) nums(i - 1) * nums(i) + maxCoins(nums.dropRight(1))
          else nums(i - 1) * nums(i) * nums(i + 1) + maxCoins(nums.patch(i, Nil, 1))
        }).max
      }
    }
  }

  import Solution.maxCoins

  "Example: ([3,1,5,8]) -> 167" in {
    maxCoins(Array(3, 1, 5, 8)) shouldBe 167
    //Output: 167
    //Explanation: nums = [3,1,5,8] --> [3,5,8] -->   [3,8]   -->  [8]  --> []
    //             coins =  3*1*5      +  3*5*8    +  1*3*8      + 1*8*1   = 167
  }

  "([]) -> 0" in {
    maxCoins(Array()) shouldBe 0
  }

  "([1 to 5]) -> 110" in {
    maxCoins((1 to 5).toArray) shouldBe 110
  }
  "([1 to 10]) -> 2420" in {
    maxCoins((1 to 10).toArray) shouldBe 2420
  }
  "([10 to 1]) -> 2420" in {
    maxCoins((10 to 1 by -1).toArray) shouldBe 2420
  }

  "([1 to 500]) -> 2,147,483,640" in {
    maxCoins((1 to 500).toArray) shouldBe 2_147_483_640
  }
}
