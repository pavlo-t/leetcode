package challenge.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3599/]]
 */
//noinspection DuplicatedCode
class c2021_01_10 extends AnyWordSpec with Matchers {
  /**
   * === Create Sorted Array through Instructions ===
   *
   * Given an integer array `instructions`, you are asked to create a sorted array from the elements in `instructions`.
   * You start with an empty container `nums`.
   * For each element from '''left to right''' in `instructions`, insert it into `nums`.
   * The '''cost''' of each insertion is the '''minimum''' of the following:
   *  - The number of elements currently in `nums` that are '''strictly less than''' `instructions[i]`.
   *  - The number of elements currently in `nums` that are '''strictly greater than''' `instructions[i]`.
   *
   * For example, if inserting element `3` into `nums = [1,2,3,5]`, the cost of insertion is `min(2, 1)`
   * (elements `1` and `2` are less than `3`, element `5` is greater than `3`) and `nums` will become `[1,2,3,3,5]`.
   *
   * Return ''the '''total cost''' to insert all elements from'' `instructions` ''into'' `nums`.
   * Since the answer may be large, return it '''modulo''' `10**9 + 7`.
   *
   * '''Constraints:'''
   *  - `1 <= instructions.length <= 100_000`
   *  - `1 <= instructions[i] <= 100_000`
   */
  object Solution {
    val Modulo = 1_000_000_007
    def createSortedArray(instructions: Array[Int]): Int = {
      def lsb(i: Int): Int = i & -i

      val size = 100_000
      val bit = Array.ofDim[Int](size + 1)

      @scala.annotation.tailrec
      def sum(i: Int, rsf: Int = 0): Int =
        if (i > 0) sum(i - lsb(i), rsf + bit(i))
        else rsf

      @scala.annotation.tailrec
      def add(i: Int): Unit =
        if (i <= size) {
          bit(i) += 1
          add(i + lsb(i))
        }

      instructions
        .indices
        .fold(0) { (acc, i) =>
          val v = instructions(i)
          val newRes = (acc + (sum(v - 1) min (i - sum(v)))) % Modulo
          add(v)
          newRes
        }
    }
  }

  object SolutionBruteForce {
    val M = 1_000_000_007
    def createSortedArray(instructions: Array[Int]): Int = {
      @scala.annotation.tailrec
      def rec(i: Int, nums: Seq[Int], rsf: Int): Int = {
        if (i == instructions.length) rsf
        else {
          val (lt, gt) = nums.foldLeft(0, 0) {
            case ((lt, gt), n) if n < instructions(i) => (lt + 1, gt)
            case ((lt, gt), n) if n > instructions(i) => (lt, gt + 1)
            case (acc, _)                             => acc
          }
          rec(i + 1, nums :+ instructions(i), (rsf + (lt min gt)) % M)
        }
      }
      rec(0, Seq(), 0)
    }
  }

  import Solution.createSortedArray

  "Example 1: (instructions = [1,5,6,2]) -> 1" in {
    createSortedArray(Array(1, 5, 6, 2)) shouldBe 1
    //Explanation: Begin with nums = [].
    //Insert 1 with cost min(0, 0) = 0, now nums = [1].
    //Insert 5 with cost min(1, 0) = 0, now nums = [1,5].
    //Insert 6 with cost min(2, 0) = 0, now nums = [1,5,6].
    //Insert 2 with cost min(1, 2) = 1, now nums = [1,2,5,6].
    //The total cost is 0 + 0 + 0 + 1 = 1.
  }
  "Example 2: (instructions = [1,2,3,6,5,4]) -> 3" in {
    createSortedArray(Array(1, 2, 3, 6, 5, 4)) shouldBe 3
    //Explanation: Begin with nums = [].
    //Insert 1 with cost min(0, 0) = 0, now nums = [1].
    //Insert 2 with cost min(1, 0) = 0, now nums = [1,2].
    //Insert 3 with cost min(2, 0) = 0, now nums = [1,2,3].
    //Insert 6 with cost min(3, 0) = 0, now nums = [1,2,3,6].
    //Insert 5 with cost min(3, 1) = 1, now nums = [1,2,3,5,6].
    //Insert 4 with cost min(3, 2) = 2, now nums = [1,2,3,4,5,6].
    //The total cost is 0 + 0 + 0 + 0 + 1 + 2 = 3.
  }
  "Example 3: (instructions = [1,3,3,3,2,4,2,1,2]) -> 4" in {
    createSortedArray(Array(1, 3, 3, 3, 2, 4, 2, 1, 2)) shouldBe 4
    //Output: 4
    //Explanation: Begin with nums = [].
    //Insert 1 with cost min(0, 0) = 0, now nums = [1].
    //Insert 3 with cost min(1, 0) = 0, now nums = [1,3].
    //Insert 3 with cost min(1, 0) = 0, now nums = [1,3,3].
    //Insert 3 with cost min(1, 0) = 0, now nums = [1,3,3,3].
    //Insert 2 with cost min(1, 3) = 1, now nums = [1,2,3,3,3].
    //Insert 4 with cost min(5, 0) = 0, now nums = [1,2,3,3,3,4].
    //Insert 2 with cost min(1, 4) = 1, now nums = [1,2,2,3,3,3,4].
    //Insert 1 with cost min(0, 6) = 0, now nums = [1,1,2,2,3,3,3,4].
    //Insert 2 with cost min(2, 4) = 2, now nums = [1,1,2,2,2,3,3,3,4].
    //The total cost is 0 + 0 + 0 + 0 + 1 + 0 + 1 + 0 + 2 = 4.
  }

  "(instructions = 1 to 100_000) -> 0" in {
    createSortedArray((1 to 100_000).toArray) shouldBe 0
  }
  "(instructions = 100_000 to 1) -> 0" in {
    createSortedArray((100_000 to 1 by -1).toArray) shouldBe 0
  }

}
