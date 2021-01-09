package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/579/week-1-january-1st-january-7th/3594/]]
 */
//noinspection DuplicatedCode
class c2021_01_06 extends AnyWordSpec with Matchers {
  /**
   * === Kth Missing Positive Number ===
   *
   * Given an array `arr` of positive integers sorted in a '''strictly increasing order''', and an integer `k`.
   *
   * ''Find the'' `k`th ''positive integer that is missing from this array''.
   *
   * '''Constraints:'''
   *  - `1 <= arr.length <= 1000`
   *  - `1 <= arr[i] <= 1000`
   *  - `1 <= k <= 1000`
   *  - `arr[i] < arr[j]` for `1 <= i < j <= arr.length`
   */
  object Solution {
    def findKthPositive(arr: Array[Int], k: Int): Int = {
      var i = 0
      while (i < arr.length && (arr(i) - i - 1) < k)
        i += 1
      i + k
    }
  }

  object SolutionBruteForce2 {
    def findKthPositive(arr: Array[Int], k: Int): Int = {
      @scala.annotation.tailrec
      def loop(i: Int): Int =
        if (i == arr.length || arr(i) - i - 1 >= k) i + k
        else loop(i + 1)

      loop(0)
    }
  }

  object SolutionBinarySearch {
    def findKthPositive(arr: Array[Int], k: Int): Int = {
      @scala.annotation.tailrec
      def bs(l: Int, r: Int): Int = {
        if (l > r) k + l
        else {
          val mid = l + (r - l) / 2
          if (arr(mid) - mid - 1 < k) bs(mid + 1, r)
          else bs(l, mid - 1)
        }
      }

      bs(0, arr.length - 1)
    }
  }

  object SolutionBruteForce {
    def findKthPositive(arr: Array[Int], k: Int): Int = {
      if (k < arr.head) k
      else {
        @scala.annotation.tailrec
        def rec(i: Int, k: Int): Int = {
          if (i == arr.length - 1) arr.last + k
          else {
            val m = arr(i + 1) - arr(i) - 1
            if (k <= m) arr(i) + k
            else rec(i + 1, k - m)
          }
        }
        rec(0, k - arr.head + 1)
      }
    }
  }

  object SolutionMyBruteForce {
    def findKthPositive(arr: Array[Int], k: Int): Int = {
      @scala.annotation.tailrec
      def rec(i: Int, n: Int, k: Int): Int = {
        if (i == arr.length || arr(i) != n)
          if (k == 1) n
          else rec(i, n + 1, k - 1)
        else rec(i + 1, n + 1, k)
      }
      rec(0, 1, k)
    }
  }

  import Solution.findKthPositive

  "Example 1: (arr = [2,3,4,7,11], k = 5) -> 9" in {
    findKthPositive(Array(2, 3, 4, 7, 11), 5) shouldBe 9
    //Explanation:
    // The missing positive integers are [1,5,6,8,9,10,12,13,...].
    // The 5th missing positive integer is 9.
  }
  "Example 2: (arr = [1,2,3,4], k = 2) -> 6" in {
    findKthPositive(Array(1, 2, 3, 4), 2) shouldBe 6
    //Explanation:
    // The missing positive integers are [5,6,7,...].
    // The 2nd missing positive integer is 6.
  }

  "(arr = [2], k = 1) -> 1" in {
    findKthPositive(Array(2), 1) shouldBe 1
  }
  "(arr = [2,3], k = 1) -> 1" in {
    findKthPositive(Array(2, 3), 1) shouldBe 1
  }
  "(arr = [1001], k = 1000) -> 1000" in {
    findKthPositive(Array(1001), 1000) shouldBe 1000
  }
  "(arr = [1 to 1000], k = 1000) -> 2000" in {
    findKthPositive((1 to 1000).toArray, 1000) shouldBe 2000
  }
  "(arr = [1001 to 2000], k = 1000) -> 1000" in {
    findKthPositive((1001 to 2000).toArray, 1000) shouldBe 1000
  }
  "(arr = [1000 to 2000], k = 1000) -> 2001" in {
    findKthPositive((1000 to 2000).toArray, 1000) shouldBe 2001
  }

}
