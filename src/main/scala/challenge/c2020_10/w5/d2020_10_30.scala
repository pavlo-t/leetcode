package challenge.c2020_10.w5

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
   * </ul>
   *
   * Solution: [[https://leetcode.com/problems/number-of-longest-increasing-subsequence/solution/]]
   */
  object SolutionStub {
    def findNumberOfLIS(nums: Array[Int]): Int = {
      nums.length
    }
  }

  /**
   * <h3>Approach 2: Segment Tree</h3>
   *
   * <b>Intuition</b>
   *
   * Suppose we knew for each length `L`, the number of sequences with length `L` ending in `x`.
   * Then when considering the next element of `nums`,
   * updating our knowledge hinges on knowing the number of sequences with length `L-1` ending in `y < x`.
   * This type of query over an interval is a natural fit for using some sort of tree.
   *
   * We could try using Fenwick trees, but we would have to store `N` of them, which naively might be `O(N^2)` space.
   * Here, we focus on an implementation of a Segment Tree.
   *
   * <b>Algorithm</b>
   *
   * Implementing Segment Trees is discussed in more detail
   * [[https://leetcode.com/articles/a-recursive-approach-to-segment-trees-range-sum-queries-lazy-propagation/ here]].
   * In this approach, we will attempt a variant of segment tree that doesn't use lazy propagation.
   *
   * First, let us call the "value" of an interval, the longest length of an increasing subsequence,
   * and the number of such subsequences in that interval.
   *
   * Each node knows about the interval of `nums` values it is considering `[node.range_left, node.range_right]`,
   * and it knows about `node.val`, which contains information on the value of interval.
   * Nodes also have `node.left` and `node.right` children that represents the left and right half of the interval
   * `node` considers. These child nodes are created on demand as appropriate.
   *
   * Now, `query(node, key)` will tell us the value of the interval specified by `node`,
   * except we'll exclude anything above `key`.
   * When key is outside the interval specified by `node`, we return the answer.
   * Otherwise, we'll divide the interval into two and query both intervals, then `merge` the result.
   *
   * What does `merge(v1, v2)` do?
   * Suppose two nodes specify adjacent intervals, and have corresponding values `v1 = node1.val`, `v2 = node2.val`.
   * What should the aggregate value, `v = merge(v1, v2)` be?
   * If there are longer subsequences in one node, then `v` will just be that.
   * If both nodes have longest subsequences of equal length, then we should count subsequences in both nodes.
   * Note that we did not have to consider cases where larger subsequences were made,
   * since these were handled by `insert`.
   *
   * What does `insert(node, key, val)` do?
   * We repeatedly insert the `key` into the correct half of the interval `that` node specifies (possibly a point),
   * and after such insertion this node's value could change, so we merge the values together again.
   *
   * Finally, in our main algorithm, for each `num in nums` we query for the value `length`, `count`
   * of the interval below `num`, and we know it will lead to `count` additional sequences of length `length + 1`.
   * We then update our tree with that knowledge.
   *
   * <b>Complexity Analysis</b><ul>
   * <li> Time Complexity: `O(N log N)` where `N` is the length of nums.
   * In our main loop, we do `O(log N)` work to `query` and `insert`.
   * <li> Space Complexity: `O(N)`, the space used by the segment tree.
   * </ul>
   *
   * [[https://leetcode.com/problems/number-of-longest-increasing-subsequence/solution/]]
   */
  object Solution {
    def findNumberOfLIS(nums: Array[Int]): Int =
      if (nums.length == 0) 0
      else {
        var min, max = nums(0)
        for (n <- nums) {
          min = n min min
          max = n max max
        }

        val root = Node(min, max)
        for (n <- nums) {
          val v: Value = query(root, n - 1)
          insert(root, n, Value(v.length + 1, v.count))
        }

        root.value.count
      }

    def query(node: Node, key: Int): Value =
      if (node.rangeRight <= key) node.value
      else if (node.rangeLeft > key) Value(0, 1)
      else merge(query(node.left, key), query(node.right, key))

    def merge(v1: Value, v2: Value): Value =
      if (v1.length == v2.length)
        if (v1.length == 0) Value(0, 1)
        else Value(v1.length, v1.count + v2.count)
      else if (v1.length > v2.length) v1
      else v2

    def insert(node: Node, key: Int, value: Value): Unit =
      if (node.rangeLeft == node.rangeRight)
        node.value = merge(value, node.value)
      else {
        if (key <= node.rangeMid) insert(node.left, key, value)
        else insert(node.right, key, value)
        node.value = merge(node.left.value, node.right.value)
      }

    case class Node(rangeLeft: Int, rangeRight: Int) {
      var value: Value = Value(0, 1)

      private var _left: Node = _
      private var _right: Node = _

      def rangeMid: Int = rangeLeft + (rangeRight - rangeLeft) / 2
      def left: Node = {
        if (_left == null) _left = Node(rangeLeft, rangeMid)
        _left
      }
      def right: Node = {
        if (_right == null) _right = Node(rangeMid + 1, rangeRight)
        _right
      }
    }

    case class Value(length: Int, count: Int)
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
   * <li> Time Complexity: `O(N^2)` where `N` is the length of nums. There are two loops and the work inside is `O(1)`.
   * <li> Space Complexity: `O(N)`, the space used by `lengths` and `counts`.
   * </ul>
   *
   * [[https://leetcode.com/problems/number-of-longest-increasing-subsequence/solution/]]
   */
  object SolutionDP {
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

  "([2_000 elements]) -> 1" in {
    val nums = Array.ofDim[Int](2_000)
    for (i <- nums.indices) nums(i) = i

    findNumberOfLIS(nums) shouldBe 1
  }
  "([2_000 elements]) -> 2_000" in {
    val nums = Array.ofDim[Int](2_000)
    for (i <- nums.indices) nums(i) = 1

    findNumberOfLIS(nums) shouldBe 2_000
  }
  "([2_000 elements]) -> 1_586_054_160" in {
    val nums = Array.ofDim[Int](2_000)
    for (i <- nums.indices) nums(i) = i % 5

    findNumberOfLIS(nums) shouldBe 1_586_054_160
  }
}
