package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/566/week-3-november-15th-november-21st/3534/]]
 */
//noinspection DuplicatedCode
class d2020_11_17 extends AnyWordSpec with Matchers {

  /**
   * === Mirror Reflection ===
   *
   * There is a special square room with mirrors on each of the four walls.
   * Except for the southwest corner, there are receptors on each of the remaining corners, numbered `0`, `1`, and `2`.
   *
   * The square room has walls of length `p`, and a laser ray from the southwest corner first meets the east wall
   * at a distance `q` from the `0`th receptor.
   *
   * Return the number of the receptor that the ray meets first.
   * (It is guaranteed that the ray will meet a receptor eventually.)
   *
   * '''Note:'''
   *  1. `1 <= p <= 1000`
   *  1. `0 <= q <= p`
   */
  object Solution {
    def mirrorReflection(p: Int, q: Int): Int = {
      val gcd = (BigInt(p) gcd BigInt(q)).toInt

      if ((p / gcd) % 2 == 0) 2
      else if ((q / gcd) % 2 == 1) 1
      else 0
    }
  }
  object SolutionMy {
    def mirrorReflection(p: Int, q: Int): Int = {
      @scala.annotation.tailrec
      def countReflections(cnt: Int, sum: Int): (Int, Int) =
        if (sum % p == 0) (cnt, sum)
        else countReflections(cnt + 1, sum + q)

      val (cnt, sum) = countReflections(1, q)

      if (cnt % 2 == 0) 2
      else if ((sum / p) % 2 == 0) 0
      else 1
    }
  }

  import Solution.mirrorReflection

  "Example 1: (p = 2, q = 1) -> 2" in {
    mirrorReflection(2, 1) shouldBe 2
    //Explanation: The ray meets receptor 2 the first time it gets reflected back to the left wall.
  }

  "p = 1" in {
    mirrorReflection(1, 0) shouldBe 0
    mirrorReflection(1, 1) shouldBe 1
  }
  "p = 2" in {
    mirrorReflection(2, 0) shouldBe 0
    mirrorReflection(2, 1) shouldBe 2
    mirrorReflection(2, 2) shouldBe 1
  }
  "p = 3" in {
    mirrorReflection(3, 0) shouldBe 0
    mirrorReflection(3, 1) shouldBe 1
    mirrorReflection(3, 2) shouldBe 0
    mirrorReflection(3, 3) shouldBe 1
  }
  "p = 4" in {
    mirrorReflection(4, 0) shouldBe 0
    mirrorReflection(4, 1) shouldBe 2
    mirrorReflection(4, 2) shouldBe 2
    mirrorReflection(4, 3) shouldBe 2
    mirrorReflection(4, 4) shouldBe 1
  }
  "p = 5" in {
    mirrorReflection(5, 0) shouldBe 0
    mirrorReflection(5, 1) shouldBe 1
    mirrorReflection(5, 2) shouldBe 0
    mirrorReflection(5, 3) shouldBe 1
    mirrorReflection(5, 4) shouldBe 0
    mirrorReflection(5, 5) shouldBe 1
  }
  "p = 6" in {
    mirrorReflection(6, 0) shouldBe 0
    mirrorReflection(6, 1) shouldBe 2
    mirrorReflection(6, 2) shouldBe 1
    mirrorReflection(6, 3) shouldBe 2
    mirrorReflection(6, 4) shouldBe 0
    mirrorReflection(6, 5) shouldBe 2
    mirrorReflection(6, 6) shouldBe 1
  }
  "p = 7" in {
    mirrorReflection(7, 0) shouldBe 0
    mirrorReflection(7, 1) shouldBe 1
    mirrorReflection(7, 2) shouldBe 0
    mirrorReflection(7, 3) shouldBe 1
    mirrorReflection(7, 4) shouldBe 0
    mirrorReflection(7, 5) shouldBe 1
    mirrorReflection(7, 6) shouldBe 0
    mirrorReflection(7, 7) shouldBe 1
  }
  "p = 8" in {
    mirrorReflection(8, 0) shouldBe 0
    mirrorReflection(8, 1) shouldBe 2
    mirrorReflection(8, 2) shouldBe 2
    mirrorReflection(8, 3) shouldBe 2
    mirrorReflection(8, 4) shouldBe 2
    mirrorReflection(8, 5) shouldBe 2
    mirrorReflection(8, 6) shouldBe 2
    mirrorReflection(8, 7) shouldBe 2
    mirrorReflection(8, 8) shouldBe 1
  }
  "p = 9" in {
    mirrorReflection(9, 0) shouldBe 0
    mirrorReflection(9, 1) shouldBe 1
    mirrorReflection(9, 2) shouldBe 0
    mirrorReflection(9, 3) shouldBe 1
    mirrorReflection(9, 4) shouldBe 0
    mirrorReflection(9, 5) shouldBe 1
    mirrorReflection(9, 6) shouldBe 0
    mirrorReflection(9, 7) shouldBe 1
    mirrorReflection(9, 8) shouldBe 0
    mirrorReflection(9, 9) shouldBe 1
  }
  "p = 10" in {
    mirrorReflection(10, 0) shouldBe 0
    mirrorReflection(10, 1) shouldBe 2
    mirrorReflection(10, 2) shouldBe 1
    mirrorReflection(10, 3) shouldBe 2
    mirrorReflection(10, 4) shouldBe 0
    mirrorReflection(10, 5) shouldBe 2
    mirrorReflection(10, 6) shouldBe 1
    mirrorReflection(10, 7) shouldBe 2
    mirrorReflection(10, 8) shouldBe 0
    mirrorReflection(10, 9) shouldBe 2
    mirrorReflection(10, 10) shouldBe 1
  }

}
