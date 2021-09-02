package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3602/]]
 */
//noinspection DuplicatedCode
class c2021_01_13 extends AnyWordSpec with Matchers {
  /**
   * === Boats to Save People ===
   *
   * The `i`-th person has weight `people[i]`, and each boat can carry a maximum weight of `limit`.
   *
   * Each boat carries at most 2 people at the same time,
   * provided the sum of the weight of those people is at most `limit`.
   *
   * Return the minimum number of boats to carry every given person.
   * (It is guaranteed each person can be carried by a boat.)
   *
   * '''Note:'''
   *  - `1 <= people.length <= 50000`
   *  - `1 <= people[i] <= limit <= 30000`
   */
  object Solution {
    def numRescueBoats(people: Array[Int], limit: Int): Int = {
      val P = people.sorted
      @scala.annotation.tailrec
      def rec(l: Int, r: Int, rsf: Int): Int = {
        if (l > r) rsf
        else if (P(l) + P(r) <= limit) rec(l + 1, r - 1, rsf + 1)
        else rec(l, r - 1, rsf + 1)
      }
      rec(0, P.length - 1, 0)
    }
  }

  import Solution.numRescueBoats

  "Example 1: (people = [1,2], limit = 3) -> 1" in {
    val people = Array(1, 2)
    val limit = 3
    val e = 1
    numRescueBoats(people, limit) shouldBe e
    //Explanation: 1 boat (1, 2)
  }
  "Example 2: (people = [3,2,2,1], limit = 3) -> 3" in {
    val people = Array(3, 2, 2, 1)
    val limit = 3
    val e = 3
    numRescueBoats(people, limit) shouldBe e
    //Explanation: 3 boats (1, 2), (2) and (3)
  }
  "Example 3: (people = [3,5,3,4], limit = 5) -> 4" in {
    val people = Array(3, 5, 3, 4)
    val limit = 5
    val e = 4
    numRescueBoats(people, limit) shouldBe e
    //Explanation: 4 boats (3), (3), (4), (5)
  }

  "(people = [1], limit = 5) -> 1" in {
    numRescueBoats(Array(1), 5) shouldBe 1
  }
  "(people = [5], limit = 5) -> 1" in {
    numRescueBoats(Array(5), 5) shouldBe 1
  }
  "(people = [1,5,9,10], limit = 5) -> 1" in {
    numRescueBoats(Array(5), 5) shouldBe 1
  }

  "(people = 1 to 50000, limit = 50000) -> 25001" in {
    numRescueBoats((1 to 50000).toArray, 50000) shouldBe 25001
  }

}
