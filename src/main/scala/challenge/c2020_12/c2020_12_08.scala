package challenge.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/570/week-2-december-8th-december-14th/3559/]]
 */
//noinspection DuplicatedCode
class c2020_12_08 extends AnyWordSpec with Matchers {

  /**
   * === Pairs of Songs With Total Durations Divisible by 60 ===
   *
   * You are given a list of songs where the i^th^ song has a duration of `time[i]` seconds.
   *
   * Return ''the number of pairs of songs for which their total duration in seconds is divisible by'' `60`.
   * Formally, we want the number of indices `i`, `j` such that `i < j` with `(time[i] + time[j]) % 60 == 0`.
   *
   * '''Constraints:'''
   *  - `1 <= time.length <= 60_000`
   *  - `1 <= time[i] <= 500`
   */
  object Solution {
    def numPairsDivisibleBy60(time: Array[Int]): Int = {
      var result = 0
      val remainders = Array.fill(60)(0)

      for (t <- time) {
        val r = t % 60
        if (r == 0)
          result += remainders(0)
        else
          result += remainders(60 - r)

        remainders(r) += 1
      }

      result
    }
  }

  object SolutionBruteForce {
    def numPairsDivisibleBy60(time: Array[Int]): Int = {
      var result = 0

      for (i <- 0 until (time.length - 1); j <- (i + 1) until time.length) {
        if ((time(i) + time(j)) % 60 == 0)
          result += 1
      }

      result
    }
  }

  import Solution.numPairsDivisibleBy60

  "Example 1: (time = [30,20,150,100,40]) -> 3" in {
    numPairsDivisibleBy60(Array(30, 20, 150, 100, 40)) shouldBe 3
    //Explanation: Three pairs have a total duration divisible by 60:
    // (time[0] = 30, time[2] = 150): total duration 180
    // (time[1] = 20, time[3] = 100): total duration 120
    // (time[1] = 20, time[4] = 40): total duration 60
  }
  "Example 2: (time = [60,60,60]) -> 3" in {
    numPairsDivisibleBy60(Array(60, 60, 60)) shouldBe 3
    //Explanation: All three pairs have a total duration of 120, which is divisible by 60.
  }

  "(time = [10]) -> 0" in {
    numPairsDivisibleBy60(Array(10)) shouldBe 0
  }

  "(time = [60000 elements]) -> 0" in {
    numPairsDivisibleBy60(Array.fill(60000)(10)) shouldBe 0
  }
  "(time = [60000 elements]) -> 1,799,970,000" in {
    numPairsDivisibleBy60(Array.fill(60000)(60)) shouldBe 1_799_970_000
  }
  "(time = [1 to 60000]) -> 29,999,000" in {
    numPairsDivisibleBy60((1 to 60000).toArray) shouldBe 29_999_000
  }

}
