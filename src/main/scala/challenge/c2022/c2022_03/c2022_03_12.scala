package challenge.c2022.c2022_03

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/problems/copy-list-with-random-pointer/]]
 */
class c2022_03_12 extends AnyWordSpec with Matchers {
  /**
   * =138. Copy List with Random Pointer=
   *
   * A linked list of length `n` is given such that each node contains an additional random pointer,
   * which could point to any node in the list, or `null`.
   *
   * Construct a '''deep copy''' of the list.
   * The deep copy should consist of exactly `n` '''brand new''' nodes,
   * where each new node has its value set to the value of its corresponding original node.
   * Both the `next` and `random` pointer of the new nodes should point to new nodes in the copied list such
   * that the pointers in the original list and copied list represent the same list state.
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
   *  - `random_index`: the index of the node (range from `0` to `n-1`) that the `random` pointer points to,
   *    or `null` if it does not point to any node.
   *
   * Your code will '''only''' be given the `head` of the original linked list.
   *
   * '''Constraints:'''
   *  - `0 <= n <= 1000`
   *  - `-10_000 <= Node.val <= 10_000`
   *  - `Node.random` is `null` or is pointing to some node in the linked list.
   *
   */
  object Solution {
    def copyRandomList(head: Node): Node = {
      val dummy = new Node(0)

      var map: Map[Node, Node] = Map()
      var curr = head
      while (curr != null) {
        map = map.updated(curr, new Node(curr.value))
        curr = curr.next
      }

      curr = head
      var currResult = dummy
      while (curr != null) {
        currResult.next = map(curr)
        if (curr.random != null) {
          currResult.next.random = map(curr.random)
        }
        currResult = currResult.next
        curr = curr.next
      }
      dummy.next
    }
  }

  //noinspection ConvertNullInitializerToUnderscore
  class Node(var _value: Int) {
    var value: Int = _value
    var next: Node = null
    var random: Node = null

    override def toString: String = {
      val sb = new StringBuilder
      sb.addOne('{')
      var curr = this
      while (curr != null) {
        sb.addOne('(')
        sb.addAll(curr.value.toString)
        sb.addOne(',')
        sb.addAll(if (curr.random == null) "n" else curr.random.value.toString)
        sb.addOne(')')
        curr = curr.next
      }
      sb.addOne('}')
      sb.toString()
    }
  }

  private val N = -1

  private def l(ns: (Int, Int)*): Node = {
    val nodes = ns.map(_._1).map(new Node(_))
    for (((_, rand), i) <- ns.zipWithIndex if rand != N) {
      nodes(i).random = nodes(rand)
    }
    nodes.reverseIterator.reduce((next, curr) => {
      curr.next = next
      curr
    })
  }


  "[]" in {
    Solution.copyRandomList(null) shouldBe null
  }
  "[[1,0]]" in {
    Solution.copyRandomList(l((1, 0))).toString shouldBe l((1, 0)).toString
  }
  "[[1,null]]" in {
    Solution.copyRandomList(l((1, N))).toString shouldBe l((1, N)).toString
  }

  "[[7,null],[13,0],[11,4],[10,2],[1,0]]" in {
    val h = l((7, N), (13, 0), (11, 4), (10, 2), (1, 0))
    Solution.copyRandomList(h).toString shouldBe h.toString
  }
  "[[1,1],[2,1]]" in {
    val h = l((1, 1), (2, 1))
    Solution.copyRandomList(h).toString shouldBe h.toString
  }
  "[[3,null],[3,0],[3,null]]" in {
    val h = l((3, N), (3, 0), (3, N))
    Solution.copyRandomList(h).toString shouldBe h.toString
  }
}
