package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/566/week-3-november-15th-november-21st/3533/]]
 */
//noinspection NameBooleanParameters,DuplicatedCode
class d2020_11_16 extends AnyWordSpec with Matchers {

  /**
   * === Longest Mountain in Array ===
   *
   * Let's call any (contiguous) subarray B (of A) a ''mountain'' if the following properties hold:
   *  - `B.length >= 3`
   *  - There exists some `0 < i < B.length - 1` such that
   *    `B[0] < B[1] < ... B[i-1] < B[i] > B[i+1] > ... > B[B.length - 1]`
   *
   * (Note that B could be any subarray of A, including the entire array A.)
   *
   * Given an array `A` of integers, return the length of the longest ''mountain''.
   *
   * Return `0` if there is no mountain.
   *
   * '''Note:'''
   *  - `0 <= A.length <= 10_000`
   *  - `0 <= A[i] <= 10_000`
   *
   * '''Follow up:'''
   *  - Can you solve it using only one pass?
   *  - Can you solve it in `O(1)` space?
   */
  object Solution {
    def longestMountain(A: Array[Int]): Int = {
      @scala.annotation.tailrec
      def loop(s: Int, e: Int, stp: Boolean, rsf: Int): Int = {
        val ne = e + 1
        if (ne < A.length)
          if (!stp)
            if (A(e) < A(ne)) loop(s, ne, stp, rsf)
            else if (A(e) > A(ne) && s < e) loop(s, ne, true, rsf)
            else loop(ne, ne, stp, rsf)
          else {
            if (A(e) > A(ne)) loop(s, ne, stp, rsf)
            else loop(e, e, false, rsf max (ne - s))
          }
        else if (stp) rsf max (ne - s)
        else rsf
      }

      loop(0, 0, false, 0)
    }
  }

  /** [[https://leetcode.com/problems/longest-mountain-in-array/solution/]] */
  object SolutionTwoPointer {
    def longestMountain(A: Array[Int]): Int = {
      var ans, base = 0

      while (base < A.length) {
        var end = base

        // if base is a left-boundary
        if (end + 1 < A.length && A(end) < A(end + 1)) {
          // set end to the peak of this potential mountain
          while (end + 1 < A.length && A(end) < A(end + 1))
            end += 1

          // if end is really a peak..
          if (end + 1 < A.length && A(end) > A(end + 1)) {
            // set 'end' to right-boundary of mountain
            while (end + 1 < A.length && A(end) > A(end + 1))
              end += 1
            // record candidate answer
            ans = ans max (end - base + 1)
          }
        }

        base = end max (base + 1)
      }

      ans
    }
  }

  object SolutionFoldLeft {
    def longestMountain(A: Array[Int]): Int = A.sliding(2).foldLeft(0, 0, false) {
      case ((maxSize, 0, false), e) if e.head < e.last    => (maxSize, 2, false)
      case ((maxSize, size, false), e) if e.head < e.last => (maxSize, size + 1, false)
      case ((maxSize, _, _), e) if e.head == e.last       => (maxSize, 0, false)
      case ((maxSize, 0, false), e) if e.head > e.last    => (maxSize, 0, false)
      case ((maxSize, size, _), e) if e.head > e.last     => (maxSize max (size + 1), size + 1, true)
      case ((maxSize, _, true), e) if e.head < e.last     => (maxSize, 2, false)
    }._1
  }
  object SolutionMyRecursion {
    def longestMountain(A: Array[Int]): Int = {
      @scala.annotation.tailrec
      def loop(i: Int, size: Int, seenThePeak: Boolean, rsf: Int): Int =
        if (i >= A.length)
          if (seenThePeak) rsf max size
          else rsf
        else {
          val a = A(i - 1)
          val b = A(i)
          if (size == 0)
            if (a < b)
              loop(i + 1, 2, false, rsf)
            else
              loop(i + 1, 0, false, rsf)
          else if (seenThePeak)
            if (a > b)
              loop(i + 1, size + 1, true, rsf)
            else if (a == b)
              loop(i + 1, 0, false, rsf max size)
            else
              loop(i + 1, 2, false, rsf max size)
          else {
            if (a > b)
              loop(i + 1, size + 1, true, rsf)
            else if (a == b)
              loop(i + 1, 0, false, rsf)
            else
              loop(i + 1, size + 1, false, rsf)
          }
        }

      loop(1, 0, false, 0)
    }
  }
  object SolutionMyImperative {
    def longestMountain(A: Array[Int]): Int =
      if (A.length < 3) 0
      else {
        var result = 0
        var mountStart: Option[Int] = None
        var seenThePeak = false

        for (i <- 1 until A.length) mountStart match {
          case None     =>
            if (A(i) > A(i - 1))
              mountStart = Some(i - 1)
          case Some(ms) =>
            if (seenThePeak) {
              if (A(i) == A(i - 1)) {
                mountStart = None
                seenThePeak = false
                result = result max (i - ms)
              } else if (A(i) > A(i - 1)) {
                mountStart = Some(i - 1)
                seenThePeak = false
                result = result max (i - ms)
              }
            } else {
              if (A(i) == A(i - 1)) {
                mountStart = None
              } else if (A(i) < A(i - 1))
                seenThePeak = true
            }
        }
        if (seenThePeak) result = result max (A.length - mountStart.get)

        result
      }
  }

  import Solution.longestMountain

  "Example 1: (Input: [2,1,4,7,3,2,5]) -> 5" in {
    longestMountain(Array(2, 1, 4, 7, 3, 2, 5)) shouldBe 5
    //Explanation: The largest mountain is [1,4,7,3,2] which has length 5.
  }
  "Example 2: (Input: [2,2,2]) -> 0" in {
    longestMountain(Array(2, 2, 2)) shouldBe 0
    //Explanation: There is no mountain.
  }

  "Test 57: (Input: [2,3,3,2,0,2]) -> 0" in {
    longestMountain(Array(2, 3, 3, 2, 0, 2)) shouldBe 0
  }

  "(Input: []) -> 0" in {
    longestMountain(Array()) shouldBe 0
  }
  "(Input: [1]) -> 0" in {
    longestMountain(Array(1)) shouldBe 0
  }
  "(Input: [1,2]) -> 0" in {
    longestMountain(Array(1, 2)) shouldBe 0
  }
  "(Input: [2,1]) -> 0" in {
    longestMountain(Array(2, 1)) shouldBe 0
  }
  "(Input: [2,2,1]) -> 0" in {
    longestMountain(Array(2, 2, 1)) shouldBe 0
  }
  "(Input: [1,2,2]) -> 0" in {
    longestMountain(Array(1, 2, 2)) shouldBe 0
  }
  "(Input: [1,2,1]) -> 3" in {
    longestMountain(Array(1, 2, 1)) shouldBe 3
  }
  "(Input: [1,2,2,1]) -> 0" in {
    longestMountain(Array(1, 2, 2, 1)) shouldBe 0
  }
  "(Input: [1,2,1,1]) -> 3" in {
    longestMountain(Array(1, 2, 1, 1)) shouldBe 3
  }
  "(Input: [1,1,2,1]) -> 3" in {
    longestMountain(Array(1, 1, 2, 1)) shouldBe 3
  }

  "(Input: [1 repeat 10_000]) -> 0" in {
    longestMountain(Array.fill(10_000)(1)) shouldBe 0
  }
  "(Input: [mount of size 10_000]) -> 10_000" in {
    val A = (1 to 10_000).toArray
    A(9999) = 1
    longestMountain(A) shouldBe 10_000
  }
  "(Input: [1,2,1,2.. until 10_000]) -> 3" in {
    val A = (1 to 10_000).map(_ % 2).toArray
    A(9999) = 1
    longestMountain(A) shouldBe 3
  }
}
