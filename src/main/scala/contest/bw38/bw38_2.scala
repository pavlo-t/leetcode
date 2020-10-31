package contest.bw38

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class bw38_2 extends AnyWordSpec with Matchers {

  /**
   * <h3>5540. Widest Vertical Area Between Two Points Containing No Points</h3>
   *
   * Given `n` `points` on a 2D plane where `points[i] = [xi, yi]`,
   * Return <em>the <b>widest vertical area</b> between two points such that no points are inside the area</em>.
   *
   * A <b>vertical area</b> is an area of fixed-width extending infinitely along the y-axis (i.e., infinite height).
   * The <b>widest vertical area</b> is the one with the maximum width.
   *
   * Note that points <b>on the edge</b> of a vertical area <b>are not</b> considered included in the area.
   *
   * <b>Constraints:</b><ul>
   * <li> `n == points.length`
   * <li> `2 <= n <= 100_000`
   * <li> `points[i].length == 2`
   * <li> `0 <= xi, yi <= 1_000_000_000`
   * </ul>
   */
  object Solution {
    def maxWidthOfVerticalArea(points: Array[Array[Int]]): Int = {
      val pts = points.map(_ (0)).distinct.sorted
      (for (i <- 1 until pts.length) yield pts(i) - pts(i - 1)).max
    }
  }

  import Solution.maxWidthOfVerticalArea

  "Example 1: ([[8,7],[9,9],[7,4],[9,7]]) -> 1" in {
    val points = Array(Array(8, 7), Array(9, 9), Array(7, 4), Array(9, 7))
    maxWidthOfVerticalArea(points) shouldBe 1
    // Explanation: Both the red and the blue area are optimal.
  }
  "Example 2: ([[3,1],[9,0],[1,0],[1,4],[5,3],[8,8]]) -> 3" in {
    val points = Array(Array(3, 1), Array(9, 0), Array(1, 0), Array(1, 4), Array(5, 3), Array(8, 8))
    maxWidthOfVerticalArea(points) shouldBe 3
  }

  "(100_000 points) -> 1" in {
    val points = Array.ofDim[Array[Int]](100_000)
    for (i <- points.indices) points(i) = Array(i, 0)
    maxWidthOfVerticalArea(points) shouldBe 1
  }
}
