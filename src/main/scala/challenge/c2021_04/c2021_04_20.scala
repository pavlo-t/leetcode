package challenge.c2021_04

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class c2021_04_20 extends AnyWordSpec with Matchers {
  /**
   * == N-ary Tree Preorder Traversal ==
   *
   * Given the `root` of an n-ary tree, return ''the preorder traversal of its nodes' values''.
   *
   * Nary-Tree input serialization is represented in their level order traversal.
   * Each group of children is separated by the null value (See examples)
   *
   * '''Constraints:'''
   *  - The number of nodes in the tree is in the range `[0, 10_000]`.
   *  - `0 <= Node.val <= 10_000`
   *  - The height of the n-ary tree is less than or equal to `1000`.
   *
   * '''Follow up:''' Recursive solution is trivial, could you do it iteratively?
   *
   * [[https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/595/week-3-april-15th-april-21st/3714/]]
   */
  object Solution {
    def preorder(root: Node): List[Int] = {
      @scala.annotation.tailrec
      def tailRec(todo: Seq[Node], rsf: List[Int]): List[Int] = todo match {
        case Nil          => rsf
        case null :: rest => tailRec(rest, rsf)
        case n :: rest    => tailRec(n.children ++ rest, n.value :: rsf)
      }
      tailRec(Seq(root), List()).reverse
    }
  }

  object SolutionIter {
    def preorder(root: Node): List[Int] = {
      val result = collection.mutable.ListBuffer[Int]()
      val stack = collection.mutable.Stack(root)
      while (stack.nonEmpty) {
        val n = stack.pop()
        if (n != null) {
          result += n.value
          stack.pushAll(n.children.reverse)
        }
      }
      result.toList
    }
  }

  object SolutionRec {
    def preorder(root: Node): List[Int] =
      if (root == null) Nil
      else root.value +: root.children.flatMap(preorder)
  }

  class Node(var _value: Int) {
    var value: Int = _value
    var children: List[Node] = List()
  }

  private def n(v: Int, ns: Node*): Node = {
    val node = new Node(v)
    node.children = ns.toList
    node
  }

  "Example 1: (root = [1,null,3,2,4,null,5,6]) -> [1,3,5,6,2,4]" in {
    val root = n(1, n(3, n(5), n(6)), n(2), n(4))
    Solution.preorder(root) shouldBe List(1, 3, 5, 6, 2, 4)
  }
  "Example 2: (root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14])" +
    " -> [1,2,3,6,7,11,14,4,8,12,5,9,13,10]" in {
    val root = n(1, n(2), n(3, n(6), n(7, n(11, n(14)))), n(4, n(8, n(12))), n(5, n(9, n(13)), n(10)))
    Solution.preorder(root) shouldBe List(1, 2, 3, 6, 7, 11, 14, 4, 8, 12, 5, 9, 13, 10)
  }

  "Performance" should {
    "(root = 10_000 children)" in {
      val root = n(1, (2 to 10_000).map(n(_)): _*)
      Solution.preorder(root) shouldBe (1 to 10000)
    }
    "(root = 10 children with depth 999)" in {
      def deepNode(from: Int, to: Int): Node =
        if (from < to) n(from, deepNode(from + 1, to))
        else n(from)
      val root = n(0, (0 to 9).map(_ * 1000).map(i => deepNode(i + 1, i + 1000)): _*)
      Solution.preorder(root) shouldBe (0 to 10000)
    }
  }
}
