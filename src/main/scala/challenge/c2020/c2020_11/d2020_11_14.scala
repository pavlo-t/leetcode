package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/565/week-2-november-8th-november-14th/3530/]]
 * [[https://medium.com/@dreamume/leetcode-458-poor-pigs-adc1bef981c1]]
 */
//noinspection DuplicatedCode
class d2020_11_14 extends AnyWordSpec with Matchers {

  /**
   * ===Poor Pigs===
   *
   * There are 1000 buckets, one and only one of them is poisonous, while the rest are filled with water.
   * They all look identical.
   * If a pig drinks the poison it will die within 15 minutes.
   * What is the minimum amount of pigs you need to figure out which bucket is poisonous within one hour?
   *
   * Answer this question, and write an algorithm for the general case.
   *
   * '''General case:'''
   *
   * If there are `n` buckets and a pig drinking poison will die within `m` minutes,
   * how many pigs (`x`) you need to figure out the '''poisonous''' bucket within `p` minutes?
   * There is exactly one bucket with poison.
   *
   * '''Note:'''
   *   1. A pig can be allowed to drink simultaneously on as many buckets as one would like, and the feeding takes no time.
   *   1. After a pig has instantly finished drinking buckets, there has to be a '''cool down time''' of ''m'' minutes.
   *   1. During this time, only observation is allowed and no feedings at all.
   *   1. Any given bucket can be sampled an infinite number of times (by an unlimited number of pigs).
   *
   * ''Hint 1:'' What if you only have one shot? Eg. 4 buckets, 15 mins to die, and 15 mins to test.
   *
   * ''Hint 2:'' How many states can we generate with x pigs and T tests?
   *
   * ''Hint 3:'' Find minimum `x` such that `(T+1)^x >= N`
   */
  object Solution {
    def poorPigs(buckets: Int, minutesToDie: Int, minutesToTest: Int): Int =
      (math.log(buckets) / math.log1p(minutesToTest / minutesToDie)).ceil.toInt
  }
  object Solution1 {
    def poorPigs(buckets: Int, minutesToDie: Int, minutesToTest: Int): Int = {
      val times = minutesToTest / minutesToDie + 1
      var result = 0
      while (math.pow(times, result) < buckets)
        result += 1
      result
    }
  }

  import Solution.poorPigs

  "(buckets=1000,minutesToDie=15,minutesToTest=60) -> 5" in {
    poorPigs(1000, 15, 60) shouldBe 5
  }
  "(buckets=1000,minutesToDie=15,minutesToTest=59) -> 5" in {
    poorPigs(1000, 15, 59) shouldBe 5
  }
  "(buckets=4,minutesToDie=15,minutesToTest=15) -> 2" in {
    poorPigs(4, 15, 15) shouldBe 2
  }
  "(buckets=1,minutesToDie=1,minutesToTest=1) -> 0" in {
    poorPigs(1, 1, 1) shouldBe 0
  }

}
