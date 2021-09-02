package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3603/]]
 */
//noinspection DuplicatedCode
class c2021_01_14 extends AnyWordSpec with Matchers {
  /**
   * === Minimum Operations to Reduce X to Zero ===
   *
   * You are given an integer array `nums` and an integer `x`.
   * In one operation, you can either remove the leftmost or the rightmost element from the array `nums`
   * and subtract its value from `x`.
   * Note that this '''modifies''' the array for future operations.
   *
   * Return ''the minimum number of operations to reduce'' `x` ''to '''exactly''''' `0` ''if it's possible,
   * otherwise, return'' `-1`.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100_000`
   *  - `1 <= nums[i] <= 10_000`
   *  - `1 <= x <= 1_000_000_000`
   */
  object Solution {
    def minOperations(nums: Array[Int], x: Int): Int = {
      val target = nums.sum - x
      if (target < 0) -1
      else if (target == 0) nums.length
      else {
        @scala.annotation.tailrec
        def longestSubarray(l: Int, r: Int, subarraySum: Int, maxLength: Int): Int = {
          if (r == nums.length) maxLength
          else {
            val ns = subarraySum + nums(r)
            if (ns > target)
              longestSubarray(l + 1, r, subarraySum - nums(l), maxLength)
            else if (ns == target)
              longestSubarray(l + 1, r + 1, ns - nums(l), maxLength max (r - l + 1))
            else
              longestSubarray(l, r + 1, ns, maxLength)
          }
        }

        longestSubarray(0, 0, 0, 0) match {
          case 0      => -1
          case length => nums.length - length
        }
      }
    }
  }

  object SolutionDfsWithMemo {
    def minOperations(nums: Array[Int], x: Int): Int = {
      val memo = Array.fill(nums.length, nums.length)(Int.MinValue)
      def dfs(l: Int, r: Int, sum: Int): Int = {
        if (sum == x) l + nums.length - 1 - r
        else if (memo(l)(r) > Int.MinValue) memo(l)(r)
        else if (l > r || sum > x) Int.MaxValue
        else {
          val result = dfs(l + 1, r, sum + nums(l)) min dfs(l, r - 1, sum + nums(r))
          memo(l)(r) = result
          result
        }
      }

      dfs(0, nums.length - 1, 0) match {
        case Int.MaxValue => -1
        case v            => v
      }
    }
  }
  object SolutionDfsBruteForce {
    def minOperations(nums: Array[Int], x: Int): Int = {
      def dfs(l: Int, r: Int, sum: Int): Int = {
        if (sum == x) l + nums.length - 1 - r
        else if (l > r || sum > x) Int.MaxValue
        else dfs(l + 1, r, sum + nums(l)) min dfs(l, r - 1, sum + nums(r))
      }

      dfs(0, nums.length - 1, 0) match {
        case Int.MaxValue => -1
        case v            => v
      }
    }
  }
  object SolutionBfsBruteForce {
    def minOperations(nums: Array[Int], x: Int): Int = {
      @scala.annotation.tailrec
      def bfs(todo: Seq[(Int, Int, Int)]): Int = {
        todo match {
          case Nil                                 => -1
          case (l, r, s) :: _ if s == x            => l + nums.length - 1 - r
          case (l, r, s) :: rest if l > r || s > x => bfs(rest)
          case (l, r, s) :: rest                   =>
            bfs(rest ++ Seq((l + 1, r, s + nums(l)), (l, r - 1, s + nums(r))))
        }
      }

      bfs(Seq((0, nums.length - 1, 0)))
    }
  }

  import Solution.minOperations

  "Example 1: (nums = [1,1,4,2,3], x = 5) -> 2" in {
    minOperations(Array(1, 1, 4, 2, 3), 5) shouldBe 2
    //Explanation: The optimal solution is to remove the last two elements to reduce x to zero.
  }
  "Example 2: (nums = [5,6,7,8,9], x = 4) -> -1" in {
    minOperations(Array(5, 6, 7, 8, 9), 1) shouldBe -1
  }
  "Example 3: (nums = [3,2,20,1,1,3], x = 10) -> 5" in {
    minOperations(Array(3, 2, 20, 1, 1, 3), 10) shouldBe 5
    //Explanation:
    // The optimal solution is to remove the last three elements and
    // the first two elements (5 operations in total) to reduce x to zero.
  }

  "(nums = [1,1,1,1,1], x = 5) -> 5" in {
    minOperations(Array(1, 1, 1, 1, 1), 5) shouldBe 5
  }
  "(nums = [1,1,1,1], x = 4) -> 4" in {
    minOperations(Array(1, 1, 1, 1), 4) shouldBe 4
  }

  "(nums = 1to1000, x = 500500) -> 1000" in {
    minOperations((1 to 1000).toArray, 500500) shouldBe 1000
  }
  "(nums = 1to10000, x = 50005000) -> 10000" in {
    minOperations((1 to 10000).toArray, 50005000) shouldBe 10000
  }
  "(nums = 1to10000, x = 50004999) -> 9999" in {
    minOperations((1 to 10000).toArray, 50004999) shouldBe 9999
  }
  "(nums = 10000to1, x = 50004999) -> 9999" in {
    minOperations((10000 to 1 by -1).toArray, 50004999) shouldBe 9999
  }
  "(nums = 1to100000, x = 705082704) -> 100000" in {
    minOperations((1 to 100000).toArray, 705082704) shouldBe 100000
  }
  "(nums = 1to100000, x = 705082703) -> 99999" in {
    minOperations((1 to 100000).toArray, 705082703) shouldBe 99999
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

}
