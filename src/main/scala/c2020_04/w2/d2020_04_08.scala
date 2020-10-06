package c2020_04.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d2020_04_08 extends AnyWordSpec with Matchers {

  /**
   * Middle of the Linked List
   *
   * Given a non-empty, singly linked list with head node `head`, return a middle node of linked list.
   *
   * If there are two middle nodes, return the second middle node.
   *
   *
   * <b>Constraints:</b><ul>
   * <li> The number of nodes in the given list will be between `1` and `100`.
   */
  object Solution {
    import scala.annotation.tailrec

    def middleNode(head: ListNode): ListNode = {
      @tailrec
      def loop(ln: ListNode, rsf: ListNode, cnt: Int): ListNode =
        if (ln.next == null) rsf
        else loop(
          ln.next,
          if (cnt % 2 == 0) rsf.next else rsf,
          cnt + 1)

      loop(head, head, 0)
    }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = s"ln($x, $next)"
    def size: Int =
      if (next == null) 1
      else next.size + 1
  }
  private def ln(x: Int, next: ListNode = null): ListNode = new ListNode(x, next)

  "Example 1" in {
    val head = ln(1, ln(2, ln(3, ln(4, ln(5)))))
    val expected = ln(3, ln(4, ln(5)))

    Solution.middleNode(head).toString shouldBe expected.toString
    // The returned node has value 3.  (The judge's serialization of this node is [3,4,5]).
    // Note that we returned a ListNode object ans, such that:
    // ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, and ans.next.next.next = NULL.
  }
  "Example 2" in {
    val head = ln(1, ln(2, ln(3, ln(4, ln(5, ln(6))))))
    val expected = ln(4, ln(5, ln(6)))

    Solution.middleNode(head).toString shouldBe expected.toString
    // Since the list has two middle nodes with values 3 and 4, we return the second one.
  }

  "My test: 1 node list" in {
    val head = ln(1)
    val expected = ln(1)

    Solution.middleNode(head).toString shouldBe expected.toString
  }

  "My test: max size" in {
    var l = ln(1)
    for (i <- 2 to 100) {
      l = ln(i, l)
    }

    Solution.middleNode(l).size shouldBe 50
  }
}
