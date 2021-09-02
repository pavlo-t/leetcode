package challenge.c2021.c2021_09

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/september-leetcoding-challenge-2021/636/week-1-september-1st-september-7th/3960/]]
 */
class c2021_09_01 extends AnyWordSpec with Matchers {
  /**
   * === Array Nesting ===
   */
  object Solution {
    def arrayNesting(nums: Array[Int]): Int = {
      @scala.annotation.tailrec
      def longestCycle(i: Int, todo: Set[Int], seen: Set[Int], cr: Int, tr: Int): Int =
        if (todo.isEmpty) cr max tr
        else if (seen.contains(nums(i))) {
          val ni = todo.head
          longestCycle(ni, todo - ni, seen + ni, 1, tr max cr)
        } else longestCycle(nums(i), todo - nums(i), seen + i, cr + 1, tr)

      longestCycle(0, nums.toSet - 0, Set(0), 1, 0)
    }

    def arrayNestingBruteForce(nums: Array[Int]): Int = {
      @scala.annotation.tailrec
      def length(i: Int, rsf: Set[Int]): Int =
        if (rsf.contains(nums(i))) rsf.size
        else length(nums(i), rsf + nums(i))

      def startFrom(i: Int): Int =
        if (i == nums.length) 0
        else length(i, Set()) max startFrom(i + 1)

      startFrom(0)
    }
  }

  "Example 1: (nums = [5,4,0,3,1,6,2]) -> 4" in {
    Solution.arrayNesting(Array(5, 4, 0, 3, 1, 6, 2)) shouldBe 4
    //Explanation:
    //nums[0] = 5, nums[1] = 4, nums[2] = 0, nums[3] = 3, nums[4] = 1, nums[5] = 6, nums[6] = 2.
    //One of the longest sets s[k]:
    //s[0] = {nums[0], nums[5], nums[6], nums[2]} = {5, 6, 2, 0}
  }
  "Example 2: (nums = [0,1,2]) -> 1" in {
    Solution.arrayNesting(Array(0, 1, 2)) shouldBe 1
  }

  "(nums = [0]) -> 1" in {
    Solution.arrayNesting(Array(0)) shouldBe 1
  }
  "(nums = [1,2,3,4,5,6,7,0]) -> 8" in {
    Solution.arrayNesting(Array(1, 2, 3, 4, 5, 6, 7, 0)) shouldBe 8
  }
  "(nums = [7,0,1,2,3,4,5,6]) -> 8" in {
    Solution.arrayNesting(Array(7, 0, 1, 2, 3, 4, 5, 6)) shouldBe 8
  }

  "(nums = (1 to 49999, 0) -> 50000" in {
    Solution.arrayNesting((1 to 49999).toArray.appended(0)) shouldBe 50000
  }
}
