package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_11_11 extends AnyWordSpec with Matchers {

  /**
   * <h3> Valid Square</h3>
   *
   * Given the coordinates of four points in 2D space, return whether the four points could construct a square.
   *
   * The coordinate (x,y) of a point is represented by an integer array with two integers.
   *
   * <b>Note:</b><ul>
   * <li> All the input integers are in the range `[-10000, 10000]`.
   * <li> A valid square has four equal sides with positive length and four equal angles (90-degree angles).
   * <li> Input points have no order.
   * </ul>
   */
  object Solution {
    def validSquare(a1: Array[Int], a2: Array[Int], a3: Array[Int], a4: Array[Int]): Boolean =
      Seq(a1, a2, a3, a4).map(p => (p(0), p(1))).sorted match {
        case ps if ps.distinct.size < 4      => false
        case Seq((x1, y1), (x2, y2), p3, p4) =>
          val dx = x2 - x1
          val dy = y2 - y1
          if (dy < 0) p3 == (x1 - dy, y1 + dx) && p4 == (x2 - dy, y2 + dx)
          else p3 == (x1 + dy, y1 - dx) && p4 == (x2 + dy, y2 - dx)
      }
  }

  object Solution1 {
    def validSquare(a1: Array[Int], a2: Array[Int], a3: Array[Int], a4: Array[Int]): Boolean = {
      val Seq(p1@(x1, y1), p2@(x2, y2), p3, p4) = Seq(a1, a2, a3, a4).map(p => (p(0), p(1))).sorted
      val dx = x2 - x1
      val dy = y2 - y1
      if (p1 == p2 && p1 == p3 && p1 == p4) false
      else if (dy < 0) p3 == (x1 - dy, y1 + dx) && p4 == (x2 - dy, y2 + dx)
      else p3 == (x1 + dy, y1 - dx) && p4 == (x2 + dy, y2 - dx)
    }
  }

  /**
   * [[https://leetcode.com/problems/valid-square/solution/]]
   */
  object SolutionBruteForceOptimised {
    def validSquare(p1: Array[Int], p2: Array[Int], p3: Array[Int], p4: Array[Int]): Boolean = {
      def ds(p1: Array[Int], p2: Array[Int]): Double = math.pow(p1(0) - p2(0), 2) + math.pow(p1(1) - p2(1), 2)

      def check(p1: Array[Int], p2: Array[Int], p3: Array[Int], p4: Array[Int]): Boolean =
        ds(p1, p2) > 0 &&
          // sides
          ds(p1, p2) == ds(p2, p3) &&
          ds(p2, p3) == ds(p3, p4) &&
          ds(p3, p4) == ds(p4, p1) &&
          // diagonals
          ds(p1, p3) == ds(p2, p4)

      check(p1, p2, p3, p4) || check(p1, p3, p2, p4) || check(p1, p2, p4, p3)
    }

  }

  import Solution.validSquare

  "Example: (p1 = [0,0], p2 = [1,1], p3 = [1,0], p4 = [0,1]) -> true" in {
    validSquare(Array(0, 0), Array(1, 1), Array(1, 0), Array(0, 1)) shouldBe true
  }

  "Test 195: ([1,0],[-1,0],[0,-1],[0,1]) -> true" in {
    validSquare(Array(1, 0), Array(-1, 0), Array(0, -1), Array(0, 1)) shouldBe true
  }
  "Test 244: ([0,0],[0,0],[0,0],[0,0]) -> false" in {
    validSquare(Array(0, 0), Array(0, 0), Array(0, 0), Array(0, 0)) shouldBe false
  }

  "([0,0],[0,1],[2,0],[2,1]) -> false" in {
    validSquare(Array(0, 0), Array(0, 1), Array(2, 0), Array(2, 1)) shouldBe false
  }
  "45 degrees turned square ([0,1],[1,0],[1,2],[2,1]) -> true" in {
    validSquare(Array(0, 1), Array(1, 0), Array(1, 2), Array(2, 1)) shouldBe true
  }
  "60 degrees turned square ([0,2],[1,0],[2,3],[3,1]) -> true" in {
    validSquare(Array(0, 2), Array(1, 0), Array(2, 3), Array(3, 1)) shouldBe true
  }
  "60 degrees turned square ([0,2],[1,0],[-2,1],[-1,-1]) -> true" in {
    validSquare(Array(0, 2), Array(1, 0), Array(-2, 1), Array(-1, -1)) shouldBe true
  }
  "30 degrees turned square ([3,2],[1,3],[2,0],[0,1]) -> true" in {
    validSquare(Array(3, 2), Array(1, 3), Array(2, 0), Array(0, 1)) shouldBe true
  }
}
