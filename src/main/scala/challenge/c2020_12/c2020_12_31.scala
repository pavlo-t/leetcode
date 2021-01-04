package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/573/week-5-december-29th-december-31st/3587/]]
 */
//noinspection DuplicatedCode
class c2020_12_31 extends AnyWordSpec with Matchers {
  /**
   * === Largest Rectangle in Histogram ===
   *
   * Given ''n'' non-negative integers representing the histogram's bar height where the width of each bar is 1,
   * find the area of largest rectangle in the histogram.
   *
   * {{{
   * 6    I
   * 5   II
   * 4   II
   * 3   II I
   * 2 I IIII
   * 1 IIIIII
   * }}}
   *
   * Above is a histogram where width of each bar is 1, given height = `[2,1,5,6,2,3]`.
   *
   * {{{
   * 6    I
   * 5   **
   * 4   **
   * 3   ** I
   * 2 I **II
   * 1 II**II
   * }}}
   *
   * The largest rectangle is shown in the shaded area, which has area = `10` unit.
   */
  object Solution {
    def largestRectangleArea(H: Array[Int]): Int = {
      @scala.annotation.tailrec
      def rec(i: Int, stack: List[Int], rsf: Int): Int = {
        if (i == H.length) stack match {
          case -1 :: Nil => rsf
          case j :: rest => rec(i, rest, rsf max (H(j) * (i - rest.head - 1)))
        } else stack match {
          case j :: rest if j >= 0 && H(j) >= H(i) => rec(i, rest, rsf max (H(j) * (i - rest.head - 1)))
          case _                                   => rec(i + 1, i :: stack, rsf)
        }
      }

      rec(0, -1 :: Nil, 0)
    }
  }

  object SolutionDivideAndConquer {
    def largestRectangleArea(H: Array[Int]): Int = {
      def dc(l: Int, r: Int): Int = {
        if (l > r) 0
        else if (l == r) H(l)
        else {
          val (min, mid) = (l to r).foldLeft((Int.MaxValue, -1)) {
            case ((min, _), j) if H(j) < min => (H(j), j)
            case (acc, _)                    => acc
          }
          (min * (r - l + 1))
            .max(dc(l, mid - 1))
            .max(dc(mid + 1, r))
        }
      }

      dc(0, H.length - 1)
    }
  }

  object SolutionIterativeOnnTimeO1Space {
    def largestRectangleArea(heights: Array[Int]): Int = {
      var result = 0
      for (l <- heights.indices) {
        var minHeight = Int.MaxValue
        for (r <- l until heights.length) {
          minHeight = minHeight min heights(r)
          result = result max ((r - l + 1) * minHeight)
        }
      }
      result
    }
  }

  object SolutionRecursionOnnTimeO1Space {
    def largestRectangleArea(heights: Array[Int]): Int = {
      @scala.annotation.tailrec
      def rec(l: Int, r: Int, min: Int, rsf: Int): Int = {
        if (l == heights.length) rsf
        else if (r == heights.length) rec(l + 1, l + 1, Int.MaxValue, rsf)
        else if (heights(r) < min) rec(l, r + 1, heights(r), rsf max ((r - l + 1) * heights(r)))
        else rec(l, r + 1, min, rsf max ((r - l + 1) * min))
      }
      rec(0, 0, Int.MaxValue, 0)
    }
  }

  import Solution.largestRectangleArea

  "Example: ([2,1,5,6,2,3]) -> 10" in {
    largestRectangleArea(Array(2, 1, 5, 6, 2, 3)) shouldBe 10
  }
  "Test 2: ([]) -> 0" in {
    largestRectangleArea(Array()) shouldBe 0
  }
  "([1]) -> 1" in {
    largestRectangleArea(Array(1)) shouldBe 1
  }
  "([2,1,5,6,2,3,4,5]) -> 12" in {
    largestRectangleArea(Array(2, 1, 5, 6, 2, 3, 4, 5)) shouldBe 12
  }
  "([2,1,5,12,2,3]) -> 12" in {
    largestRectangleArea(Array(2, 1, 5, 12, 2, 3)) shouldBe 12
  }

}
