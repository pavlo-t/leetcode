package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3596/]]
 */
//noinspection ZeroIndexToHead,DuplicatedCode
class c2021_01_w2 extends AnyWordSpec with Matchers {
  /**
   * === Find Root of N-Ary Tree ===
   *
   * You are given all the nodes of an [[https://leetcode.com/articles/introduction-to-n-ary-trees/ N-ary tree]]
   * as an array of `Node`objects, where each node has a '''unique value'''.
   *
   * Return ''the '''root''' of the N-ary tree''.
   *
   * '''Custom testing:'''
   *
   * An N-ary tree can be serialized as represented in its level order traversal where each group of children
   * is separated by the `null` value (see examples).
   *
   * {{{
   *          1
   * 2     3     4     5
   *     6   7   8   9  10
   *        11  12  13
   *        14
   * }}}
   *
   * For example, the above tree is serialized as
   * `[1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]`.
   *
   * The testing will be done in the following way:
   *  1. The '''input data''' should be provided as a serialization of the tree.
   *  1. The driver code will construct the tree from the serialized input data and put each
   *     `Node` object into an array '''in an arbitrary order'''.
   *  1. The driver code will pass the array to `findRoot`,
   *     and your function should find and return the root `Node` object in the array.
   *  1. The driver code will take the returned `Node` object and serialize it.
   *     If the serialized value and the input data are the '''same''', the test '''passes'''.
   *
   * '''Constraints:'''
   *  - The total number of nodes is between `[1, 50_000]`.
   *  - Each node has a '''unique''' value.
   *
   *
   * '''Follow up:'''
   *  - Could you solve this problem in constant space complexity with a linear time algorithm?
   */
  object Solution {
    def findRoot(tree: List[Node]): Node = {
      val seenOnce = tree.foldLeft(0)((acc, n) => n.children.foldLeft(acc ^ n.value)(_ ^ _.value))
      tree.find(_.value == seenOnce).get
    }
  }
  object SolutionFoldSumDiff {
    def findRoot(tree: List[Node]): Node = {
      val seenOnce = tree.foldLeft(0)((acc, n) => n.children.foldLeft(acc + n.value)(_ - _.value))
      tree.find(_.value == seenOnce).get
    }
  }

  object SolutionSetOfNodeValues {
    def findRoot(tree: List[Node]): Node = {
      val children = tree.flatMap(_.children.map(_.value)).toSet
      tree.find(n => !children.contains(n.value)).get
    }
  }
  object SolutionSetOfNodes {
    def findRoot(tree: List[Node]): Node = {
      val children = tree.flatMap(_.children).toSet
      tree.find(!children.contains(_)).get
    }
  }

  class Node(var _value: Int) {
    var value: Int = _value
    var children: List[Node] = List()

    def c(i: Int): Node = children(i)
    override def toString: String =
      if (children.isEmpty) s"$value"
      else s"$value,${children.mkString("[", ",", "]")}"
  }

  import Solution.findRoot

  private def n(v: Int, c: List[Node] = List()): Node = {
    val n = new Node(v)
    n.children = c
    n
  }

  "Example 1: [1,null,3,2,4,null,5,6]" in {
    val r = n(1, List(n(3, List(n(5), n(6))), n(2), n(4)))
    val tree = List(
      r,
      r.c(0), r.c(1), r.c(2),
      r.c(0).c(0), r.c(0).c(1))

    findRoot(util.Random.shuffle(tree)).toString shouldBe r.toString
    //Explanation:
    //     1
    //  3  2  4
    // 5 6
    // The driver code creates the tree and gives findRoot the Node objects in an arbitrary order.
    // For example, the passed array could
    // be [Node(5),Node(4),Node(3),Node(6),Node(2),Node(1)]
    // or [Node(2),Node(6),Node(1),Node(3),Node(5),Node(4)].
    // The findRoot function should return the root Node(1),
    // and the driver code will serialize it and compare with the input data.
    // The input data and serialized Node(1) are the same, so the test passes.
  }
  "Example 2: [1,null,2,3,4,5,null,null,6,7,null,8,null,9,10,null,null,11,null,12,null,13,null,null,14]" in {
    val r =
      n(1, List(
        n(2),
        n(3, List(
          n(6),
          n(7, List(
            n(11, List(
              n(14))))))),
        n(4, List(
          n(8, List(
            n(12))))),
        n(5, List(
          n(9, List(
            n(13))),
          n(10)))))
    val tree = List(
      r,
      r.c(0), r.c(1), r.c(2), r.c(3),
      r.c(1).c(0), r.c(1).c(1), r.c(2).c(0), r.c(3).c(0), r.c(3).c(1),
      r.c(1).c(1).c(0), r.c(2).c(0).c(0), r.c(3).c(0).c(0),
      r.c(1).c(1).c(0).c(0))

    findRoot(util.Random.shuffle(tree)).toString shouldBe r.toString
    //          1
    // 2     3     4     5
    //     6   7   8   9  10
    //        11  12  13
    //        14
  }


}
