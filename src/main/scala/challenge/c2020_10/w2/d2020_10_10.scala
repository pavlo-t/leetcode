package challenge.c2020_10.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


//noinspection DuplicatedCode
class d2020_10_10 extends AnyWordSpec with Matchers {

  /**
   * Minimum Number of Arrows to Burst Balloons
   *
   * There are some spherical balloons spread in two-dimensional space.
   * For each balloon, provided input is the start and end coordinates of the horizontal diameter.
   * Since it's horizontal, y-coordinates don't matter, and hence the x-coordinates of start and end of the diameter suffice.
   * The start is always smaller than the end.
   *
   * An arrow can be shot up exactly vertically from different points along the x-axis.
   * A balloon with <code>x<sub>start</sub></code> and <code>x<sub>end</sub></code>
   * bursts by an arrow shot at `x` if <code>x<sub>start</sub> ≤ x ≤ x<sub>end</sub></code>.
   * There is no limit to the number of arrows that can be shot.
   * An arrow once shot keeps traveling up infinitely.
   *
   * Given an array `points` where <code>points[i] = [x<sub>start</sub>, x<sub>end</sub>]</code>,
   * return <em>the minimum number of arrows that must be shot to burst all balloons</em>.
   *
   *
   * <b>Constraints:</b><ul>
   * <li> <code>0 <= points.length <= 10<sup>4</sup></code>
   * <li> <code>points.length == 2</code>
   * <li> <code>-2<sup>31</sup> <= x<sub>start</sub> < x<sub>end</sub> <= 2<sup>31</sup> - 1</code>
   * </ul>
   */
  object Solution {
    import scala.annotation.tailrec

    def findMinArrowShots1(points: Array[Array[Int]]): Int = {
      val hits = getHits(points)

      @tailrec
      def findBestHits(todo: Seq[(Set[Int], Int)], rsf: Set[Int]): Int = {
        if (todo.isEmpty) rsf.min
        else {
          val (current, cnt) = todo.head
          val next = hits.filter(_.diff(current).nonEmpty).map(_ ++ current -> (cnt + 1)).toSeq
          if (next.isEmpty) findBestHits(todo.tail, rsf + cnt)
          else findBestHits(todo.tail ++ next, rsf)
        }
      }

      findBestHits(Seq(Set[Int]() -> 0), Set())
    }

    def getHits(points: Array[Array[Int]]): Set[Set[Int]] = {
      val indices = points.indices.toSet

      @tailrec
      def loop(todo: Set[Set[Int]], rsf: Set[Set[Int]]): Set[Set[Int]] = {
        if (todo.isEmpty) rsf
        else {
          val current = todo.head
          val next =
            indices
              .diff(current)
              .filter(j => current.forall(i => overlaps(points(i), points(j))))
              .map(current + _)
          if (next.isEmpty) loop(todo.tail, rsf + current)
          else loop(todo.tail ++ next, rsf)
        }
      }

      loop(Set(Set()), Set())
    }

    def overlaps(l: Array[Int], r: Array[Int]): Boolean =
      !(l(1) < r(0) || l(0) > r(1))

    def findMinArrowShots(points: Array[Array[Int]]): Int = {
      if (points.length == 0) 0
      else {
        points.sortInPlaceWith(_ (1) < _ (1))

        var current = points(0)(1)
        var count = 1

        for (p <- points.tail if p(0) > current) {
          current = p(1)
          count += 1
        }

        count
      }
    }
  }

  "Example 1" in {
    val points = Array(Array(10, 16), Array(2, 8), Array(1, 6), Array(7, 12))
    Solution.findMinArrowShots(points) shouldBe 2
    // Explanation: One way is to shoot one arrow for example at x = 6
    //   (bursting the balloons [2,8] and [1,6]) and another arrow at x = 11 (bursting the other two balloons).
  }
  "Example 2" in {
    val points = Array(Array(1, 2), Array(3, 4), Array(5, 6), Array(7, 8))
    Solution.findMinArrowShots(points) shouldBe 4
  }
  "Example 3" in {
    val points = Array(Array(1, 2), Array(2, 3), Array(3, 4), Array(4, 5))
    Solution.findMinArrowShots(points) shouldBe 2
  }
  "Example 4" in {
    val points = Array(Array(1, 2))
    Solution.findMinArrowShots(points) shouldBe 1
  }
  "Example 5" in {
    val points = Array(Array(2, 3), Array(2, 3))
    Solution.findMinArrowShots(points) shouldBe 1
  }

  "Test 16" in {
    val points = Array(Array(10, 16), Array(2, 8), Array(1, 6), Array(7, 12))
    Solution.findMinArrowShots(points) shouldBe 2
  }
  "Test 24" in {
    val points = Array(Array(3, 9), Array(7, 12), Array(3, 8), Array(6, 8), Array(9, 10), Array(2, 9), Array(0, 9),
      Array(3, 9), Array(0, 6), Array(2, 8))
    Solution.findMinArrowShots(points) shouldBe 2
  }
  "Test 31" in {
    //points: [[4289383,51220269,0], [81692777,96329692,1], [57747793,81986128,2], [19885386,69645878,3], [96516649,186158070,4], [25202362,75692389,5], [83368690,85888749,6], [44897763,112411689,7], [65180540,105563966,8], [4089172,7544908,9]]
    //points: [[4089172,7544908,9], [4289383,51220269,0], [19885386,69645878,3], [25202362,75692389,5], [57747793,81986128,2], [83368690,85888749,6], [81692777,96329692,1], [65180540,105563966,8], [44897763,112411689,7], [96516649,186158070,4]]
    //current = 7544908 count = 1
    //  hit: [4089172, 7544908, 9]
    //  hit: [4289383, 51220269, 0]
    //current = 69645878 count = 2
    //  hit: [19885386, 69645878, 3]
    //  hit: [25202362, 75692389, 5]
    //  hit: [57747793, 81986128, 2]
    //current = 85888749 count = 3
    //  hit: [83368690, 85888749, 6]
    //  hit: [81692777, 96329692, 1]
    //  hit: [65180540, 105563966, 8]
    //  hit: [44897763, 112411689, 7]
    //current = 186158070 count = 4
    //  hit: [96516649, 186158070, 4]
    val points =
    Array(
      Array(4289383, 51220269),
      Array(81692777, 96329692),
      Array(57747793, 81986128),
      Array(19885386, 69645878),
      Array(96516649, 186158070),
      Array(25202362, 75692389),
      Array(83368690, 85888749),
      Array(44897763, 112411689),
      Array(65180540, 105563966),
      Array(4089172, 7544908))
    Solution.findMinArrowShots(points) shouldBe 4
  }

  "My test: empty array" in {
    Solution.findMinArrowShots(Array()) shouldBe 0
  }
  "My test: max and min value" in {
    Int.MaxValue shouldBe (math.pow(2, 31) - 1)
    Int.MinValue shouldBe -math.pow(2, 31)

    val points = Array(Array(Int.MinValue, Int.MaxValue), Array(1, 2))

    Solution.findMinArrowShots(points) shouldBe 1
  }

  "My test Solution.overlaps" in {
    Solution.overlaps(Array(1, 2), Array(2, 4)) shouldBe true
    Solution.overlaps(Array(1, 2), Array(0, 1)) shouldBe true
    Solution.overlaps(Array(1, 4), Array(2, 3)) shouldBe true
    Solution.overlaps(Array(2, 3), Array(1, 4)) shouldBe true

    Solution.overlaps(Array(1, 2), Array(3, 4)) shouldBe false
    Solution.overlaps(Array(3, 4), Array(1, 2)) shouldBe false
  }
  "My test Solution.overlaps: Test 31" in {
    val a1 = Array(19885386, 69645878)
    val a2 = Array(25202362, 75692389)
    val a3 = Array(57747793, 81986128)

    Solution.overlaps(a1, a2) shouldBe true
    Solution.overlaps(a1, a3) shouldBe true
    Solution.overlaps(a2, a3) shouldBe true
    Solution.overlaps(a2, a1) shouldBe true
    Solution.overlaps(a3, a1) shouldBe true
    Solution.overlaps(a3, a2) shouldBe true
  }
  "My test Solution.getHits: Test 31" in {
    val points =
      Array(
        Array(4289383, 51220269),
        Array(81692777, 96329692),
        Array(57747793, 81986128),
        Array(19885386, 69645878),
        Array(96516649, 186158070),
        Array(25202362, 75692389),
        Array(83368690, 85888749),
        Array(44897763, 112411689),
        Array(65180540, 105563966),
        Array(4089172, 7544908))
    val expected = Set(Set(5, 0, 7, 3), Set(4, 8, 7), Set(5, 2, 7, 3, 8), Set(0, 9), Set(8, 7, 2, 1), Set(8, 7, 1, 6))

    Solution.getHits(points) shouldBe expected
  }

}
