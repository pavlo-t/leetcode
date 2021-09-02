package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3600/]]
 */
//noinspection DuplicatedCode
class c2021_01_11 extends AnyWordSpec with Matchers {
  /**
   * === Merge Sorted Array ===
   */
  object Solution {
    def merge(nums1: Array[Int], m: Int, nums2: Array[Int], n: Int): Unit = {
      @scala.annotation.tailrec
      def rec(i: Int, j: Int): Unit = {
        if (i < 0 && j < 0) ()
        else if (i < 0 || (j >= 0 && nums1(i) < nums2(j))) {
          nums1(i + j + 1) = nums2(j)
          rec(i, j - 1)
        } else {
          nums1(i + j + 1) = nums1(i)
          rec(i - 1, j)
        }
      }
      rec(m - 1, n - 1)
    }
  }

  object SolutionTmpArray {
    def merge(nums1: Array[Int], m: Int, nums2: Array[Int], n: Int): Unit = {
      val tmp = Array.fill(m + n)(0)
      @scala.annotation.tailrec
      def rec(i: Int, j: Int): Unit = {
        if (i == m && j == n) ()
        else if (i == m || (j < n && nums1(i) >= nums2(j))) {
          tmp(i + j) = nums2(j)
          rec(i, j + 1)
        } else {
          tmp(i + j) = nums1(i)
          rec(i + 1, j)
        }
      }
      rec(0, 0)

      for (i <- tmp.indices)
        nums1(i) = tmp(i)
    }
  }
  object SolutionTmpSeq {
    def merge(nums1: Array[Int], m: Int, nums2: Array[Int], n: Int): Unit = {
      @scala.annotation.tailrec
      def rec(i: Int, j: Int, rsf: Seq[Int]): Seq[Int] = {
        if (i == m && j == n) rsf
        else if (i == m || (j < n && nums1(i) >= nums2(j)))
          rec(i, j + 1, rsf :+ nums2(j))
        else
          rec(i + 1, j, rsf :+ nums1(i))
      }
      val tmp = rec(0, 0, Seq())

      for (i <- tmp.indices)
        nums1(i) = tmp(i)
    }
  }

  import Solution.merge

  "Example 1: (nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3) -> [1,2,2,3,5,6]" in {
    val nums1 = Array(1, 2, 3, 0, 0, 0)
    val m = 3
    val nums2 = Array(2, 5, 6)
    val n = 3

    merge(nums1, m, nums2, n)

    nums1 shouldBe Array(1, 2, 2, 3, 5, 6)
  }
  "Example 2: (nums1 = [1], m = 1, nums2 = [], n = 0) -> [1]" in {
    val nums1 = Array(1)
    merge(nums1, 1, Array(), 0)
    nums1 shouldBe Array(1)
  }
  "(nums1 = [0], m = 0, nums2 = [1], n = 1) -> [1]" in {
    val nums1 = Array(0)
    merge(nums1, 0, Array(1), 1)
    nums1 shouldBe Array(1)
  }
}
