package challenge.c2022.c2022_09

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * =429. N-ary Tree Level Order Traversal=
 *
 * Given an n-ary tree, return the ''level order'' traversal of its nodes' values.
 *
 * ===Constraints===
 *
 *  - The height of the n-ary tree is less than or equal to `1000`
 *  - The total number of nodes is between `[0, 10_000]`
 */
class c2022_09_05 extends AnyWordSpec with Matchers {

  object Solution {
    def levelOrder(root: Node): List[List[Int]] = {
      @scala.annotation.tailrec
      def rec(curr: List[Node],
              next: List[Node],
              rsf: List[List[Int]]): List[List[Int]] = (curr, next) match {
        case (Nil, Nil)           => (rsf.head.reverse :: rsf.tail).reverse
        case (Nil, next)          => rec(next.reverse, List.empty, List.empty :: rsf.head.reverse :: rsf.tail)
        case (head :: rest, next) => rec(rest, head.children.reverse ::: next, (head.value :: rsf.head) :: rsf.tail)
      }

      if (root == null) List.empty
      else rec(List(root), List.empty, List(List.empty))
    }
  }

  class Node(var _value: Int) {
    var value: Int = _value
    var children: List[Node] = List()
  }

  private def n(v: Int, children: Node*): Node = {
    val node = new Node(v)
    node.children = children.toList
    node
  }


  "[1,null,3,2,4,null,5,6]" in {
    val root =
      n(1,
        n(3,
          n(5),
          n(6)),
        n(2),
        n(4))
    Solution.levelOrder(root) shouldBe List(List(1), List(3, 2, 4), List(5, 6))
  }

  "[1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]" in {
    val root =
      n(1,
        n(2),
        n(3,
          n(6),
          n(7,
            n(11,
              n(14)))),
        n(4,
          n(8,
            n(12))),
        n(5,
          n(9,
            n(13)),
          n(10)))
    val expected = List(List(1), List(2, 3, 4, 5), List(6, 7, 8, 9, 10), List(11, 12, 13), List(14))
    Solution.levelOrder(root) shouldBe expected

  }

}
