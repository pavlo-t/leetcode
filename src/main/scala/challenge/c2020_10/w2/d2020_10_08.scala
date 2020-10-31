package challenge.c2020_10.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d2020_10_08 extends AnyWordSpec with Matchers {

  /**
   * Binary Search
   *
   * Given a <b>sorted</b> (in ascending order) integer array `nums` of `n` elements and a target value,
   * write a function to search `target` in `nums`.
   * If `target` exists, then return its index, otherwise return `-1`.
   *
   *
   * <b>Constraints:</b><ol>
   * <li> You may assume that all elements in `nums` are unique.
   * <li> `n` will be in the range `[1, 10000]`.
   * <li> The value of each element in `nums` will be in the range `[-9999, 9999]`.
   */
  object Solution {
    import scala.annotation.tailrec

    def search1(nums: Array[Int], target: Int): Int = {
      if (nums.length == 1) if (nums(0) == target) 0 else -1
      else {
        @tailrec
        def bs(i: Int, f: Int, t: Int): Int = {
          if (nums(i) == target) i
          else if (f == t) -1
          else if (nums(i) < target) {
            val nf = i + 1
            bs(nf + (t - nf) / 2, nf, t)
          } else {
            val nt = i - 1
            bs(f + (nt - f) / 2, f, nt)
          }
        }

        bs(nums.length / 2, 0, nums.length - 1)
      }
    }

    def search(nums: Array[Int], target: Int): Int = {
      @tailrec
      def bs(i: Int, f: Int, t: Int): Int = {
        if (nums(i) == target) i
        else if (f == t) -1
        else if (nums(i) < target) {
          val nf = i + 1
          bs(nf + (t - nf) / 2, nf, t)
        } else {
          val nt = i - 1
          bs(f + (nt - f) / 2, f, nt)
        }
      }

      bs(nums.length / 2, 0, nums.length - 1)
    }
  }

  "Example 1" in {
    Solution.search(Array(-1, 0, 3, 5, 9, 12), 9) shouldBe 4
    // Explanation: 9 exists in nums and its index is 4
  }
  "Example 2" in {
    Solution.search(Array(-1, 0, 3, 5, 9, 12), 2) shouldBe -1
    // Explanation: 2 does not exist in nums so return -1
  }

  "My test: 1 el array, target exists" in {
    Solution.search(Array(0), 0) shouldBe 0
  }
  "My test: 1 el array, target absent" in {
    Solution.search(Array(0), 1) shouldBe -1
  }
  "My test: 3 el array, 0" in {
    Solution.search(Array(0, 1, 2), 0) shouldBe 0
  }
  "My test: 3 el array, 1" in {
    Solution.search(Array(0, 1, 2), 1) shouldBe 1
  }
  "My test: 3 el array, 2" in {
    Solution.search(Array(0, 1, 2), 2) shouldBe 2
  }
  "My test: 3 el array, -1" in {
    Solution.search(Array(0, 1, 2), -1) shouldBe -1
  }
  "My test: max size array" in {
    import util.Random

    val length = 10000
    val nums = Array.ofDim[Int](length)
    var current = -9999
    for (i <- 0 until length) {
      nums(i) = current
      current += Random.nextInt(2) + 1
    }
    val target = Random.nextInt(19999) - 9999

    val result = Solution.search(nums, target)

    result shouldBe nums.indexOf(target)
  }
}
