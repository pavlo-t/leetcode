package c2020_10.w4

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


//noinspection DuplicatedCode
class d2020_10_23 extends AnyWordSpec with Matchers {

  /**
   * <h3>132 Pattern</h3>
   *
   * Given an array of `n` integers `nums`,
   * a <b>132 pattern</b> is a subsequence of three integers `nums[i]`, `nums[j]` and `nums[k]`
   * such that `i < j < k` and `nums[i] < nums[k] < nums[j]`.
   *
   * Return <em>`true` if there is a <b>132 pattern</b> in `nums`, otherwise, return `false`</em>.
   *
   * <b>Follow up</b>: The `O(n^2)` is trivial, could you come up with the `O(n log n)` or the `O(n)` solution?
   *
   * <b>Constraints:</b><ul>
   * <li> `n == nums.length`
   * <li> `1 <= n <= 10_000`
   * <li> `-10^9 <= nums[i] <= 10^9`
   * </ul>
   */
  object Solution {
    def find132pattern(nums: Array[Int]): Boolean = {
      if (nums.length < 3) false
      else {
        false
      }
    }
  }
  object SolutionStack {
    def find132pattern(nums: Array[Int]): Boolean = {
      if (nums.length < 3) false
      else {
        val minNums = Array.ofDim[Int](nums.length)
        minNums(0) = nums(0)
        for (i <- 1 until nums.length) minNums(i) = math.min(nums(i), minNums(i - 1))

        val stack = collection.mutable.Stack[Int]()

        for (j <- (nums.length - 1) to 1 by -1) {
          if (nums(j) > minNums(j)) {
            while (stack.nonEmpty && stack.top <= minNums(j))
              stack.pop()
            if (stack.nonEmpty && stack.top < nums(j))
              return true
            stack.push(nums(j))
          }
        }

        false
      }
    }
  }
  object SolutionMy {
    def find132pattern(nums: Array[Int]): Boolean = {
      if (nums.length < 3) false
      else {
        val minNums = nums.clone()
        for (i <- 1 until minNums.length) {
          if (minNums(i) > minNums(i - 1)) minNums(i) = minNums(i - 1)
        }

        val kSet = collection.mutable.Set[Int]()

        for (j <- (nums.length - 1) to 1 by -1) {
          val i = j - 1
          if (kSet.exists(kVal => minNums(i) < kVal && kVal < nums(j)))
            return true
          else kSet.addOne(nums(j))
        }

        false
      }
    }
  }
  object SolutionQuadratic {
    def find132pattern(nums: Array[Int]): Boolean = {
      if (nums.length < 3) false
      else {
        var minI = Int.MaxValue
        for (j <- 0 until (nums.length - 1)) {
          minI = math.min(minI, nums(j))
          for (k <- (j + 1) until nums.length) {
            if (minI < nums(k) && nums(k) < nums(j)) return true
          }
        }
        false
      }
    }
  }
  object SolutionCubic {
    def find132pattern(nums: Array[Int]): Boolean = {
      if (nums.length < 3) false
      else {
        for (i <- 0 until (nums.length - 2)) {
          for (j <- (i + 1) until (nums.length - 1)) {
            for (k <- (j + 1) until nums.length) {
              if (nums(i) < nums(k) && nums(k) < nums(j)) return true
            }
          }
        }
        false
      }
    }
  }

  import Solution.find132pattern

  "Example 1: ([1,2,3,4]) -> false" in {
    val nums = Array(1, 2, 3, 4)
    find132pattern(nums) shouldBe false
    // Explanation: There is no 132 pattern in the sequence.
  }
  "Example 2: ([3,1,4,2]) -> true" in {
    val nums = Array(3, 1, 4, 2)
    find132pattern(nums) shouldBe true
    // Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
  }
  "Example 3: ([-1,3,2,0]) -> true" in {
    val nums = Array(-1, 3, 2, 0)
    find132pattern(nums) shouldBe true
    // Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].
  }

  "Test 89: ([1,2,3,4,-4,-3,-5,-1]) -> false" in {
    val nums = Array(1, 2, 3, 4, -4, -3, -5, -1)
    find132pattern(nums) shouldBe false
  }
  "Test 91: ([3,5,0,3,4]) -> true" in {
    val nums = Array(3, 5, 0, 3, 4)
    find132pattern(nums) shouldBe true
  }

  "([1]) -> false" in {
    val nums = Array(1)
    find132pattern(nums) shouldBe false
  }
  "([1,2]) -> false" in {
    val nums = Array(1, 2)
    find132pattern(nums) shouldBe false
  }
  "([1,2,3]) -> false" in {
    val nums = Array(1, 2, 3)
    find132pattern(nums) shouldBe false
  }
  "([1,3,2]) -> true" in {
    val nums = Array(1, 3, 2)
    find132pattern(nums) shouldBe true
  }
  "([1,3,2,4]) -> true" in {
    val nums = Array(1, 3, 2, 4)
    find132pattern(nums) shouldBe true
  }
  "([4,1,3,2]) -> true" in {
    val nums = Array(4, 1, 3, 2)
    find132pattern(nums) shouldBe true
  }
  "(1_000 els) -> false" in {
    val nums = Array.fill(1_000)(0)
    find132pattern(nums) shouldBe false
  }
  "(10_000 els) -> false" in {
    val nums = Array.fill(10_000)(0)
    find132pattern(nums) shouldBe false
  }
  "(10_000 els) -> true" in {
    val length = 10_000
    val nums = Array.ofDim[Int](length)
    for (i <- nums.indices) nums(i) = i
    nums(0) = -2
    nums(length - 1) = -1

    find132pattern(nums) shouldBe true
  }
}
