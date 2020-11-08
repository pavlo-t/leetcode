package contest.w213

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/weekly-contest-213/problems/furthest-building-you-can-reach/]]
 */
//noinspection DuplicatedCode
class w213_3 extends AnyWordSpec with Matchers {

  /**
   * <h3>1642. Furthest Building You Can Reach</h3>
   *
   * You are given an integer array `heights` representing the heights of buildings, some `bricks`, and some `ladders`.
   *
   * You start your journey from building `0` and move to the next building by possibly using bricks or ladders.
   *
   * While moving from building `i` to building `i+1` (<b>0-indexed</b>),<ul>
   * <li> If the current building's height is <b>greater than or equal</b> to the next building's height,
   * you do <b>not</b> need a ladder or bricks.
   * <li> If the current building's height is <b>less than</b> the next building's height,
   * you can either use <b>one ladder</b> or `(h[i+1] - h[i])` <b>bricks</b>.
   * </ul>
   *
   * <em>Return the furthest building index (0-indexed) you can reach if you use the given ladders and bricks
   * optimally</em>.<br>
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= heights.length <= 100_000`
   * <li> `1 <= heights[i] <= 1000_000`
   * <li> `0 <= bricks <= 1_000_000_000`
   * <li> `0 <= ladders <= heights.length`
   * </ul>
   */
  object Solution {
    def furthestBuilding(heights: Array[Int], bricks: Int, ladders: Int): Int = {
      import collection.mutable

      val deltas = mutable.PriorityQueue[Int]()(Ordering[Int].reverse)

      @scala.annotation.tailrec
      def getFurthestBuilding(i: Int, bricksNeeded: Int): Int = {
        if (i >= heights.length) i - 1
        else {
          val delta = heights(i) - heights(i - 1)
          if (delta <= 0) getFurthestBuilding(i + 1, bricksNeeded)
          else {
            deltas.enqueue(delta)
            if (deltas.size <= ladders) getFurthestBuilding(i + 1, bricksNeeded)
            else {
              val newBricksNeeded = bricksNeeded + deltas.dequeue()
              if (newBricksNeeded <= bricks) getFurthestBuilding(i + 1, newBricksNeeded)
              else i - 1
            }
          }
        }
      }

      getFurthestBuilding(1, 0)
    }
  }
  object SolutionWhile {
    def furthestBuilding(heights: Array[Int], bricks: Int, ladders: Int): Int = {
      import collection.mutable

      val pq = mutable.PriorityQueue[Int]()(Ordering[Int].reverse)
      var bricksNeeded = 0

      for (i <- 0 until (heights.length - 1)) {
        val delta = heights(i + 1) - heights(i)
        if (delta > 0) {
          pq.enqueue(delta)
          if (pq.size > ladders) {
            bricksNeeded += pq.dequeue()
            if (bricksNeeded > bricks)
              return i
          }
        }
      }

      heights.length - 1
    }
  }

  import Solution.furthestBuilding

  "Example 1: (heights = [4,2,7,6,9,14,12], bricks = 5, ladders = 1) -> 4" in {
    furthestBuilding(Array(4, 2, 7, 6, 9, 14, 12), 5, 1) shouldBe 4
    // Explanation: Starting at building 0, you can follow these steps:
    // - Go to building 1 without using ladders nor bricks since 4 >= 2.
    // - Go to building 2 using 5 bricks. You must use either bricks or ladders because 2 < 7.
    // - Go to building 3 without using ladders nor bricks since 7 >= 6.
    // - Go to building 4 using your only ladder. You must use either bricks or ladders because 6 < 9.
    // It is impossible to go beyond building 4 because you do not have any more bricks or ladders.
  }
  "Example 2: (heights = [4,12,2,7,3,18,20,3,19], bricks = 10, ladders = 2) -> 7" in {
    furthestBuilding(Array(4, 12, 2, 7, 3, 18, 20, 3, 19), 10, 2) shouldBe 7
  }
  "Example 3: (heights = [14,3,19,3], bricks = 17, ladders = 0) -> 3" in {
    furthestBuilding(Array(14, 3, 19, 3), 17, 0) shouldBe 3
  }

  "(heights = [max length], bricks = max, ladders = 0) -> 99_999" in {
    val heights = Array.ofDim[Int](100_000)
    for (i <- heights.indices) heights(i) = i + 1

    furthestBuilding(heights, 100_000, 0) shouldBe 99_999
  }

  "(heights = [max length], bricks = max, ladders = max / 2) -> 99_999" in {
    val heights = Array.ofDim[Int](100_000)
    for (i <- heights.indices) heights(i) = (i + 1) * 2

    furthestBuilding(heights, 1_000_000_000, 50_000) shouldBe 99_999
  }
}
