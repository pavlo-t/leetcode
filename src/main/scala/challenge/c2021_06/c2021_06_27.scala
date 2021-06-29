package challenge.c2021_06

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/june-leetcoding-challenge-2021/606/week-4-june-22nd-june-28th/3793/]]
 */
class c2021_06_27 extends AnyWordSpec with Matchers {
  /**
   * == Candy ==
   *
   * There are `n` children standing in a line.
   * Each child is assigned a rating value given in the integer array `ratings`.
   *
   * You are giving candies to these children subjected to the following requirements:
   *  - Each child must have at least one candy.
   *  - Children with a higher rating get more candies than their neighbors.
   *
   * Return ''the minimum number of candies you need to have to distribute the candies to the children''.
   *
   * '''Constraints:'''
   *  - `1 <= ratings.length <= 20_000`
   *  - `0 <= ratings[i] <= 20_000`
   */
  object Solution {
    /** [[https://leetcode.com/problems/candy/discuss/135698/Simple-solution-with-one-pass-using-O(1)-space]] */
    def candy(ratings: Array[Int]): Int = {
      @scala.annotation.tailrec
      def rec(i: Int, up: Int, down: Int, peak: Int, rsf: Int): Int =
        if (i == ratings.length)
          rsf
        else if (ratings(i - 1) < ratings(i))
          rec(i + 1, up + 1, 0, up + 1, rsf + 2 + up)
        else if (ratings(i - 1) == ratings(i))
          rec(i + 1, 0, 0, 0, rsf + 1)
        else if (peak > down)
          rec(i + 1, 0, down + 1, peak, rsf + 1 + down)
        else
          rec(i + 1, 0, down + 1, peak, rsf + 2 + down)

      rec(1, 0, 0, 0, 1)
    }

    /** [[https://rustgym.com/leetcode/135]] */
    def candyLoop(ratings: Array[Int]): Int = {
      val results = Array.fill(ratings.length)(1)
      for (i <- 1 until ratings.length)
        if (ratings(i) > ratings(i - 1)) results(i) = results(i) max (results(i - 1) + 1)
      for (i <- (ratings.length - 2) to 0 by -1)
        if (ratings(i) > ratings(i + 1)) results(i) = results(i) max (results(i + 1) + 1)
      results.sum
    }
  }

  import Solution.candy

  "Example 1: (ratings = [1,0,2]) -> 5" in {
    candy(Array(1, 0, 2)) shouldBe 5
    //Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
  }
  "Example 2: (ratings = [1,2,2]) -> 4" in {
    candy(Array(1, 2, 2)) shouldBe 4
    //Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
    //The third child gets 1 candy because it satisfies the above two conditions.
  }

  "(ratings = [0..20_000]) -> 200_010_000" in {
    candy((0 until 20_000).toArray) shouldBe 200_010_000
  }

}
