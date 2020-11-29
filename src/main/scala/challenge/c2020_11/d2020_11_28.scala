package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

import scala.io.Source

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/567/week-4-november-22nd-november-28th/3546/]]
 */
//noinspection DuplicatedCode
class d2020_11_28 extends AnyWordSpec with Matchers {

  /**
   * == Sliding Window Maximum ==
   *
   * You are given an array of integers `nums`,
   * there is a sliding window of size `k` which is moving from the very left of the array to the very right.
   * You can only see the `k` numbers in the window.
   * Each time the sliding window moves right by one position.
   *
   * Return ''the max sliding window''.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 100_000`
   *  - `-10_000 <= nums[i] <= 10_000`
   *  - `1 <= k <= nums.length`
   */
  object Solution {
    def maxSlidingWindow(nums: Array[Int], k: Int): Array[Int] = {
      val w = collection.mutable.ArrayDeque[Int]()
      val result = Array.ofDim[Int](nums.length - k + 1)

      for (i <- 0 until k) {
        w.removeLastWhile(nums(_) <= nums(i))
        w.addOne(i)
      }
      result(0) = nums(w.head)

      for (i <- k until nums.length) {
        if (w.head <= i - k) w.removeHead(true)
        w.removeLastWhile(nums(_) <= nums(i))
        w.addOne(i)
        result(i - k + 1) = nums(w.head)
      }

      result
    }
  }

  object SolutionMyBad {
    def maxSlidingWindow(nums: Array[Int], k: Int): Array[Int] =
      if (k == 1) nums
      else {
        val result = Array.ofDim[Int](nums.length - k + 1)

        val q = collection.mutable.ArrayDeque[Int]()
        q.addAll(nums.take(k))
        var max = q.max

        result(0) = max

        for (i <- k until nums.length) {
          val oldHead = q.removeHead()
          q.addOne(nums(i))
          if (oldHead == max) max = q.max
          else max = max max nums(i)

          result(i - k + 1) = max
        }

        result
      }
  }

  object SolutionIterative {
    def maxSlidingWindow(nums: Array[Int], k: Int): Array[Int] =
      if (k == 1) nums
      else {
        val result = Array.ofDim[Int](nums.length - k + 1)

        for (l <- 0 to nums.length - k; r = l + k - 1) {
          result(l) = (l to r).fold(-10000)((rsf, i) => rsf max nums(i))
        }

        result
      }
  }
  object SolutionBuiltins {
    def maxSlidingWindow(nums: Array[Int], k: Int): Array[Int] =
      nums.sliding(k).map(_.max).toArray
  }

  import Solution.maxSlidingWindow

  "Example 1: (nums = [1,3,-1,-3,5,3,6,7], k = 3) -> [3,3,5,5,6,7]" in {
    val nums = Array(1, 3, -1, -3, 5, 3, 6, 7)
    val expected = Array(3, 3, 5, 5, 6, 7)
    maxSlidingWindow(nums, 3) shouldBe expected
    //Explanation:
    //Window position                Max
    //---------------               -----
    //[1  3  -1] -3  5  3  6  7       3
    // 1 [3  -1  -3] 5  3  6  7       3
    // 1  3 [-1  -3  5] 3  6  7       5
    // 1  3  -1 [-3  5  3] 6  7       5
    // 1  3  -1  -3 [5  3  6] 7       6
    // 1  3  -1  -3  5 [3  6  7]      7
  }
  "Example 2: (nums = [1], k = 1) -> [1]" in {
    maxSlidingWindow(Array(1), 1) shouldBe Array(1)
  }
  "Example 3: (nums = [1,-1], k = 1) -> [1,-1]" in {
    maxSlidingWindow(Array(1, -1), 1) shouldBe Array(1, -1)
  }
  "Example 4: (nums = [9,11], k = 2) -> [11]" in {
    maxSlidingWindow(Array(9, 11), 2) shouldBe Array(11)
  }
  "Example 5: (nums = [4,-2], k = 2) -> [4]" in {
    maxSlidingWindow(Array(4, -2), 2) shouldBe Array(4)
  }

  "Test 30: ([1,3,1,2,0,5], 3) -> [3,3,2,5]" in {
    maxSlidingWindow(Array(1, 3, 1, 2, 0, 5), 3) shouldBe Array(3, 3, 2, 5)
  }

  "(nums = [1,2...100_000], k = 2) -> [2,3...100_000]" in {
    val nums = (1 to 100000).toArray
    val expected = (2 to 100000).toArray
    maxSlidingWindow(nums, 2) shouldBe expected
  }
  "(nums = [1,2...100_000], k = 50000) -> [50000...100_000]" in {
    val nums = (1 to 100000).toArray
    val expected = (50000 to 100000).toArray
    maxSlidingWindow(nums, 50000) shouldBe expected
  }
  "(nums = 100k elements [10_000,10_000...1,1], k = 50000) -> [10_000,10_000...5001,5001,5000]" in {
    val nums = (10000 to 1 by -1).flatMap(Array.fill(10)(_)).toArray
    val expected = (10000 to 5000 by -1).flatMap(Array.fill(10)(_)).toArray.dropRight(9)

    maxSlidingWindow(nums, 50000) shouldBe expected
  }

  "Test 50: (100000 elements, 50000) -> 50001 elements" in {
    val numsStr +: kStr +: Seq() = Source.fromResource("challenge/c2020_11/d2020_11_28.txt").getLines().toSeq
    val nums = numsStr.split(',').map(_.toInt)
    val k = kStr.toInt

    maxSlidingWindow(nums, k).length shouldBe 50001
  }
}
