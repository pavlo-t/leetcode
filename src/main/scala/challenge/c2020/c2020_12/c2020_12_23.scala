package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3578/]]
 */
//noinspection DuplicatedCode
class c2020_12_23 extends AnyWordSpec with Matchers {

  /**
   * === Next Greater Element III ===
   *
   * Given a positive integer `n`, find ''the smallest integer which has exactly the same digits existing in the integer''
   * `n` ''and is greater in value than'' `n`.
   * If no such positive integer exists, return `-1`.
   *
   * '''Note''' that the returned integer should fit in '''32-bit integer''',
   * if there is a valid answer but it does not fit in '''32-bit integer''', return `-1`.
   *
   * '''Constraints:'''
   *  - `1 <= n <= 2^31 - 1`
   */
  object Solution {
    def nextGreaterElement(n: Int): Int =
      if (n < 12) -1
      else {
        val str = n.toString
        for (r <- str.length - 1 to 1 by -1; l = r - 1 if str(l) < str(r)) {
          val (lStr, rStr) = str.splitAt(r)
          val rSorted = rStr.reverse
          val i = rSorted.indexWhere(_ > str(l))

          (lStr.updated(l, rSorted(i)) + rSorted.updated(i, lStr(l))).toIntOption match {
            case None    => return -1
            case Some(i) => return i
          }
        }
        -1
      }
  }

  import Solution.nextGreaterElement

  "Example 1: (n = 12) -> 21" in (nextGreaterElement(12) shouldBe 21)
  "Example 2: (n = 21) -> -1" in (nextGreaterElement(21) shouldBe -1)

  "Test 30: (n = 230241) -> 230412" in (nextGreaterElement(230241) shouldBe 230412)
  "Test 34: (n = 12443322) -> 13222344" in (nextGreaterElement(12443322) shouldBe 13222344)

  "(n = 22) -> -1" in (nextGreaterElement(22) shouldBe -1)
  "(n = 23) -> 32" in (nextGreaterElement(23) shouldBe 32)
  "(n = 213) -> 231" in (nextGreaterElement(213) shouldBe 231)

  "(n = 2147483647 (== 2^31 - 1)) -> -1" in (nextGreaterElement(2147483647) shouldBe -1)
  "(n = 2147483646 (== 2^31 - 2)) -> -1" in (nextGreaterElement(2147483646) shouldBe -1)

}
