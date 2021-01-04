package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/december-leetcoding-challenge/572/week-4-december-22nd-december-28th/3583/]]
 */
//noinspection DuplicatedCode
class c2020_12_28 extends AnyWordSpec with Matchers {
  /**
   * === Reach a Number ===
   *
   * You are standing at position `0` on an infinite number line.
   * There is a goal at position `target`.
   *
   * On each move, you can either go left or right.
   * During the ''n''-th move (starting from 1), you take ''n'' steps.
   *
   * Return the minimum number of steps required to reach the destination.
   *
   * '''Note:'''
   *  - `target` will be a non-zero integer in the range `[-10^9, 10^9]`.
   */
  object Solution {
    // https://leetcode.com/problems/reach-a-number/solution/
    def reachNumber(target: Int): Int = {
      @scala.annotation.tailrec
      def rec(target: Int, step: Int): Int = {
        if (target <= 0 && target % 2 == 0) step - 1
        else rec(target - step, step + 1)
      }
      rec(target.abs, 1)
    }
  }

  object SolutionMy {
    def reachNumber(target: Int): Int = {
      val T = target.abs
      @scala.annotation.tailrec
      def rec(sum: Int, step: Int): Int = {
        if (sum == T || (sum > T && (sum - T) % 2 == 0)) step - 1
        else rec(sum + step, step + 1)
      }
      rec(0, 1)
    }
  }

  object SolutionBruteForce {
    def reachNumber(target: Int): Int = {
      @scala.annotation.tailrec
      def rec(states: Set[Int], step: Int): Int = {
        if (states.contains(target)) step - 1
        else rec(states.flatMap(i => Set(i + step, i - step)), step + 1)
      }
      rec(Set(0), 1)
    }
  }

  import Solution.reachNumber

  "Example 1: (target = 3) -> 2" in {
    reachNumber(3) shouldBe 2
    //Explanation:
    // On the first move we step from 0 to 1.
    // On the second step we step from 1 to 3.
  }
  "Example 2: (target = 2) -> 3" in {
    reachNumber(2) shouldBe 3
    //Explanation:
    // On the first move we step from 0 to 1.
    // On the second move we step  from 1 to -1.
    // On the third move we step from -1 to 2.
  }

  "(target = -1) -> 1" in (reachNumber(-1) shouldBe 1) // 0->-1
  "(target = -2) -> 3" in (reachNumber(-2) shouldBe 3) // 0->-1-> 1->-2
  "(target = -3) -> 2" in (reachNumber(-3) shouldBe 2) // 0->-1->-3

  "(target =  0) -> 0" in (reachNumber(0) shouldBe 0)
  "(target =  1) -> 1" in (reachNumber(1) shouldBe 1) //  1
  "(target =  2) -> 3" in (reachNumber(2) shouldBe 3) //  1-2+3
  "(target =  3) -> 2" in (reachNumber(3) shouldBe 2) //  1+2
  "(target =  4) -> 3" in (reachNumber(4) shouldBe 3) // -1+2+3
  "(target =  5) -> 5" in (reachNumber(5) shouldBe 5) //  1-2-3+4+5
  "(target =  6) -> 3" in (reachNumber(6) shouldBe 3) //  1+2+3
  "(target =  7) -> 5" in (reachNumber(7) shouldBe 5) //  1+2-3+4+5
  "(target =  8) -> 4" in (reachNumber(8) shouldBe 4) // -1+2+3+4
  "(target =  9) -> 5" in (reachNumber(9) shouldBe 5) //  1+2-3+4+5
  "(target = 10) -> 4" in (reachNumber(10) shouldBe 4) // 1+2+3+4
  "(target = 11) -> 5" in (reachNumber(11) shouldBe 5) // 1-2+3+4+5
  "(target = 12) -> 7" in (reachNumber(12) shouldBe 7) // 1-2+3+4+5-6+7 == 1+2-3+4-5+6+7
  "(target = 13) -> 5" in (reachNumber(13) shouldBe 5) //-1+2+3+4+5

  "(target = 100000) -> 447" in (reachNumber(100000) shouldBe 447)
  "(target =-100000) -> 447" in (reachNumber(-100000) shouldBe 447)
  "(target = 1,000,000,000) -> 44723" in (reachNumber(1_000_000_000) shouldBe 44723)
  "(target =-1,000,000,000) -> 44723" in (reachNumber(-1_000_000_000) shouldBe 44723)

}
