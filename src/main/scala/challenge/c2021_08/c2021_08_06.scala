package challenge.c2021_08

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/august-leetcoding-challenge-2021/613/week-1-august-1st-august-7th/3871/]]
 */
class c2021_08_06 extends AnyWordSpec with Matchers {
  /**
   * == N-ary Tree Level Order Traversal ==
   *
   * Given an n-ary tree, return the ''level'' order traversal of its nodes' values.
   *
   * ''Nary-Tree input serialization is represented in their level order traversal,
   * each group of children is separated by the null value (See examples)''.
   *
   * '''Constraints:'''
   * - The height of the n-ary tree is less than or equal to `1000`
   * - The total number of nodes is between `[0, 10_000]`
   */
  object Solution {
    def levelOrder(root: Node): List[List[Int]] = {
      @scala.annotation.tailrec
      def rec(todo: Seq[Node], nextLvl: Seq[Node], rsf: List[List[Int]]): List[List[Int]] = {
        todo match {
          case Nil       => if (nextLvl.isEmpty) rsf else rec(nextLvl, Seq(), Nil :: rsf)
          case n :: rest => rec(rest, nextLvl ++ n.children, (n.value :: rsf.head) :: rsf.tail)
        }
      }

      if (root == null) List()
      else rec(Seq(root), Seq(), List(List())).map(_.reverse).reverse
    }
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
  "Example 1: (root = [1,null,3,2,4,null,5,6]) -> [[1],[3,2,4],[5,6]]" in {
    val root = n(1, n(3, n(5), n(6)), n(2), n(4))
    val e = List(List(1), List(3, 2, 4), List(5, 6))
    Solution.levelOrder(root) shouldBe e
  }
  "Example 2: (root = [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14])" +
    " -> [[1],[2,3,4,5],[6,7,8,9,10],[11,12,13],[14]]" in {
    val root = n(1, n(2), n(3, n(6), n(7, n(11, n(14)))), n(4, n(8, n(12))), n(5, n(9, n(13)), n(10)))
    val e = List(List(1), List(2, 3, 4, 5), List(6, 7, 8, 9, 10), List(11, 12, 13), List(14))
    Solution.levelOrder(root) shouldBe e
  }

  "Performance" should {
    "(root = 10_000 children)" in {
      val root = n(1, (2 to 10_000).map(n(_)): _*)
      Solution.levelOrder(root) shouldBe List(List(1), 2 to 10_000)
    }
    "(root = 10 children with depth 999)" in {
      def deepNode(from: Int, to: Int): Node =
        if (from < to) n(from, deepNode(from + 1, to))
        else n(from)
      val root = n(0, (0 to 9).map(_ * 1000).map(i => deepNode(i + 1, i + 1000)): _*)
      val result = Solution.levelOrder(root)
      result should have size 1001
    }
  }

}
