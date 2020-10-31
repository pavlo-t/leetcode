package challenge.c2020_10.w5

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_29 extends AnyWordSpec with Matchers {

  /**
   * <h3>Maximize Distance to Closest Person</h3>
   *
   * You are given an array representing a row of `seats`
   * where `seats[i] = 1` represents a person sitting in the `i`th seat,
   * and `seats[i] = 0` represents that the `i`th seat is empty <b>(0-indexed)</b>.
   *
   * There is at least one empty seat, and at least one person sitting.
   *
   * Alex wants to sit in the seat such that the distance between him and the closest person to him is maximized.
   *
   * Return <em>that maximum distance to the closest person</em>.
   *
   * <b>Constraints:</b><ul>
   * <li> `2 <= seats.length <= 20_000`
   * <li> `seats[i]` is `0` or `1`.
   * <li> At least one seat is <b>empty</b>.
   * <li> At least one seat is <b>occupied</b>.
   * </ul>
   */
object Solution {
  def maxDistToClosest(seats: Array[Int]): Int = {
    var maxDist = 0
    var i = 0

    // Handle prefix:
    while (seats(i) == 0) {
      maxDist += 1
      i += 1
    }

    // Handle middle:
    var currentDist = 0
    while (i < seats.length) {
      if (seats(i) == 1) {
        maxDist = math.max(maxDist, (currentDist + 1) / 2)
        currentDist = 0
      } else {
        currentDist += 1
      }
      i += 1
    }

    // Handle suffix:
    if (seats.last == 0)
      maxDist = math.max(maxDist, currentDist)

    maxDist
  }
}

  import Solution.maxDistToClosest

  "Example 1: ([1,0,0,0,1,0,1]) -> 2" in {
    val seats = Array(1, 0, 0, 0, 1, 0, 1)
    maxDistToClosest(seats) shouldBe 2
    // Explanation:
    //   If Alex sits in the second open seat (i.e. seats[2]), then the closest person has distance 2.
    //   If Alex sits in any other open seat, the closest person has distance 1.
    //   Thus, the maximum distance to the closest person is 2.
  }
  "Example 2: ([1,0,0,0]) -> 3" in {
    val seats = Array(1, 0, 0, 0)
    maxDistToClosest(seats) shouldBe 3
    // Explanation:
    //   If Alex sits in the last seat (i.e. seats[3]), the closest person is 3 seats away.
    //   This is the maximum distance possible, so the answer is 3.
  }
  "Example 3: ([0,1]) -> 1" in {
    val seats = Array(0, 1)
    maxDistToClosest(seats) shouldBe 1
  }

  "([1,0,0,0,0,1,0,1]) -> 2" in {
    val seats = Array(1, 0, 0, 0, 0, 1, 0, 1)
    maxDistToClosest(seats) shouldBe 2
  }
  "([0,0,1]) -> 2" in {
    val seats = Array(0, 0, 1)
    maxDistToClosest(seats) shouldBe 2
  }
  "([1,0]) -> 1" in {
    val seats = Array(1, 0)
    maxDistToClosest(seats) shouldBe 1
  }
  "([1,0,0]) -> 2" in {
    val seats = Array(1, 0, 0)
    maxDistToClosest(seats) shouldBe 2
  }
}
