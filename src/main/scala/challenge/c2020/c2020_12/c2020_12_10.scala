package challenge.c2020.c2020_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/december-leetcoding-challenge/570/week-2-december-8th-december-14th/3561/]]
 */
class c2020_12_10 extends AnyWordSpec with Matchers {

  /**
   * === Valid Mountain Array ===
   *
   * Given an array of integers `arr`, return `true` ''if and only if it is a valid mountain array''.
   *
   * Recall that `arr` is a mountain array if and only if:
   *  - `arr.length >= 3`
   *  - There exists some `i` with `0 < i < arr.length - 1` such that:
   *    - `arr[0] < arr[1] < ... < arr[i - 1] < arr[i]`
   *    - `arr[i] > arr[i + 1] > ... > arr[arr.length - 1]`
   *
   * '''Constraints:'''
   *  - `1 <= arr.length <= 10_000`
   *  - `0 <= arr[i] <= 10_000`
   */
  object Solution {
    def validMountainArray(arr: Array[Int]): Boolean =
      if (arr.length < 3 || arr(0) >= arr(1)) false
      else {
        arr.tail.foldLeft((arr.head, false)) {
          case ((l, _), c) if c == l    => return false
          case ((l, false), c) if l < c => (c, false)
          case ((_, false), c)          => (c, true)
          case ((l, true), c) if l < c  => return false
          case ((_, true), c)           => (c, true)
        }._2
      }
  }

  import Solution.validMountainArray

  "Example 1: (arr = [2,1]) -> false" in {
    validMountainArray(Array(2, 1)) shouldBe false
  }
  "Example 2: (arr = [3,5,5]) -> false" in {
    validMountainArray(Array(3, 5, 5)) shouldBe false
  }
  "Example 3: (arr = [0,3,2,1]) -> true" in {
    validMountainArray(Array(0, 3, 2, 1)) shouldBe true
  }

  "(arr = [1..10_000]) -> false" in {
    val arr = (1 to 10000).toArray
    validMountainArray(arr) shouldBe false
  }
  "(arr = [10_000..1]) -> false" in {
    val arr = (10000 to 1 by -1).toArray
    validMountainArray(arr) shouldBe false
  }
  "(arr = [1..9999,9998]) -> true" in {
    val arr = (1 to 10000).toArray
    arr(9999) = 9998
    validMountainArray(arr) shouldBe true
  }

}
