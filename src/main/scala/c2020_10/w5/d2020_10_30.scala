package c2020_10.w5

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_10_30 extends AnyWordSpec with Matchers {

  /**
   * <h3>Number of Longest Increasing Subsequence</h3>
   *
   * Given an integer array `nums`, return <em>the number of longest increasing subsequences</em>.
   *
   * <b>Constraints:</b><ul>
   * <li> `0 <= nums.length <= 2_000`
   * <li> `-1_000_000 <= nums[i] <= 1_000_000`
   */
  object SolutionStub {
    def findNumberOfLIS(nums: Array[Int]): Int = {
      ???
    }
  }

  /**
   * <h3>Approach 1: Dynamic Programming</h3>
   *
   * <b>Intuition and Algorithm</b>
   *
   * Suppose for sequences ending at `nums[i]`,
   * we knew the length `length[i]` of the longest sequence,
   * and the number `count[i]` of such sequences with that length.
   *
   * For every `i < j` with `A[i] < A[j]`, we might append `A[j]` to a longest subsequence ending at `A[i]`.
   * It means that we have demonstrated `count[i]` subsequences of length `length[i] + 1`.
   *
   * Now, if those sequences are longer than `length[j]`, then we know we have `count[i]` sequences of this length.
   * If these sequences are equal in length to `length[j]`,
   * then we know that there are now `count[i]` additional sequences to be counted of that length
   * (ie. `count[j] += count[i]`).
   *
   * <b>Complexity Analysis</b><ul>
   * <li> Time Complexity: `O(N^2)` where `N` is the length of nums. There are two for-loops and the work inside is O(1)O(1).
   * <li> Space Complexity: `O(N)`, the space used by `lengths` and `counts`.
   * </ul>
   */
  object Solution {
    def findNumberOfLIS(nums: Array[Int]): Int = {
      if (nums.length <= 1) nums.length
      else {
        val lengths = Array.fill(nums.length)(0)
        val counts = Array.fill(nums.length)(1)

        for (j <- nums.indices) {
          for (i <- 0 until j) {
            if (nums(i) < nums(j)) {
              if (lengths(i) >= lengths(j)) {
                lengths(j) = lengths(i) + 1
                counts(j) = counts(i)
              } else if (lengths(i) + 1 == lengths(j))
                counts(j) += counts(i)
            }
          }
        }

        val longest = lengths.max
        lengths.zip(counts).filter(_._1 == longest).map(_._2).sum
      }
    }
  }

  object SolutionDPMy {
    def findNumberOfLIS(nums: Array[Int]): Int =
      if (nums.length <= 1) nums.length
      else {
        val lcs = Array.fill(nums.length)(0 -> 1)

        for (i <- nums.indices) {
          for (j <- 0 until i) {
            if (nums(i) > nums(j)) {
              if (lcs(i)._1 <= lcs(j)._1)
                lcs(i) = (lcs(j)._1 + 1, lcs(j)._2)
              else if (lcs(i)._1 == lcs(j)._1 + 1)
                lcs(i) = (lcs(i)._1, lcs(i)._2 + lcs(j)._2)
            }
          }
        }

        val longest = lcs.maxBy(_._1)._1
        lcs.filter(_._1 == longest).map(_._2).sum
      }
  }

  object SolutionBruteForceWithTodo {
    import scala.annotation.tailrec

    def findNumberOfLIS(nums: Array[Int]): Int = {
      if (nums.length == 0) 0
      else {
        def nexts(c: Seq[Int]): Seq[Seq[Int]] = {
          val l = c.last
          var i = l + 1
          val rs = collection.mutable.ListBuffer[Seq[Int]]()
          while (i < nums.length) {
            if (nums(i) > nums(l))
              rs.addOne(c.appended(i))
            i += 1
          }
          rs.toSeq
        }

        @tailrec
        def loop(todo: Seq[Seq[Int]], rsf: Seq[Seq[Int]]): Seq[Seq[Int]] = {
          if (todo.isEmpty) rsf
          else {
            val ns = nexts(todo.head)
            if (ns.isEmpty) loop(todo.tail, rsf :+ todo.head)
            else loop(todo.tail ++ ns, rsf)
          }
        }

        val iss = loop(nums.indices.map(Seq(_)), Seq())
        val longest = iss.maxBy(_.size).size

        iss.count(_.size == longest)
      }
    }
  }


  import Solution.findNumberOfLIS

  "Example 1: ([1,3,5,4,7]) -> 2" in {
    val nums = Array(1, 3, 5, 4, 7)
    findNumberOfLIS(nums) shouldBe 2
    // Explanation: The two longest increasing subsequences are [1, 3, 4, 7] and [1, 3, 5, 7].
  }
  "Example 2: ([2,2,2,2,2]) -> 5" in {
    val nums = Array(2, 2, 2, 2, 2)
    findNumberOfLIS(nums) shouldBe 5
    // Explanation: The length of longest continuous increasing subsequence is 1,
    //   and there are 5 subsequences' length is 1, so output 5.
  }

  "([]) -> 0" in {
    findNumberOfLIS(Array()) shouldBe 0
  }
  "([1]) -> 1" in {
    findNumberOfLIS(Array(1)) shouldBe 1
  }
  "([1,2]) -> 1" in {
    findNumberOfLIS(Array(1, 2)) shouldBe 1
  }
  "([1,1]) -> 2" in {
    findNumberOfLIS(Array(1, 1)) shouldBe 2
  }
  "([2,1]) -> 2" in {
    findNumberOfLIS(Array(2, 1)) shouldBe 2
  }

  "([<2_000 elements>]) -> 1" in {
    val nums = Array.ofDim[Int](2_000)
    for (i <- nums.indices) nums(i) = i

    findNumberOfLIS(nums) shouldBe 1
  }
  "([<2_000 elements>]) -> 2_000" in {
    val nums = Array.ofDim[Int](2_000)
    for (i <- nums.indices) nums(i) = 1

    findNumberOfLIS(nums) shouldBe 2_000
  }
  "([<2_000 elements>]) -> 1_586_054_160" in {
    val nums = Array.ofDim[Int](2_000)
    for (i <- nums.indices) nums(i) = i % 5

    findNumberOfLIS(nums) shouldBe 1_586_054_160
  }
}
