package c2020_10.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d03 extends AnyWordSpec with Matchers {

  /**
   * K-diff Pairs in an Array
   *
   * Given an array of integers `nums` and an integer `k`,
   * return <em>the number of <b>unique</b> k-diff pairs in the array</em>.
   *
   * A <b>k-diff</b> pair is an integer pair `(nums[i], nums[j])`, where the following are true:<ul>
   * <li> `0 <= i, j < nums.length`
   * <li> `i != j`
   * <li> `a <= b`
   * <li> `b - a == k`
   * </ul>
   *
   * Constraints:<ul>
   * <li> `1 <= nums.length <= 104`
   * <li> `-107 <= nums[i] <= 107`
   * <li> `0 <= k <= 107`
   */
  object Solution {
    import collection.mutable
    import scala.annotation.tailrec

    def findPairs1(nums: Array[Int], k: Int): Int = {
      val results = mutable.Set[(Int, Int)]()

      @tailrec
      def loop(i: Int): Unit = {
        if (i < nums.length) {
          for (j <- (i + 1) until nums.length) {
            if (nums(i) < nums(j) && (nums(j) - nums(i) == k))
              results += nums(i) -> nums(j)
            else if (nums(i) - nums(j) == k)
              results += nums(j) -> nums(i)
          }
          loop(i + 1)
        }
      }

      loop(0)

      results.size
    }

    def findPairs(nums: Array[Int], k: Int): Int = {
      var result = 0

      if (k == 0) {
        val counts = mutable.Map[Int, Int]()
        for (i <- nums) {
          counts.update(i, counts.getOrElse(i, 0) + 1)
          if (counts(i) == 2)
            result += 1
        }
      } else {
        val dns = nums.distinct
        dns.sortInPlace()

        for {
          i <- 0 until (dns.length - 1)
          j <- (i + 1) until dns.length
        } {
          if (dns(j) - dns(i) == k)
            result += 1
        }
      }

      result
    }
  }

  "Example 1" in {
    Solution.findPairs(Array(3, 1, 4, 1, 5), 2) shouldBe 2
    // Explanation: There are two 2-diff pairs in the array, (1, 3) and (3, 5).
    //   Although we have two 1s in the input, we should only return the number of unique pairs.
  }
  "Example 2" in {
    Solution.findPairs(Array(1, 2, 3, 4, 5), 1) shouldBe 4
    // Explanation: There are four 1-diff pairs in the array, (1, 2), (2, 3), (3, 4) and (4, 5).
  }
  "Example 3" in {
    Solution.findPairs(Array(1, 3, 1, 5, 4), 0) shouldBe 1
    // Explanation: There is one 0-diff pair in the array, (1, 1).
  }
  "Example 4" in {
    Solution.findPairs(Array(1, 2, 4, 4, 3, 3, 0, 9, 2, 3), 3) shouldBe 2
  }
  "Example 5" in {
    Solution.findPairs(Array(-1, -2, -3), 1) shouldBe 2
  }

  "Test 53" in {
    Solution.findPairs(Array(1, 3, 1, 5, 4), 0) shouldBe 1
  }
}
