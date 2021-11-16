package challenge.c2021.c2021_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/problems/diameter-of-n-ary-tree/]] */
//noinspection DuplicatedCode
class c2021_11_w3 extends AnyWordSpec with Matchers {

  /**
   * = 1522. Diameter of N-Ary Tree =
   *
   * Given a `root` of an [[https://leetcode.com/articles/introduction-to-n-ary-trees/ N-ary tree]],
   * you need to compute the length of the diameter of the tree.
   *
   * The diameter of an N-ary tree is the length of the '''longest''' path between any two nodes in the tree.
   * This path may or may not pass through the root.
   *
   * (''Nary-Tree input serialization is represented in their level order traversal,
   * each group of children is separated by the null value''.)
   *
   * '''Constraints:'''
   *  - The depth of the n-ary tree is less than or equal to `1000`.
   *  - The total number of nodes is between `[1, 10_000]`.
   */
  object Solution {
    def diameter(root: Node): Int = {
      import scala.util.chaining.scalaUtilChainingOps

      def height(n: Node): Int =
        n.children
          .map(height)
          .maxOption
          .getOrElse(0) + 1

      def diam(n: Node): Int =
        n.children
          .map(height)
          .foldLeft((0, 0)) { case (prev@(m1, m2), h) =>
            if (h >= m1) (h, m1)
            else if (h > m2) (m1, h)
            else prev
          }
          .pipe { case (m1, m2) => m1 + m2 }
          .max(n.children.map(diam).maxOption.getOrElse(0))

      diam(root)
    }
  }

  class Node(var _value: Int) {
    var value: Int = _value
    var children: List[Node] = List()
  }

  private def n(v: Int, cs: Node*): Node = {
    val node = new Node(v)
    node.children = cs.toList
    node
  }

  "(1) -> 0" in (Solution.diameter(n(1)) shouldBe 0)
  "(1,n,2) -> 1" in (Solution.diameter(n(1, n(2))) shouldBe 1)
  "(1,n,3,2,4,n,5,6) -> 3" in {
    val r = n(1, n(3, n(5), n(6)), n(2), n(4))
    Solution.diameter(r) shouldBe 3
    // Explanation: 5-3-1-2 or 5-3-1-4
    //    1
    //  3  2 4
    // 5 6
  }
  "(1,n,2,n,3,4,n,5,n,6) -> 4" in {
    val r = n(1, n(2, n(3, n(5)), n(4, n(6))))
    Solution.diameter(r) shouldBe 4
    //    1
    //   2
    //  3 4
    // 5   6
  }
  "(1,n,2,3,4,5,n,n,6,7,n,8,n,9,10,n,n,11,n,12,n,13,n,n,14) -> 7" in {
    val r = n(1, n(2), n(3, n(6), n(7, n(11, n(14)))), n(4, n(8, n(12))), n(5, n(9, n(13)), n(10)))
    Solution.diameter(r) shouldBe 7
    //        1
    //  2   3   4   5
    //     6 7  8  9 10
    //       11 12 13
    //       14
  }

  "Performance" should {
    "(root = 10_000 children)" in {
      val root = n(1, (2 to 10_000).map(n(_)): _*)
      Solution.diameter(root) shouldBe 2
    }
    "(root = 10 children with depth 999)" in {
      def deepNode(from: Int, to: Int): Node =
        if (from < to) n(from, deepNode(from + 1, to))
        else n(from)

      val root = n(0, (0 to 9).map(_ * 1000).map(i => deepNode(i + 1, i + 1000)): _*)
      Solution.diameter(root) shouldBe 2000
    }
  }
}
