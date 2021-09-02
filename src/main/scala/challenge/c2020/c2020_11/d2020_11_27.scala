package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/567/week-4-november-22nd-november-28th/3545/]]
 */
//noinspection DuplicatedCode
class d2020_11_27 extends AnyWordSpec with Matchers {

  /**
   * === Partition Equal Subset Sum ===
   *
   * Given a '''non-empty''' array `nums` containing '''only positive integers''',
   * find if the array can be partitioned into two subsets such that the sum of elements in both subsets is equal.
   *
   * '''Constraints:'''
   *  - `1 <= nums.length <= 200`
   *  - `1 <= nums[i] <= 100`
   */
  object Solution {
    def canPartition(nums: Array[Int]): Boolean = {
      val total = nums.sum
      if (total % 2 != 0) false
      else {
        val target = total / 2
        // 0 = undefined; 1 = false; 2 = true
        val memo = Array.ofDim[Short](nums.length + 1, target + 1)

        def dfs(i: Int, sum: Int): Boolean = {
          if (sum == 0) true
          else if (i == 0 || sum < 0) false
          else if (memo(i)(sum) != 0) memo(i)(sum) == 2
          else {
            val result = dfs(i - 1, sum - nums(i - 1)) || dfs(i - 1, sum)
            memo(i)(sum) = if (result) 2 else 1
            result
          }
        }

        dfs(nums.length, target)
      }
    }
  }

  object SolutionJavaLangBoolean {
    def canPartition(nums: Array[Int]): Boolean = {
      val total = nums.sum
      if (total % 2 != 0) false
      else {
        val target = total / 2
        val memo = Array.ofDim[java.lang.Boolean](nums.length + 1, target + 1)

        def dfs(i: Int, sum: Int): Boolean = {
          if (sum == 0) true
          else if (i == 0 || sum < 0) false
          else if (memo(i)(sum) != null) memo(i)(sum)
          else {
            val result = dfs(i - 1, sum - nums(i - 1)) || dfs(i - 1, sum)
            memo(i)(sum) = result
            result
          }
        }

        dfs(nums.length, target)
      }
    }
  }

  object SolutionMyImperative {
    def canPartition(nums: Array[Int]): Boolean = {
      val sum = nums.sum
      if (sum % 2 != 0) false
      else {
        val target = sum / 2
        val currentSums = collection.mutable.Set(0)
        for (i <- nums) {
          currentSums ++= currentSums.map(_ + i)
          if (currentSums.contains(target)) return true
        }
        currentSums.contains(target)
      }
    }
  }
  object SolutionMyImmutable {
    def canPartition(nums: Array[Int]): Boolean = {
      val sum = nums.sum
      if (sum % 2 != 0) false
      else {
        val target = sum / 2

        @scala.annotation.tailrec
        def loop(i: Int, currentSums: Set[Int]): Boolean =
          if (currentSums.contains(target)) true
          else if (i >= nums.length) false
          else loop(i + 1, currentSums ++ currentSums.map(_ + nums(i)).filter(_ <= target))

        loop(0, Set(0))
      }
    }
  }

  import Solution.canPartition

  "Example 1: (nums = [1,5,11,5]) -> true" in {
    canPartition(Array(1, 5, 11, 5)) shouldBe true
    //Explanation: The array can be partitioned as [1, 5, 5] and [11].
  }
  "Example 2: (nums = [1,2,3,5]) -> false" in {
    canPartition(Array(1, 2, 3, 5)) shouldBe false
    //Explanation: The array cannot be partitioned into equal sum subsets.
  }

  "(nums = 200 bad elements) -> false" in {
    val nums = Array.fill(200)(100)
    nums(199) = 98
    canPartition(nums) shouldBe false
  }
  "(nums = 200 good elements) -> true" in {
    val nums = (1 to 100).flatMap(i => Array(i, i)).toArray
    canPartition(nums) shouldBe true
  }
}
