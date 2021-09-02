package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/567/week-4-november-22nd-november-28th/3541/]]
 */
//noinspection NameBooleanParameters,DuplicatedCode
class d2020_11_23 extends AnyWordSpec with Matchers {

  /**
   * === House Robber III ===
   *
   * The thief has found himself a new place for his thievery again.
   * There is only one entrance to this area, called the "root".
   * Besides the root, each house has one and only one parent house.
   * After a tour, the smart thief realized that "all houses in this place forms a binary tree".
   * It will automatically contact the police if two directly-linked houses were broken into on the same night.
   *
   * Determine the maximum amount of money the thief can rob tonight without alerting the police.
   */
  object Solution {
    def rob(root: TreeNode): Int = {
      def loop(n: TreeNode): (Int, Int) =
        if (n == null) (0, 0)
        else {
          val l = loop(n.left)
          val r = loop(n.right)

          ((n.value + l._2 + r._2) max (l._1 + r._1), l._1 + r._1)
        }

      loop(root)._1
    }
  }

  object SolutionWithCache {
    def rob(root: TreeNode): Int = {
      val cache = collection.mutable.Map[(TreeNode, Boolean), Int]()
      def loop(n: TreeNode, canRob: Boolean): Int = {
        val k = (n, canRob)

        if (n == null) 0
        else if (cache.contains(k)) cache(k)
        else {
          val rl = loop(n.left, true)
          val nl = loop(n.left, false)
          val rr = loop(n.right, true)
          val nr = loop(n.right, false)

          val result = ((if (canRob) n.value else 0) + nl + nr) max (rl + rr) max (rl + nr) max (nl + rr)
          cache(k) = result

          result
        }
      }

      loop(root, true)
    }
  }
  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String = s"$value,${if (left == null) "" else left},${if (right == null) "" else right}"
  }


  import Solution.rob

  private def N(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: ([3,2,3,null,3,null,1]) -> 7" in {
    val root = N(3, N(2, r = N(3)), N(3, r = N(1)))
    rob(root) shouldBe 7
    //     3
    //    / \
    //   2   3
    //    \   \
    //     3   1
    //
    //Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.
  }
  "Example 2: ([3,4,5,1,3,null,1]) -> 9" in {
    val root = N(3, N(4, N(1), N(3)), N(5, r = N(1)))
    rob(root) shouldBe 9
    //     3
    //    / \
    //   4   5
    //  / \   \
    // 1   3   1
    //
    //Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.
  }

  "([]) -> 0" in {
    rob(null) shouldBe 0
  }
  "([3]) -> 3" in {
    rob(N(3)) shouldBe 3
  }
  "([3,1,1]) -> 3" in {
    rob(N(3, N(1), N(1))) shouldBe 3
  }
  "([3,2,2]) -> 4" in {
    rob(N(3, N(2), N(2))) shouldBe 4
  }
  "([1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]) -> 97" in {
    val root = N(1, N(2, N(4, N(8), N(9)), N(5, N(10), N(11))), N(3, N(6, N(12), N(13)), N(7, N(14), N(15))))
    rob(root) shouldBe 97
  }
}
