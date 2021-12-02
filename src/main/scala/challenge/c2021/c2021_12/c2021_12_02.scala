package challenge.c2021.c2021_12

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/** [[https://leetcode.com/problems/odd-even-linked-list/]] */
class c2021_12_02 extends AnyWordSpec with Matchers {

  /**
   * = 328. Odd Even Linked List =
   */
  object Solution {
    /** [[https://leetcode.com/problems/odd-even-linked-list/solution/]] */
    def oddEvenList(head: ListNode): ListNode =
      if (head == null) head
      else {
        var odd = head
        var even = head.next
        val evenHead = even
        while (even != null && even.next != null) {
          odd.next = even.next
          odd = odd.next
          even.next = odd.next
          even = even.next
        }
        odd.next = evenHead
        head
      }

    def oddEvenListMy(head: ListNode): ListNode =
      if (head == null || head.next == null || head.next.next == null) head
      else {
        var r = head.next
        var len = 2
        // get to the last node, count list length
        while (r.next != null) {
          r = r.next
          len += 1
        }
        len = len / 2

        var l = head
        while (len > 0) {
          r.next = l.next
          l.next = l.next.next
          l = l.next
          r = r.next
          r.next = null
          len -= 1
        }
        head
      }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = {
      val sb = new StringBuilder
      sb.addAll(s"[$x")
      var curr = this.next
      while (curr != null) {
        sb.addOne(',')
        sb.addAll(curr.x.toString)
        curr = curr.next
      }
      sb.addOne(']')
      sb.toString()
    }

    //override def equals(other: Any): Boolean =
    //  other.isInstanceOf[ListNode] && {
    //    var t = this
    //    var o = other.asInstanceOf[ListNode]
    //    while (t != null && o != null) {
    //      if (t.x != o.x) return false
    //      else {
    //        t = t.next
    //        o = o.next
    //      }
    //    }
    //    t == null && o == null
    //  }
  }

  private def l(xs: Int*): ListNode =
    if (xs.isEmpty) null
    else {
      val head = new ListNode(xs.head)
      var curr = head
      for (x <- xs.tail) {
        val n = new ListNode(x)
        curr.next = n
        curr = n
      }
      head
    }

  private def check(in: ListNode, e: ListNode): Unit =
    if (in == null) Solution.oddEvenList(in) shouldBe e
    else Solution.oddEvenList(in).toString shouldBe e.toString

  "[]" in check(null, null)
  "[1]" in check(l(1), l(1))
  "[1,2]" in check(l(1, 2), l(1, 2))
  "[1,2,3]" in check(l(1, 2, 3), l(1, 3, 2))
  "[1,2,3,4]" in check(l(1, 2, 3, 4), l(1, 3, 2, 4))
  //  l       r
  // [1,2,3,4,5]
  //    l     r
  // [1,3,4,5,2]
  //      l   r
  // [1,3,5,2,4]
  "[1,2,3,4,5]" in check(l(1, 2, 3, 4, 5), l(1, 3, 5, 2, 4))
  //  l         r
  // [1,2,3,4,5,6]
  //    l       r
  // [1,3,4,5,6,2]
  //      l     r
  // [1,3,5,6,2,4]
  //        l   r
  // [1,3,5,2,4,6]
  "[1,2,3,4,5,6]" in check(l(1, 2, 3, 4, 5, 6), l(1, 3, 5, 2, 4, 6))
  "[2,1,3,5,6,4,7]" in check(l(2, 1, 3, 5, 6, 4, 7), l(2, 3, 6, 7, 1, 5, 4))
}
