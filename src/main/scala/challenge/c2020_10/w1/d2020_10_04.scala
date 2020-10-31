package challenge.c2020_10.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d2020_10_04 extends AnyWordSpec with Matchers {

  /**
   * Remove Covered Intervals
   *
   * Given a list of `intervals`, remove all intervals that are covered by another interval in the list.
   *
   * Interval `[a,b)` is covered by interval `[c,d)` if and only if `c <= a` and `b <= d`.
   *
   * After doing so, return <em>the number of remaining intervals</em>.
   *
   * Constraints:<ul>
   * <li> `1 <= intervals.length <= 1000`
   * <li> `intervals[i].length == 2`
   * <li> `0 <= intervals[i][0] < intervals[i][1] <= 10^5`
   * <li> All the intervals are <b>unique</b>.
   */
  object Solution {
    import scala.annotation.tailrec

    def removeCoveredIntervals(intervals: Array[Array[Int]]): Int = {
      var result = 0

      @tailrec
      def isCovered(interval: Array[Int], idx: Int, otherIdx: Int): Boolean = {
        if (otherIdx == intervals.length) false
        else if (idx == otherIdx) isCovered(interval, idx, otherIdx + 1)
        else if (intervals(otherIdx)(0) <= interval(0) && intervals(otherIdx)(1) >= interval(1)) true
        else isCovered(interval, idx, otherIdx + 1)
      }

      for ((i, idx) <- intervals.zipWithIndex) {
        if (!isCovered(i, idx, 0)) result += 1
      }

      result
    }
  }

  "Example 1" in {
    Solution.removeCoveredIntervals(Array(Array(1, 4), Array(3, 6), Array(2, 8))) shouldBe 2
    // Explanation: Interval [3,6] is covered by [2,8], therefore it is removed.
  }
  "Example 2" in {
    Solution.removeCoveredIntervals(Array(Array(1, 4), Array(2, 3))) shouldBe 1
  }
  "Example 3" in {
    Solution.removeCoveredIntervals(Array(Array(0, 10), Array(5, 12))) shouldBe 2
  }
  "Example 4" in {
    Solution.removeCoveredIntervals(Array(Array(3, 10), Array(4, 10), Array(5, 11))) shouldBe 2
  }
  "Example 5" in {
    Solution.removeCoveredIntervals(Array(Array(1, 2), Array(1, 4), Array(3, 4))) shouldBe 1
  }
}
