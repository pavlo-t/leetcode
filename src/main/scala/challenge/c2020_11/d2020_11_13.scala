package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_11_13 extends AnyWordSpec with Matchers {

  /**
   * === Populating Next Right Pointers in Each Node ===
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
   *
   * Initially, all next pointers are set to `NULL`.
   *
   * '''Follow up:'''
   *   - You may only use constant extra space.
   *   - Recursive approach is fine, you may assume implicit stack space does not count as extra space for this problem.
   *
   * '''Constraints:'''
   *   - The number of nodes in the given tree is less than `4096`.
   *   - `-1000 <= node.val <= 1000`
   */
  object Solution {
    def connect(root: Node): Node =
      if (root == null) null
      else if (root.left == null) new Node(root.value)
      else {
        val result = new Node(root.value)
        def deepCopy(from: Node, to: Node): Unit =
          if (from != null && from.left != null) {
            to.left = new Node(from.left.value)
            to.right = new Node(from.right.value)
            deepCopy(from.left, to.left)
            deepCopy(from.right, to.right)
          }
        def _connect(n: Node): Unit =
          if (n != null && n.left != null) {
            n.left.next = n.right
            if (n.next != null) n.right.next = n.next.left
            _connect(n.left)
            _connect(n.right)
          }
        deepCopy(root, result)
        _connect(result)

        result
      }
  }

  object SolutionRecursionMutation {
    def connect(root: Node): Node =
      if (root == null || root.left == null) root
      else {
        root.left.next = root.right
        if (root.next != null) root.right.next = root.next.left
        connect(root.left)
        connect(root.right)
        root
      }
  }

  object SolutionWithMap {
    import collection.mutable

    def connect(root: Node): Node = {
      val nodeLevels = mutable.Map[Int, mutable.ArrayBuffer[Node]]()

      def add(l: Int, n: Node): Unit =
        if (nodeLevels.contains(l)) nodeLevels(l) += n
        else nodeLevels(l) = mutable.ArrayBuffer(n)

      def fillNodeLevels(n: Node, l: Int = 0): Unit =
        if (n != null) {
          add(l, n)
          fillNodeLevels(n.left, l + 1)
          fillNodeLevels(n.right, l + 1)
        }
      fillNodeLevels(root)

      nodeLevels.foreach { case (_, ns) =>
        for (i <- 1 until ns.length) {
          ns(i - 1).next = ns(i)
        }
      }

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
    //override def toString: String = value.toString
  }

  import Solution.connect

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

    connect(root).toString shouldBe expected.toString
    // Explanation:
    //   The serialized output is in level order as connected by the next pointers,
    //   with '#' signifying the end of each level.
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

    val result = connect(root)

    result.left.next.value shouldBe 2048
  }

}
