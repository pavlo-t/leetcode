package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/565/week-2-november-8th-november-14th/3524/]]
 */
//noinspection DuplicatedCode
class d2020_11_08 extends AnyWordSpec with Matchers {
  /**
   * <h3>Binary Tree Tilt</h3>
   *
   * Given the `root` of a binary tree, return <em>the sum of every tree node's <b>tilt</b></em>.
   *
   * The <b>tilt</b> of a tree node is the <b>absolute difference</b> between the sum of all left subtree node
   * <b>values</b> and all right subtree node <b>values</b>.
   * If a node does not have a left child, then the sum of the left subtree node <b>values</b> is treated as `0`.
   * The rule is similar if there the node does not have a right child.
   *
   * <b>Constraints:</b><ul>
   * <li> The number of nodes in the tree is in the range `[0, 10_000]`.
   * <li> `-1000 <= Node.val <= 1000`
   * </ul>
   */
  object Solution {
    def findTilt(root: TreeNode): Int = {
      var tilt = 0
      def sum(t: TreeNode): Int =
        if (t == null) 0
        else {
          val sumL = sum(t.left)
          val sumR = sum(t.right)
          tilt += math.abs(sumL - sumR)
          t.value + sumL + sumR
        }
      sum(root)
      tilt
    }
  }

  object SolutionMy {
    def sum(t: TreeNode): Int =
      if (t == null) 0
      else t.value + sum(t.left) + sum(t.right)

    def findTilt(root: TreeNode): Int =
      if (root == null) 0
      else math.abs(sum(root.left) - sum(root.right)) + findTilt(root.left) + findTilt(root.right)
  }

  class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
    var value: Int = _value
    var left: TreeNode = _left
    var right: TreeNode = _right

    override def toString: String = s"$value,$left,$right"
  }
  private def N(v: Int, l: TreeNode = null, r: TreeNode = null): TreeNode = new TreeNode(v, l, r)

  import Solution.findTilt

  "Example 1: (root = [1,2,3]) -> 1" in {
    val root = N(1, N(2), N(3))
    findTilt(root) shouldBe 1
    // Explanation:
    // Tilt of node 2 : |0-0| = 0 (no children)
    // Tilt of node 3 : |0-0| = 0 (no children)
    // Tile of node 1 : |2-3| = 1 (left subtree is just left child, so sum is 2; right subtree is just right child, so sum is 3)
    // Sum of every tilt : 0 + 0 + 1 = 1
  }
  "Example 2: (root = [4,2,9,3,5,null,7]) -> 15" in {
    val root = N(4, N(2, N(3), N(5)), N(9, null, N(7)))
    findTilt(root) shouldBe 15
    // Explanation:
    // Tilt of node 3 : |0-0| = 0 (no children)
    // Tilt of node 5 : |0-0| = 0 (no children)
    // Tilt of node 7 : |0-0| = 0 (no children)
    // Tilt of node 2 : |3-5| = 2 (left subtree is just left child, so sum is 3; right subtree is just right child, so sum is 5)
    // Tilt of node 9 : |0-7| = 7 (no left child, so sum is 0; right subtree is just right child, so sum is 7)
    // Tilt of node 4 : |(3+5+2)-(9+7)| = |10-16| = 6 (left subtree values are 3, 5, and 2, which sums to 10;
    //   right subtree values are 9 and 7, which sums to 16)
    // Sum of every tilt : 0 + 0 + 0 + 2 + 7 + 6 = 15
  }
  "Example 3: (root = [21,7,14,1,1,2,2,3,3]) -> 9" in {
    val root = N(21, N(7, N(1, N(3), N(3)), N(1)), N(14, N(2), N(2)))
    findTilt(root) shouldBe 9
  }

  "Test 19: (root = [1,2,null,3,4]) -> 10" in {
    val root = N(1, N(2, N(3), N(4)), null)
    findTilt(root) shouldBe 10
  }

  "(root = []) -> 0" in {
    findTilt(null) shouldBe 0
  }
  "(root = [1]) -> 0" in {
    findTilt(N(1)) shouldBe 0
  }
  "(root = [1,2,2]) -> 0" in {
    findTilt(N(1, N(2), N(2))) shouldBe 0
  }
  "(root = [1 repeat 10_000]) -> 0" in {
    def buildTree(i: Int): TreeNode =
      if (i == 0) null
      else N(1, buildTree(i / 2), buildTree(i / 2))

    def size(n: TreeNode): Int =
      if (n == null) 0
      else 1 + size(n.left) + size(n.right)

    val root = buildTree(10_000)
    size(root) shouldBe 16_383
    findTilt(root) shouldBe 0
  }
  "(root = [1 repeat 10_000]) -> >= 0" in {
    def buildTree(i: Int): TreeNode =
      if (i == 0) null
      else N(util.Random.nextInt(2001) - 1000, buildTree(i / 2), buildTree(i / 2))

    val root = buildTree(10_000)

    findTilt(root) should be >= 0
  }

}
