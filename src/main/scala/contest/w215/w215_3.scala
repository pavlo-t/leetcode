package contest.w215

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/weekly-contest-215/problems/minimum-operations-to-reduce-x-to-zero/]]
 */
//noinspection DuplicatedCode
class w215_3 extends AnyWordSpec with Matchers {
  /**
   * === 5602. Minimum Operations to Reduce X to Zero ===
   *
   * You are given an integer array `nums` and an integer `x`.
   * In one operation, you can either remove the leftmost or the rightmost element from the array `nums`
   * and subtract its value from `x`.
   * Note that this '''modifies''' the array for future operations.
   *
   * Return ''the '''minimum number''' of operations to reduce ''`x`'' to '''exactly''' ''`0`'' if it's possible,
   * otherwise, return ''`-1`.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100_000`
   *  - `1 <= nums[i] <= 10_000`
   *  - `1 <= x <= 1000_000_000`
   *
   */
  object Solution {
    def minOperations(nums: Array[Int], x: Int): Int = nums.sum - x match {
      case 0         => nums.length
      case targetSum =>
        @scala.annotation.tailrec
        def findLongestSubarray(l: Int, r: Int, curSum: Int, result: Int): Int =
          if (r == nums.length) result
          else {
            val nextSum = curSum + nums(r)
            if (nextSum > targetSum && l <= r)
              findLongestSubarray(l + 1, r, curSum - nums(l), result)
            else if (nextSum == targetSum)
              findLongestSubarray(l + 1, r + 1, nextSum - nums(l), result max (r - l + 1))
            else
              findLongestSubarray(l, r + 1, nextSum, result)
          }

        findLongestSubarray(0, 0, 0, 0) match {
          case 0 => -1
          case l => nums.length - l
        }
    }
  }
  object SolutionSlidingWindow {
    def minOperations(nums: Array[Int], x: Int): Int = {
      val target = nums.sum - x
      if (target == 0) nums.length
      else {
        var sum = 0
        var pre = 0
        var result = 0

        for (i <- nums.indices) {
          sum += nums(i)
          while (sum > target && pre <= i) {
            sum -= nums(pre)
            pre += 1
          }
          if (sum == target)
            result = result max (i - pre + 1)
        }

        if (result == 0) -1 else nums.length - result
      }
    }
  }
  object SolutionPrefixSumWithMap {
    def minOperations(nums: Array[Int], x: Int): Int = {
      val target = nums.sum - x
      if (target == 0) nums.length
      else {
        val map = collection.mutable.Map(0 -> -1)

        var sum = 0
        var res = Int.MinValue

        for (i <- nums.indices) {
          sum += nums(i)
          if (map.contains(sum - target))
            res = res max (i - map(sum - target))
          map(sum) = i
        }

        if (res == Int.MinValue) -1
        else nums.length - res
      }
    }
  }

  object SolutionImperativeAlmostGoodEnough {
    def minOperations(nums: Array[Int], x: Int): Int = {
      var result = Int.MaxValue

      val lSums = nums.clone()
      val rSums = nums.reverse.clone()
      for (i <- 1 until lSums.length) {
        lSums(i) += lSums(i - 1)
        rSums(i) += rSums(i - 1)
      }

      lSums.indexWhere(_ >= x) match {
        case -1                 => return -1
        case i if lSums(i) == x => result = i + 1
        case _                  =>
      }
      rSums.indexWhere(_ >= x) match {
        case -1                 => return -1
        case i if rSums(i) == x => result = result min (i + 1)
        case _                  =>
      }
      for (li <- lSums.indices if lSums(li) < x) {
        var ri = 0
        while (lSums(li) + rSums(ri) <= x) {
          if (lSums(li) + rSums(ri) == x)
            result = result min (li + ri + 2)
          ri += 1
        }
      }

      if (result == Int.MaxValue) -1 else result
    }
  }
  object Solution2 {
    def minOperations(nums: Array[Int], x: Int): Int = {
      var firstIdx = 0
      var sum = 0
      while (firstIdx < nums.length && sum < x) {
        sum += nums(firstIdx)
        firstIdx += 1
      }

      @scala.annotation.tailrec
      def sumLeftFromIndex(i: Int, sum: Int = 0, cnt: Int = 0): Int = {
        if (sum > x || cnt > nums.length) Int.MaxValue
        else if (sum == x) cnt
        else if (i == -1) sumLeftFromIndex(nums.length - 2, sum + nums.last, cnt + 1)
        else sumLeftFromIndex(i - 1, sum + nums(i), cnt + 1)
      }

      var result = if (sum == x) firstIdx + 1 else Int.MaxValue

      for (i <- (firstIdx - 1) to -1 by -1) {
        result = result min sumLeftFromIndex(i)
      }

      if (result == Int.MaxValue) -1 else result
    }
  }
  object Solution1 {
    def minOperations(nums: Array[Int], x: Int): Int = {
      var i = 0
      var sum = 0
      while (i < nums.length && sum < x) {
        sum += nums(i)
        i += 1
      }

      @scala.annotation.tailrec
      def sumLeftFromIndex(i: Int, sum: Int = 0, cnt: Int = 0): Int = {
        if (sum > x || cnt > nums.length) Int.MaxValue
        else if (sum == x) cnt
        else if (i == -1) sumLeftFromIndex(nums.length - 2, sum + nums.last, cnt + 1)
        else sumLeftFromIndex(i - 1, sum + nums(i), cnt + 1)
      }

      ((i - 1) to -1 by -1)
        .fold(if (sum == x) i + 1 else Int.MaxValue)(_ min sumLeftFromIndex(_)) match {
        case Int.MaxValue => -1
        case v            => v
      }
    }
  }

  object SolutionRecursionBruteForce {
    def minOperations(nums: Array[Int], x: Int): Int = {
      def loop(x: Int, l: Int, r: Int): Int = {
        if (x == 0) 0
        else if (x < 0) Int.MaxValue
        else if (l > r) Int.MaxValue
        else if (nums(l) == x || nums(r) == x) 1
        else if (nums(l) > x && nums(r) > x) Int.MaxValue
        else 1 + (loop(x - nums(l), l + 1, r) min loop(x - nums(r), l, r - 1))
      }

      loop(x, 0, nums.length - 1) match {
        case Int.MaxValue => -1
        case v            => v
      }
    }
  }

  import Solution.minOperations

  "Example 1: (nums = [1,1,4,2,3], x = 5) -> 2" in {
    minOperations(Array(1, 1, 4, 2, 3), 5) shouldBe 2
    //Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
  }
  "Example 2: (nums = [5,6,7,8,9], x = 4) -> -1" in {
    minOperations(Array(5, 6, 7, 8, 9), 4) shouldBe -1
  }
  "Example 3: (nums = [3,2,20,1,1,3], x = 10) -> 5" in {
    minOperations(Array(3, 2, 20, 1, 1, 3), 10) shouldBe 5
    //Explanation:
    // The optimal solution is to remove the last three elements and the first two elements
    // (5 operations in total) to reduce x to zero.
  }

  "(nums = [2,3,4,1,1], x = 5) -> 2" in {
    minOperations(Array(2, 3, 4, 1, 1), 5) shouldBe 2
  }
  "(nums = [1,1], x = 3) -> -1" in {
    minOperations(Array(1, 1), 3) shouldBe -1
  }

  "Test 86: (nums = [1 repeat 100_000], x = 99891) -> 99891" in {
    val nums = Array.fill(100_000)(1)

    minOperations(nums, 99891) shouldBe 99891
  }

  "([random repeat 100_000], x = 1_000_000_000) -> ???" in {
    val nums = Array.fill(100_000)(util.Random.nextInt(10_001) + 1)

    val result = minOperations(nums, 1_000_000_000)
    println(s"result: $result")
    result should be >= -1
  }
  "([random repeat 100_000], x = 250_000_000) -> ???" in {
    val nums = Array.fill(100_000)(util.Random.nextInt(10_001) + 1)

    val result = minOperations(nums, 250_000_000)
    println(s"result: $result")
    result should be >= -1
  }
}
