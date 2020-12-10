package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/570/week-2-december-8th-december-14th/3558/]]
 */
//noinspection DuplicatedCode
class c2020_12_w2 extends AnyWordSpec with Matchers {

  /**
   * === Missing Ranges ===
   *
   * You are given an inclusive range `[lower, upper]` and a '''sorted unique''' integer array `nums`,
   * where all elements are in the inclusive range.
   *
   * A number `x` is considered missing if `x` is in the range `[lower, upper]` and `x` is not in `nums`.
   *
   * Return ''the '''smallest sorted''' list of ranges that '''cover every missing number exactly'''''.
   * That is, no element of `nums` is in any of the ranges, and each missing number is in one of the ranges.
   *
   * Each range `[a,b]` in the list should be output as:
   *  - `"a->b"` if `a != b`
   *  - `"a"` if `a == b`
   *
   * '''Constraints:'''
   *  - `-1_000_000_000 <= lower <= upper <= 1_000_000_000`
   *  - `0 <= nums.length <= 100`
   *  - `lower <= nums[i] <= upper`
   *  - All the values of `nums` are '''unique'''.
   */
  object Solution {
    private def toStrRange(lower: Int, upper: Int): String =
      if (lower == upper) s"$lower"
      else s"$lower->$upper"

    def findMissingRanges(nums: Array[Int], lower: Int, upper: Int): List[String] =
      if (nums.isEmpty) List(toStrRange(lower, upper))
      else {
        val result = collection.mutable.ListBuffer[String]()

        if (lower < nums.head)
          result.append(toStrRange(lower, nums.head - 1))

        for (i <- 0 until (nums.length - 1); j = i + 1) {
          if (nums(i) + 1 < nums(j))
            result.append(toStrRange(nums(i) + 1, nums(j) - 1))
        }

        if (nums.last < upper)
          result.append(toStrRange(nums.last + 1, upper))

        result.toList
      }
  }

  import Solution.findMissingRanges

  """Example 1: (nums = [0,1,3,50,75], lower = 0, upper = 99) -> ["2","4->49","51->74","76->99"]""" in {
    val nums = Array(0, 1, 3, 50, 75)
    val expected = List("2", "4->49", "51->74", "76->99")

    findMissingRanges(nums, 0, 99) shouldBe expected
    //Explanation: The ranges are:
    //[2,2] --> "2"
    //[4,49] --> "4->49"
    //[51,74] --> "51->74"
    //[76,99] --> "76->99"
  }
  """Example 2: (nums = [], lower = 1, upper = 1) -> ["1"]""" in {
    findMissingRanges(Array(), 1, 1) shouldBe List("1")
    //Explanation: The only missing range is [1,1], which becomes "1".
  }
  """Example 3: (nums = [], lower = -3, upper = -1) -> ["-3->-1"]""" in {
    findMissingRanges(Array(), -3, -1) shouldBe List("-3->-1")
    //Explanation: The only missing range is [-3,-1], which becomes "-3->-1".
  }
  """Example 4: (nums = [-1], lower = -1, upper = -1) -> []""" in {
    findMissingRanges(Array(-1), -1, -1) shouldBe List()
    //Explanation: There are no missing ranges since there are no missing numbers.
  }
  """Example 5: (nums = [-1], lower = -2, upper = -1) -> ["-2"]""" in {
    findMissingRanges(Array(-1), -2, -1) shouldBe List("-2")
  }

  """(nums = [1,2], lower = 0, upper = 2) -> ["0"]""" in {
    findMissingRanges(Array(1, 2), 0, 2) shouldBe List("0")
  }

  """(nums = [1..100], lower = -1_000_000_000, upper = 1_000_000_000) -> ["-1000000000->0","101->1000000000"]""" in {
    val nums = (1 to 100).toArray
    val expected = List("-1000000000->0", "101->1000000000")

    findMissingRanges(nums, -1000000000, 1000000000) shouldBe expected
  }
}
