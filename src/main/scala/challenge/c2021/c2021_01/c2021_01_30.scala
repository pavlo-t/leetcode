package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/583/week-5-january-29th-january-31st/3622/]]
 */
class c2021_01_30 extends AnyWordSpec with Matchers {
  /**
   * === Minimize Deviation in Array ===
   *
   * You are given an array `nums` of `n` positive integers.
   *
   * You can perform two types of operations on any element of the array any number of times:
   *  - If the element is '''even''', '''divide''' it by `2`.
   *    - For example, if the array is `[1,2,3,4]`,
   *      then you can do this operation on the last element,
   *      and the array will be `[1,2,3,2]`.
   *  - If the element is '''odd''', '''multiply''' it by `2`.
   *    - For example, if the array is `[1,2,3,4]`,
   *      then you can do this operation on the first element,
   *      and the array will be `[2,2,3,4]`.
   *
   * The '''deviation''' of the array is the '''maximum difference''' between any two elements in the array.
   *
   * Return ''the '''minimum deviation''' the array can have after performing some number of operations''.
   *
   * '''Constraints:'''
   *  - `2 <= nums.length <= 100_000`
   *  - `1 <= nums[i] <= 1_000_000_000`
   */
  object Solution {
    def minimumDeviation(nums: Array[Int]): Int = {
      val evens = collection.mutable.PriorityQueue[Int]()
      var min = Int.MaxValue
      for (n <- nums)
        if (n % 2 == 0) {
          evens.enqueue(n)
          min = min min n
        } else {
          evens.enqueue(n * 2)
          min = min min (n * 2)
        }

      var result = Int.MaxValue
      while (evens.nonEmpty) {
        val n = evens.dequeue()
        result = result min (n - min)
        if (n % 2 == 0) {
          evens.enqueue(n / 2)
          min = min min (n / 2)
        } else {
          return result
        }
      }
      result
    }
  }

  import Solution.minimumDeviation

  "Example 1: (nums = [1,2,3,4]) -> 1" in {
    minimumDeviation(Array(1, 2, 3, 4)) shouldBe 1
    //Explanation: You can transform the array to [1,2,3,2], then to [2,2,3,2], then the deviation will be 3 - 2 = 1.
  }
  "Example 2: (nums = [4,1,5,20,3]) -> 3" in {
    minimumDeviation(Array(4, 1, 5, 20, 3)) shouldBe 3
    //Explanation: You can transform the array after two operations to [4,2,5,5,3], then the deviation will be 5 - 2 = 3.
  }
  "Example 3: (nums = [2,10,8]) -> 3" in {
    minimumDeviation(Array(2, 10, 8)) shouldBe 3
  }

  "(nums = Array.fill(100_000)(1)) -> 0" in {
    minimumDeviation(Array.fill(100_000)(1)) shouldBe 0
  }

  "(nums = 1 to 100_000) -> 99_997" in {
    minimumDeviation((1 to 100_000).toArray) shouldBe 99_997
  }

}
