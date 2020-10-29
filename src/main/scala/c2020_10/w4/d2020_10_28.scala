package c2020_10.w4

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_28 extends AnyWordSpec with Matchers {

  /**
   * <h3>Summary Ranges</h3>
   *
   * You are given a <b>sorted unique</b> integer array `nums`.
   *
   * Return <em>the <b>smallest sorted</b> list of ranges that <b>cover all the numbers in the array exactly</b></em>.
   * That is, each element of `nums` is covered by exactly one of the ranges,
   * and there is no integer `x` such that `x` is in one of the ranges but not in `nums`.
   *
   * Each range `[a,b]` in the list should be output as:<ul>
   * <li> `"a->b"` if `a != b`
   * <li> `"a"` if `a == b`
   * </ul>
   *
   * <b>Constraints:</b><ul>
   * <li> `0 <= nums.length <= 20`
   * <li> `-2**31 <= nums[i] <= 2**31 - 1`
   * <li> All the values of `nums` are unique.
   * </ul>
   */
  object Solution {
    import scala.annotation.tailrec

    def summaryRanges(nums: Array[Int]): List[String] =
      if (nums.isEmpty) List()
      else {
        def range(f: Int, l: Int): String =
          if (f == l) f.toString
          else s"$f->$l"

        @tailrec
        def toRanges(ns: List[Int], first: Int, last: Int, rsf: List[String]): List[String] = ns match {
          case Nil      => range(first, last) :: rsf
          case n :: nss =>
            if (n == last + 1) toRanges(nss, first, n, rsf)
            else toRanges(nss, n, n, range(first, last) :: rsf)
        }

        val ns = nums.toList
        toRanges(ns.tail, ns.head, ns.head, List()).reverse
      }
  }

  import Solution.summaryRanges

  """Example 1: ([0,1,2,4,5,7]) -> ["0->2","4->5","7"]""" in {
    val nums = Array(0, 1, 2, 4, 5, 7)
    val expected = Array("0->2", "4->5", "7")

    summaryRanges(nums) shouldBe expected
    // Explanation: The ranges are:
    //   [0,2] --> "0->2"
    //   [4,5] --> "4->5"
    //   [7,7] --> "7"
  }
  """Example 2: ([0,2,3,4,6,8,9]) -> ["0","2->4","6","8->9"]""" in {
    val nums = Array(0, 2, 3, 4, 6, 8, 9)
    val expected = Array("0", "2->4", "6", "8->9")

    summaryRanges(nums) shouldBe expected
    // Explanation: The ranges are:
    //   [0,0] --> "0"
    //   [2,4] --> "2->4"
    //   [6,6] --> "6"
    //   [8,9] --> "8->9"
  }
  """Example 3: ([]) -> []""" in {
    summaryRanges(Array()) shouldBe Array()
  }
  """Example 4: ([-1]) -> ["-1"]""" in {
    summaryRanges(Array(-1)) shouldBe Array("-1")
  }
  """Example 5: ([0]) -> ["0"]""" in {
    summaryRanges(Array(0)) shouldBe Array("0")
  }
}
