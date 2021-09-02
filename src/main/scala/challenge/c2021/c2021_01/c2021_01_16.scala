package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/581/week-3-january-15th-january-21st/3606/]]
 */
class c2021_01_16 extends AnyWordSpec with Matchers {
  /**
   * === Kth Largest Element in an Array ===
   * Find the '''k'''th largest element in an unsorted array.
   * Note that it is the kth largest element in the sorted order, not the kth distinct element.
   *
   * '''Note:'''
   *  - You may assume `k` is always valid, `1 ≤ k ≤ array's length`.
   */
  object Solution {
    def findKthLargest(nums: Array[Int], k: Int): Int = {
      val h = collection.mutable.PriorityQueue()(Ordering[Int].reverse)
      for (n <- nums) {
        h.enqueue(n)
        if (h.size > k) h.dequeue()
      }
      h.dequeue()
    }
  }

  object SolutionBuiltinSort {
    def findKthLargest(nums: Array[Int], k: Int): Int =
      nums.sorted(Ordering[Int].reverse)(k - 1)
  }

  import Solution.findKthLargest

  "Example 1: ([3,2,1,5,6,4] and k = 2) -> 5" in {
    findKthLargest(Array(3, 2, 1, 5, 6, 4), 2) shouldBe 5
  }
  "Example 2: ([3,2,3,1,2,4,5,5,6] and k = 4) -> 4" in {
    findKthLargest(Array(3, 2, 3, 1, 2, 4, 5, 5, 6), 4) shouldBe 4
  }

}
