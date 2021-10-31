package challenge.c2021.c2021_10

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/]] */
//noinspection DuplicatedCode
class c2021_10_31 extends AnyWordSpec with Matchers {

  /**
   * = 430. Flatten a Multilevel Doubly Linked List =
   *
   * You are given a doubly linked list which in addition to the next and previous pointers,
   * it could have a child pointer, which may or may not point to a separate doubly linked list.
   * These child lists may have one or more children of their own, and so on,
   * to produce a multilevel data structure, as shown in the example below.
   *
   * Flatten the list so that all the nodes appear in a single-level, doubly linked list.
   * You are given the head of the first level of the list.
   *
   * '''Constraints:'''
   *  - The number of Nodes will not exceed `1000`.
   *  - `1 <= Node.val <= 100_000`
   */
  object Solution {
    // From comments at https://leetcode.com/problems/flatten-a-multilevel-doubly-linked-list/solution/
    def flatten(head: Node): Node = {
      var tail: Node = null

      def rec(head: Node): Node = {
        if (head == null) null
        else {
          head.prev = tail
          tail = head

          val next = head.next

          head.next = rec(head.child)
          head.child = null

          tail.next = rec(next)

          head
        }
      }

      rec(head)
    }

    // 18:28-18:31
    def flattenMutable(head: Node): Node = {
      if (head == null) head
      else {
        def attachNext(n: Node, next: Node): Node = {
          if (n != null && next != null) {
            var curr = n
            while (curr.next != null) curr = curr.next
            curr.next = next
            next.prev = curr
          }
          n
        }

        head.next =
          if (head.child != null)
            attachNext(flattenMutable(head.child), flattenMutable(head.next))
          else
            flattenMutable(head.next)
        if (head.next != null)
          head.next.prev = head
        head.child = null

        head
      }
    }

    // 17:58-18:17
    def flattenImmutable(head: Node): Node = {
      if (head == null) head
      else {
        def attachNext(n: Node, next: Node): Node = {
          if (n != null && next != null) {
            var curr = n
            while (curr.next != null) curr = curr.next
            curr.next = next
            next.prev = curr
          }
          n
        }

        val n = new Node(head.value)
        n.next =
          if (head.child != null)
            attachNext(flattenImmutable(head.child), flattenImmutable(head.next))
          else
            flattenImmutable(head.next)
        if (n.next != null)
          n.next.prev = n

        n
      }
    }
  }

  //noinspection ConvertNullInitializerToUnderscore
  class Node(var _value: Int) {
    var value: Int = _value
    var prev: Node = null
    var next: Node = null
    var child: Node = null

    override def toString: String = {
      val sb = new StringBuilder
      sb.appendAll(s"L(")
      if (prev != null) sb.appendAll(s"p=${prev.value},$value")
      else sb.append(value)
      if (child != null) sb.appendAll(s",c=$child")
      if (next != null) sb.appendAll(s",$next")
      sb.appendAll(")")
      sb.toString()
    }
  }

  private def l(v: Int, n: Node, c: Node = null): Node = {
    val r = new Node(v)
    r.next = n
    if (n != null) n.prev = r
    r.child = c

    r
  }

  private def l(vs: Int*): Node = {
    var c: Node = null
    for (v <- vs.reverseIterator) c = l(v, c)
    c
  }

  "Example 1: (head = [1,2,3,4,5,6,null,null,null,7,8,9,10,null,null,11,12]) -> [1,2,3,7,8,11,12,9,10,4,5,6]" in {
    val l1 = l(11, 12)
    val l2 = l(7, l(8, l(9, 10), l1))
    val head = l(1, l(2, l(3, l(4, 5, 6), l2)))
    val e = l(1, 2, 3, 7, 8, 11, 12, 9, 10, 4, 5, 6)
    Solution.flatten(head).toString shouldBe e.toString
    // Explanation: The multilevel linked list in the input is as follows:
    // 1-2-3-4-5-6-NULL
    //     |
    //     7-8-9-10-NULL
    //       |
    //       11-12-NULL
    // After flattening the multilevel linked list it becomes:
    // 1-2-3-7-8-11-12-9-10-4-5-6-NULL
  }
  "Example 2: (head = [1,2,null,3]) -> [1,3,2]" in {
    val head = l(1, l(2), l(3))
    val e = l(1, 3, 2)
    Solution.flatten(head).toString shouldBe e.toString
    // Explanation: The input multilevel linked list is as follows:
    // 1-2-NULL
    // |
    // 3-NULL
  }
  "Example 3: (head = []) -> []" in {
    Solution.flatten(null) shouldBe null
  }

}
