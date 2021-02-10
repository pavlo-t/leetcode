package challenge.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3635/]]
 */
//noinspection ConvertNullInitializerToUnderscore
class c2021_02_10 extends AnyWordSpec with Matchers {
  /**
   * === Copy List with Random Pointer ===
   *
   * A linked list of length `n` is given such that each node contains an additional random pointer,
   * which could point to any node in the list, or `null`.
   *
   * Construct a [[https://en.wikipedia.org/wiki/Object_copying#Deep_copy deep copy]] of the list.
   * The deep copy should consist of exactly `n` '''brand new''' nodes,
   * where each new node has its value set to the value of its corresponding original node.
   * Both the `next` and `random` pointer of the new nodes should point to new nodes in the copied list
   * such that the pointers in the original list and copied list represent the same list state.
   * '''None of the pointers in the new list should point to nodes in the original list'''.
   *
   * For example, if there are two nodes `X` and `Y` in the original list, where `X.random --> Y`,
   * then for the corresponding two nodes `x` and `y` in the copied list, `x.random --> y`.
   *
   * Return ''the head of the copied linked list''.
   *
   * The linked list is represented in the input/output as a list of `n` nodes.
   * Each node is represented as a pair of `[val, random_index]` where:
   *  - `val`: an integer representing `Node.val`
   *  - `random_index`: the index of the node (range from `0` to `n-1`) that the random pointer points to,
   *    or `null` if it does not point to any node.
   *
   * Your code will '''only''' be given the `head` of the original linked list.
   *
   * '''Constraints:'''
   *  - `0 <= n <= 1000`
   *  - `-10_000 <= Node.val <= 10_000`
   *  - `Node.random` is `null` or is pointing to some node in the linked list.
   */
  object Solution {
    def copyRandomList(head: Node): Node = {
      @scala.annotation.tailrec
      def copyList(n: Node, p: Node = null, map: Map[Node, Node] = Map()): Map[Node, Node] = {
        if (n == null) map
        else {
          val nc = new Node(n.value)
          if (p != null) p.next = nc
          copyList(n.next, nc, map.updated(n, nc))
        }
      }
      val map = copyList(head)

      @scala.annotation.tailrec
      def setRandoms(n: Node): Node =
        if (n != null) {
          if (n.random != null)
            map(n).random = map(n.random)
          setRandoms(n.next)
        } else map.getOrElse(head, null)

      setRandoms(head)
    }
  }

  object SolutionMy {
    def copyRandomList(head: Node): Node = {
      val sentinel = new Node(0)
      @scala.annotation.tailrec
      def copyList(n: Node, prev: Node = sentinel): Node =
        if (n == null) sentinel.next
        else {
          val nCopy = new Node(n.value)
          prev.next = nCopy
          copyList(n.next, nCopy)
        }
      val headCopy = copyList(head)
      val size = head.size

      @scala.annotation.tailrec
      def setRandoms(n: Node, nCopy: Node): Node =
        if (n != null) {
          if (n.random != null)
            nCopy.random = headCopy(size - n.random.size)
          setRandoms(n.next, nCopy.next)
        } else headCopy

      setRandoms(head, headCopy)
    }

    implicit class NodeSize(node: Node) {
      def apply(i: Int): Node = {
        @scala.annotation.tailrec
        def tailRec(n: Node, i: Int): Node = {
          if (i == 0) n
          else if (n == null || n.next == null) null
          else tailRec(n.next, i - 1)
        }
        tailRec(node, i)
      }

      def size: Int = {
        @scala.annotation.tailrec
        def tailRec(n: Node, s: Int): Int =
          if (n == null) s
          else tailRec(n.next, s + 1)
        tailRec(node, 0)
      }
    }
  }

  class Node(var _value: Int) {
    var value: Int = _value
    var next: Node = null
    var random: Node = null

    def apply(i: Int): Node = {
      @scala.annotation.tailrec
      def tailRec(n: Node, i: Int): Node = {
        if (i == 0) n
        else if (n.next == null) null
        else tailRec(n.next, i - 1)
      }
      tailRec(this, i)
    }

    override def toString: String = {
      @scala.annotation.tailrec
      def rIndex(n: Node, i: Int = -1): Int =
        if (n == null) i
        else if (n.next == null) i + 1
        else rIndex(n.next, i + 1)

      val s = rIndex(this)

      @scala.annotation.tailrec
      def stringifyNodes(n: Node, rsf: Seq[String] = Seq()): String =
        if (n == null) rsf.mkString(",")
        else {
          val ri = rIndex(n.random) match {
            case -1 => null
            case rs => s - rs
          }
          stringifyNodes(n.next, rsf :+ s"[${n.value},$ri]")
        }

      "[" + stringifyNodes(this) + "]"
    }
  }

  import Solution.copyRandomList

  private def l(v: Int, n: Node = null): Node = {
    val node = new Node(v)
    node.next = n
    node
  }

  "Example 1: (head = [[7,null],[13,0],[11,4],[10,2],[1,0]])" in {
    val head = l(7, l(13, l(11, l(10, l(1)))))
    head(0).random = null
    head(1).random = head(0)
    head(2).random = head(4)
    head(3).random = head(2)
    head(4).random = head(0)

    val result = copyRandomList(head)

    (result == head) shouldBe false
    result.toString shouldBe head.toString
  }
  "Example 2: (head = [[1,1],[2,1]])" in {
    val head = l(1, l(2))
    head(0).random = head(1)
    head(1).random = head(1)

    val result = copyRandomList(head)

    (result == head) shouldBe false
    result.toString shouldBe head.toString
  }
  "Example 3: (head = [[3,null],[3,0],[3,null]])" in {
    val head = l(3, l(3, l(3)))
    head(0).random = null
    head(1).random = head(0)
    head(2).random = null

    val result = copyRandomList(head)

    (result == head) shouldBe false
    result.toString shouldBe head.toString
  }
  "Example 4: (head = [])" in {
    copyRandomList(null) shouldBe null
    //Explanation: The given linked list is empty (null pointer), so return null.
  }

  "(head = [1 to 1000])" in {
    @scala.annotation.tailrec
    def buildList(v: Int, p: Node = null, r: Node = null): Node =
      if (v <= 0) r
      else if (p == null) {
        val n = new Node(v)
        buildList(v - 1, n, n)
      } else {
        val n = new Node(v)
        n.random = r
        p.next = n
        buildList(v - 1, n, r)
      }

    val head = buildList(1000)

    val result = copyRandomList(head)

    (result == head) shouldBe false
    result.toString shouldBe head.toString
  }
}
