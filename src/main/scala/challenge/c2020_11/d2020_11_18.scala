package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/566/week-3-november-15th-november-21st/3535/]]
 */
//noinspection DuplicatedCode
class d2020_11_18 extends AnyWordSpec with Matchers {

  /**
   * === Merge Intervals ===
   *
   * Given an array of `intervals` where `intervals[i] = [iStart, iEnd]`, merge all overlapping intervals,
   * and return ''an array of the non-overlapping intervals that cover all the intervals in the input''.
   *
   * '''Constraints:'''
   *  - `1 <= intervals.length <= 10_000`
   *  - `intervals[i].length == 2`
   *  - `0 <= iStart <= iEnd <= 10_000`
   */
  object Solution {
    def merge(intervals: Array[Array[Int]]): Array[Array[Int]] =
      intervals
        .sortBy(_ (0))
        .foldLeft(Array[Array[Int]]()) { case (acc, Array(s, e)) =>
          if (acc.nonEmpty && acc.last(1) >= s) {
            acc.last(1) = acc.last(1) max e
            acc
          }
          else acc.appended(Array(s, e))
        }
  }
  object SolutionWithArrayBuffer {
    def merge(intervals: Array[Array[Int]]): Array[Array[Int]] =
      intervals
        .map { case Array(s, e) => (s, e) }
        .sortInPlace()
        .foldLeft(collection.mutable.ArrayBuffer[Array[Int]]()) { case (acc, (s, e)) =>
          if (acc.nonEmpty && acc.last(1) >= s) {
            acc.last(1) = acc.last(1) max e
            acc
          }
          else acc.addOne(Array(s, e))
        }
        .toArray
  }
  object Solution2 {
    def merge(intervals: Array[Array[Int]]): Array[Array[Int]] =
      intervals
        .map { case Array(s, e) => (s, e) }
        .sorted
        .foldLeft(Seq[(Int, Int)]()) {
          case (Nil, i)                                => Seq(i)
          case (acc :+ ls -> le, is -> ie) if le >= is => acc.appended((ls, le max ie))
          case (acc, i)                                => acc.appended(i)
        }
        .map { case (s, e) => Array(s, e) }
        .toArray
  }
  object Solution1 {
    def merge(intervals: Array[Array[Int]]): Array[Array[Int]] =
      intervals
        .map { case Array(s, e) => (s, e) }
        .sorted
        .foldLeft(Seq[(Int, Int)]()) { case (acc, i@(s, e)) =>
          if (acc.isEmpty) Seq(i)
          else if (acc.last._2 >= s) acc.updated(acc.length - 1, (acc.last._1, acc.last._2 max e))
          else acc.appended(i)
        }
        .map { case (s, e) => Array(s, e) }
        .toArray
  }

  import Solution.merge

  "Example 1: (intervals = [[1,3],[2,6],[8,10],[15,18]]) -> [[1,6],[8,10],[15,18]]" in {
    val intervals = Array(Array(1, 3), Array(2, 6), Array(8, 10), Array(15, 18))
    val expected = Array(Array(1, 6), Array(8, 10), Array(15, 18))
    merge(intervals) shouldBe expected
    //Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
  }
  "Example 2: (intervals = [[1,4],[4,5]]) -> [[1,5]]" in {
    merge(Array(Array(1, 4), Array(4, 5))) shouldBe Array(Array(1, 5))
    //Explanation: Intervals [1,4] and [4,5] are considered overlapping.
  }

  "Test 19: (intervals = [[1,4],[0,4]]) -> [[0,4]]" in {
    merge(Array(Array(1, 4), Array(0, 4))) shouldBe Array(Array(0, 4))
  }

  "(intervals = [10_000 intervals: [1,2],[2,3]...]) -> [[1,10_000]]" in {
    val intervals = (1 to 10_000).map(i => Array(i, i + 1)).toArray
    val expected = Array(Array(1, 10_001))
    merge(intervals) shouldBe expected
  }
}
