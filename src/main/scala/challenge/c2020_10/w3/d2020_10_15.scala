package challenge.c2020_10.w3

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


//noinspection DuplicatedCode
class d2020_10_15 extends AnyWordSpec with Matchers {

  /**
   * <h3>Rotate Array</h3>
   *
   * Given an array, rotate the array to the right by <em>k</em> steps, where <em>k</em> is non-negative.
   *
   * <b>Follow up:</b><ul>
   * <li> Try to come up as many solutions as you can, there are at least 3 different ways to solve this problem.
   * <li> Could you do it in-place with O(1) extra space?
   * </ul>
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= nums.length <= 20000`
   * <li> `-2 ** 31 <= nums[i] <= (2 ** 31) - 1`
   * <li> `0 <= k <= 100000`
   * </ul>
   */
  object Solution {
    import scala.annotation.tailrec

    // Using reverse
    def rotate(nums: Array[Int], k: Int): Unit = {
      val realK = k % nums.length
      if (nums.length > 1 && realK > 0) {
        @tailrec
        def reverse(start: Int, end: Int): Unit = {
          if (start < end) {
            val tmp = nums(start)
            nums(start) = nums(end)
            nums(end) = tmp
            reverse(start + 1, end - 1)
          }
        }
        reverse(0, nums.length - 1)
        reverse(0, realK - 1)
        reverse(realK, nums.length - 1)
      }
    }
  }
  object SolutionCyclicReplacements {
    def rotate(nums: Array[Int], k: Int): Unit = {
      val realK = k % nums.length
      if (nums.length > 1 && realK > 0) {
        var count = 0
        def doRotate(start: Int): Unit = {
          var i = start
          var current = nums(start - realK + nums.length)
          var next = nums(i)
          do {
            next = nums(i)
            nums(i) = current
            current = next
            i += realK
            count += 1
            if (i >= nums.length)
              i -= nums.length
          } while (i != start)
        }
        for (s <- nums.indices if count < nums.length) {
          doRotate(s)
        }
      }
    }
  }
  object SolutionConstantMemoryMultiplePasses {
    import scala.annotation.tailrec

    def rotate(nums: Array[Int], k: Int): Unit = {
      val realK = k % nums.length
      if (nums.length > 1 && realK > 0) {
        @tailrec
        def loop(steps: Int = 0): Unit =
          if (steps > 0) {
            var tmp = nums(nums.length - 1)
            for (i <- nums.indices) {
              val current = nums(i)
              nums(i) = tmp
              tmp = current
            }
            loop(steps - 1)
          }

        loop(realK)
      }
    }
  }
  object SolutionWithBuffer {
    def rotate(nums: Array[Int], k: Int): Unit = {
      val realK = k % nums.length
      if (realK > 0) {
        val buffer = collection.mutable.Queue[Int]()
        var i = 0
        var j = nums.length - realK
        while (i < nums.length) {
          buffer.enqueue(nums(i))
          if (j >= nums.length)
            nums(i) = buffer.dequeue()
          else
            nums(i) = nums(j)

          i += 1
          j += 1
        }
      }
    }
  }

  "Example 1: ([1,2,3,4,5,6,7], k = 3) -> [5,6,7,1,2,3,4]" in {
    val nums = Array(1, 2, 3, 4, 5, 6, 7)
    val expected = Array(5, 6, 7, 1, 2, 3, 4)

    Solution.rotate(nums, 3)

    nums shouldBe expected
    // Explanation:
    //   rotate 1 steps to the right: [7,1,2,3,4,5,6]
    //   rotate 2 steps to the right: [6,7,1,2,3,4,5]
    //   rotate 3 steps to the right: [5,6,7,1,2,3,4]
  }
  "Example 2: ([-1,-100,3,99], k = 2) -> [3,99,-1,-100]" in {
    val nums = Array(-1, -100, 3, 99)
    val expected = Array(3, 99, -1, -100)

    Solution.rotate(nums, 2)

    nums shouldBe expected
    // Explanation:
    //   rotate 1 steps to the right: [99,-1,-100,3]
    //   rotate 2 steps to the right: [3,99,-1,-100]
  }

  "Test 34: ([1,2,3,4,5,6], 4) -> [3,4,5,6,1,2]" in {
    val nums = Array(1, 2, 3, 4, 5, 6)
    val expected = Array(3, 4, 5, 6, 1, 2)

    Solution.rotate(nums, 4)

    nums shouldBe expected
  }

  "My test: ([0], 0) -> [0]" in {
    val nums = Array(0)
    val expected = Array(0)

    Solution.rotate(nums, 0)

    nums shouldBe expected
  }
  "My test: ([0], 100000) -> [0]" in {
    val nums = Array(0)
    val expected = Array(0)

    Solution.rotate(nums, 100000)

    nums shouldBe expected
  }
  "My test: ([1,2,3], 1) -> [3,1,2]" in {
    val nums = Array(1, 2, 3)
    val expected = Array(3, 1, 2)

    Solution.rotate(nums, 1)

    nums shouldBe expected
  }
  "My test: ([1,2,3,4,5,6,7,8], 4) -> [5,6,7,8,1,2,3,4]" in {
    val nums = Array(1, 2, 3, 4, 5, 6, 7, 8)
    val expected = Array(5, 6, 7, 8, 1, 2, 3, 4)

    Solution.rotate(nums, 4)

    nums shouldBe expected
  }
  "My test: ([1..20000], 100000) -> [1..20000]" in {
    val length = 20000
    val nums = Array.ofDim[Int](length)
    val expected = nums.clone()
    for (i <- 1 to length) {
      nums(i - 1) = i
      expected(i - 1) = i
    }

    Solution.rotate(nums, 100000)

    nums shouldBe expected
  }
  "My test: ([1..20000], 99999) -> [2..20000,1]" in {
    val length = 20000
    val nums = Array.ofDim[Int](length)
    val expected = nums.clone()
    for (i <- 1 to length) {
      nums(i - 1) = i
      expected(i - 1) = i + 1
    }
    expected(19999) = 1

    Solution.rotate(nums, 99999)

    nums shouldBe expected
  }
  "My test: ([1..20000], 1) -> [20000,1..19999]" in {
    val length = 20000
    val nums = Array.ofDim[Int](length)
    val expected = nums.clone()
    for (i <- 1 to length) {
      nums(i - 1) = i
      expected(i - 1) = i - 1
    }
    expected(0) = 20000

    Solution.rotate(nums, 1)

    nums shouldBe expected
  }
}
