package challenge.c2022.c2022_07

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * =510. Inorder Successor in BST II=
 *
 * [[https://leetcode.com/problems/inorder-successor-in-bst-ii]]
 *
 * Given a `node` in a binary search tree, return ''the in-order successor of that node in the BST''.
 * If that node has no in-order successor, return `null`.
 *
 * The successor of a `node` is the node with the smallest key greater than `node.val`.
 *
 * You will have direct access to the node but not to the root of the tree.
 * Each node will have a reference to its parent node.
 * Below is the definition for `Node`:
 *
 * {{{
 * class Node {
 *   public int val;
 *   public Node left;
 *   public Node right;
 *   public Node parent;
 * }
 * }}}
 *
 * '''Constraints:'''
 *  - The number of nodes in the tree is in the range `[1, 10_000]`.
 *  - `-100_000 <= Node.val <= 100_000`
 *  - All Nodes will have unique values.
 *
 * '''Follow up:''' Could you solve it without looking up any of the node's values?
 *
 */
class c2022_07_w2 extends AnyWordSpec with Matchers {

  object Solution {
    def inorderSuccessor(node: Node): Node =
      if (node.right != null)
        node.right.min
      else
        node.findParent

    private implicit class NodeOps(val n: Node) {
      @scala.annotation.tailrec
      @inline final def min: Node =
        if (n.left == null) n
        else n.left.min

      @scala.annotation.tailrec
      @inline final def findParent: Node =
        if (n.parent == null || n.parent.left == n) n.parent
        else n.parent.findParent
    }
  }

  //noinspection ConvertNullInitializerToUnderscore
  class Node(var _value: Int) {
    var value: Int = _value
    var left: Node = null
    var right: Node = null
    var parent: Node = null

    override def toString: String = s"Node(${value.toString})"
  }

  private def n(v: Int, l: Node = null, r: Node = null, p: Node = null): Node = {
    val node = new Node(v)
    if (l != null) {
      node.left = l
      l.parent = node
    }
    if (r != null) {
      node.right = r
      r.parent = node
    }
    node.parent = p
    node
  }

  "tree = [2,1,3], node = 1" in {
    //  2
    // 1 3
    val tree = n(2, n(1), n(3))
    Solution.inorderSuccessor(tree.left) shouldBe tree
    // Explanation: 1's in-order successor node is 2.
    // Note that both the node and the return value is of Node type.
  }
  "tree = [2,1,3], node = 2" in {
    //  2
    // 1 3
    val tree = n(2, n(1), n(3))
    Solution.inorderSuccessor(tree) shouldBe tree.right
  }
  "tree = [5,3,6,2,4,null,null,1], node = 6" in {
    //     5
    //   3  6
    //  2 4
    // 1
    val tree = n(5, n(3, n(2, n(1)), n(4)), n(6))
    Solution.inorderSuccessor(tree.right) shouldBe null
    // Explanation: There is no in-order successor of the current node, so the answer is null.
  }
  "tree = [5,3,6,2,4,null,null,1], node = 3" in {
    //     5
    //   3  6
    //  2 4
    // 1
    val tree = n(5, n(3, n(2, n(1)), n(4)), n(6))
    Solution.inorderSuccessor(tree.left) shouldBe tree.left.right
  }
  "tree = [6,3,7,2,5,null,null,1,null,4], node = 3" in {
    //       6
    //   3     7
    //  2  5
    // 1  4
    val tree = n(6, n(3, n(2, n(1)), n(5, n(4))), n(7))
    Solution.inorderSuccessor(tree.left) shouldBe tree.left.right.left
  }

  "test 23" in {
    //           15
    //      6         18
    //   3    7     17  20
    // 2   4   13
    //        9
    val tree = n(15, n(6, n(3, n(2), n(4)), n(7, r = n(13, n(9)))), n(18, n(17), n(20)))
    Solution.inorderSuccessor(tree.left.right.right) shouldBe tree
  }

}
