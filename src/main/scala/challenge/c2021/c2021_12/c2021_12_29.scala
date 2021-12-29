package challenge.c2021.c2021_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/problems/populating-next-right-pointers-in-each-node/]] */
//noinspection DuplicatedCode
class c2021_12_29 extends AnyWordSpec with Matchers {

  /**
   * = 116. Populating Next Right Pointers in Each Node =
   *
   * You are given a '''perfect binary tree''' where all leaves are on the same level,
   * and every parent has two children.
   * The binary tree has the following definition:
   *
   * {{{
   * struct Node {
   *   int val;
   *   Node *left;
   *   Node *right;
   *   Node *next;
   * }
   * }}}
   *
   * Populate each next pointer to point to its next right node.
   * If there is no next right node, the next pointer should be set to `NULL`.
   *
   * Initially, all next pointers are set to `NULL`.
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[0, 2**12 - 1]`.
   *  - `-1000 <= Node.val <= 1000`
   *
   * '''Follow-up:'''
   *  - You may only use constant extra space.
   *  - The recursive approach is fine. You may assume implicit stack space does not count as extra space for this problem.
   */
  object Solution {
    /** From other submissions: [[https://leetcode.com/submissions/detail/609187715/]] */
    def connect(root: Node): Node = {
      if (root != null) {
        if (root.right != null) {
          root.left.next = root.right
          if (root.next != null)
            root.right.next = root.next.left
        }

        connect(root.left)
        connect(root.right)
      }

      root
    }

    def connect_my_MutateInput(root: Node): Node = {
      @scala.annotation.tailrec
      def recMut(currLvl: Vector[Node], nextLvl: Vector[Node]): Unit = currLvl match {
        case curr +: rest =>
          curr.next = rest.headOption.orNull
          recMut(rest, nextLvl :++ Vector(curr.left, curr.right).filter(_ != null))

        case _ => if (nextLvl.nonEmpty) recMut(nextLvl, Vector.empty)
      }

      recMut(Vector(root).filter(_ != null), Vector.empty)

      root
    }
  }

  //noinspection ConvertNullInitializerToUnderscore
  class Node(var _value: Int) {
    var value: Int = _value
    var left: Node = null
    var right: Node = null
    var next: Node = null

    override def toString: String = s"{$value:$left,$right;${if (next == null) "n" else next.value}}"
  }

  private def N(v: Int, l: Node = null, r: Node = null, n: Node = null): Node = {
    val node = new Node(v)
    node.left = l
    node.right = r
    node.next = n
    node
  }

  "Example 1: (root = [1,2,3,4,5,6,7]) -> [1,#,2,3,#,4,5,6,7,#]" in {
    val root = N(1, N(2, N(4), N(5)), N(3, N(6), N(7)))
    val expected = N(1,
      N(2, N(4, n = N(5)), N(5, n = N(6)), n = N(3)),
      N(3, N(6, n = N(7)), N(7)))

    Solution.connect(root).toString shouldBe expected.toString
    //      1
    //    /   \
    //   2     3
    //  / \   / \
    // 4   5 6   7
    // Explanation: Given the above perfect binary tree,
    // your function should populate each next pointer to point to its next right node.
    // The serialized output is in level order as connected by the next pointers,
    // with '#' signifying the end of each level.
  }
  "Example 2: (root = []) -> []" in {
    Solution.connect(null) shouldBe null
  }

  private def buildTree(i: Int): Node = {
    if (i < 1) null
    else {
      val n = N(i)
      n.left = buildTree(i / 2)
      n.right = buildTree(i / 2)
      n
    }
  }

  private def size(n: Node): Int =
    if (n == null) 0
    else 1 + size(n.left) + size(n.right)

  "(root = [4096 nodes]) -> ???" in {
    val root = buildTree(4096)

    size(root) shouldBe 8191
    root.left.next shouldBe null

    val result = Solution.connect(root)

    result.left.next.value shouldBe 2048
  }
}
