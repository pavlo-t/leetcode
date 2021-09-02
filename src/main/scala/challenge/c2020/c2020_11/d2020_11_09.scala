package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_11_09 extends AnyWordSpec with Matchers {

  /**
   * <h3>Maximum Difference Between Node and Ancestor</h3>
   *
   * Given the `root` of a binary tree,
   * find the maximum value `V` for which there exist <b>different</b> nodes `A` and `B` where `V = |A.val - B.val|`
   * and `A` is an ancestor of `B`.
   *
   * A node `A` is an ancestor of `B` if either:
   * any child of `A` is equal to `B`,
   * or any child of `A` is an ancestor of `B`.
   *
   * <b>Constraints:</b><ul>
   * <li> The number of nodes in the tree is in the range `[2, 5000]`.
   * <li> `0 <= Node.val <= 100_000`
   * </ul>
   */
  object Solution {
    def maxAncestorDiff(root: TreeNode): Int = {
      def maxDiff(n: TreeNode, max: Int, min: Int): Int =
        if (n == null) max - min
        else {
          val newMax = n.value max max
          val newMin = n.value min min
          maxDiff(n.left, newMax, newMin) max maxDiff(n.right, newMax, newMin)
        }
      maxDiff(root, root.value, root.value)
    }
  }

  object SolutionMut {
    def maxAncestorDiff(root: TreeNode): Int =
      if (root == null) 0
      else {
        var result = 0
        def helper(n: TreeNode, curMax: Int, curMin: Int): Unit =
          if (n != null) {
            // update `result`
            result = result max math.abs(n.value - curMax) max math.abs(n.value - curMin)
            // update max and min
            val newMax = curMax max n.value
            val newMin = curMin min n.value
            helper(n.left, newMax, newMin)
            helper(n.right, newMax, newMin)
          }
        helper(root, root.value, root.value)
        result
      }
  }
  object SolutionMyWithCache {
    def maxAncestorDiff(root: TreeNode): Int = {
      val cache = collection.mutable.Map[TreeNode, Some[(Int, Int)]]()

      def getMaxMin(n: TreeNode): Option[(Int, Int)] = {
        if (n == null) None
        else if (cache.contains(n)) cache(n)
        else {
          val (rMin, rMax) = getMaxMin(n.right).getOrElse((n.value, n.value))
          val (lMin, lMax) = getMaxMin(n.left).getOrElse((n.value, n.value))
          cache(n) = Some((n.value min lMin min rMin, n.value max lMax max rMax))
          cache(n)
        }
      }
      def getMaxChildDiff(n: TreeNode): Int = {
        if (n == null || (n.left == null && n.right == null)) 0
        else {
          val (lMin, lMax) = getMaxMin(n.left).getOrElse((n.value, n.value))
          val (rMin, rMax) = getMaxMin(n.right).getOrElse((n.value, n.value))
          math.abs(n.value - lMin) max
            math.abs(n.value - lMax) max
            math.abs(n.value - rMin) max
            math.abs(n.value - rMax) max
            getMaxChildDiff(n.left) max
            getMaxChildDiff(n.right)
        }
      }
      getMaxChildDiff(root)
    }
  }
  object SolutionMyRec {
    def maxAncestorDiff(root: TreeNode): Int = {
      def getMaxMin(n: TreeNode): Option[(Int, Int)] = {
        if (n == null) None
        else {
          val (rMin, rMax) = getMaxMin(n.right).getOrElse((n.value, n.value))
          val (lMin, lMax) = getMaxMin(n.left).getOrElse((n.value, n.value))
          Some((n.value min lMin min rMin, n.value max lMax max rMax))
        }
      }
      def getMaxChildDiff(n: TreeNode): Int = {
        if (n == null || (n.left == null && n.right == null)) 0
        else {
          val (lMin, lMax) = getMaxMin(n.left).getOrElse((n.value, n.value))
          val (rMin, rMax) = getMaxMin(n.right).getOrElse((n.value, n.value))
          math.abs(n.value - lMin) max
            math.abs(n.value - lMax) max
            math.abs(n.value - rMin) max
            math.abs(n.value - rMax) max
            getMaxChildDiff(n.left) max
            getMaxChildDiff(n.right)
        }
      }
      getMaxChildDiff(root)
    }
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right
    override def toString: String = s"$value,$left,$right"
  }

  import Solution.maxAncestorDiff

  private def T(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  "Example 1: (root = [8,3,10,1,6,null,14,null,null,4,7,13]) -> 7" in {
    val root =
      T(8,
        T(3, T(1), T(6, T(4), T(7))),
        T(10, null, T(14, T(13))))
    maxAncestorDiff(root) shouldBe 7
    // Explanation: We have various ancestor-node differences, some of which are given below :
    //  |8 - 3| = 5
    //  |3 - 7| = 4
    //  |8 - 1| = 7
    //  |10 - 13| = 3
    //  Among all possible differences, the maximum value of 7 is obtained by |8 - 1| = 7.
  }
  "Example 2: (root = [1,null,2,null,0,3]) -> 3" in {
    val root = T(1, null, T(2, null, T(0, T(3))))
    maxAncestorDiff(root) shouldBe 3
  }

  "(root = [1 repeat 5000, wide]) -> 0" in {
    def build(i: Int): TreeNode =
      if (i == 0) null
      else T(1, build(i / 2), build(i / 2))
    def size(n: TreeNode): Int =
      if (n == null) 0
      else 1 + size(n.left) + size(n.right)

    val root = build(5000)
    size(root) shouldBe 8191
    maxAncestorDiff(root) shouldBe 0
  }
  "(root = [0, 100_000, random repeat 5000, wide]) -> 100_000" in {
    def build(i: Int): TreeNode =
      if (i == 0) null
      else T(util.Random.nextInt(100_000), build(i / 2), build(i / 2))

    val root = build(5000)
    root.value = 0
    root.left.value = 100_000

    maxAncestorDiff(root) shouldBe 100_000
  }
  "(root = [0, 100_000, random repeat 5000, one line]) -> 100_000" in {
    @scala.annotation.tailrec
    def build(i: Int, root: TreeNode, prev: TreeNode): TreeNode =
      if (i == 0) root
      else if (root == null) {
        val n = T(1)
        build(i - 1, n, n)
      } else {
        val n = T(if (i == 4999) 0 else if (i == 4001) 100_000 else util.Random.nextInt(100_001))
        prev.left = n
        build(i - 1, root, n)
      }
    def size(n: TreeNode): Int =
      if (n == null) 0
      else 1 + size(n.left) + size(n.right)

    val root = build(5000, null, null)

    size(root) shouldBe 5000
    root.left.value shouldBe 0
    maxAncestorDiff(root) shouldBe 100_000
  }
}
