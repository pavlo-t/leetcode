package c2020_10.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d2020_10_07 extends AnyWordSpec with Matchers {

  /**
   * Rotate List
   *
   * Given a linked list, rotate the list to the right by k places, where k is non-negative.
   */
  object Solution {
    import scala.annotation.tailrec

    def rotateRightSimpleRec(head: ListNode, k: Int): ListNode =
      if (head == null || head.next == null) head
      else {
        @tailrec
        def rotateOneRight(c: ListNode, p: ListNode, h: ListNode): ListNode = {
          if (c.next == null) {
            p.next = null
            c.next = h
            c
          } else rotateOneRight(c.next, c, h)
        }
        @tailrec
        def loop(h: ListNode, i: Int): ListNode =
          if (i == 0) h
          else loop(rotateOneRight(h.next, h, h), i - 1)

        loop(head, k)
      }

    def rotateRightWithBuffer(head: ListNode, k: Int): ListNode = {
      import collection.mutable

      if (head == null || head.next == null) head
      else {
        @tailrec
        def toBuffer(h: ListNode, rsf: mutable.ListBuffer[Int]): mutable.ListBuffer[Int] =
          if (h == null) rsf
          else {
            rsf.addOne(h.x)
            toBuffer(h.next, rsf)
          }

        val buffer = toBuffer(head, mutable.ListBuffer[Int]())
        val realK = k % buffer.size

        val newHead = buffer.takeRight(realK)
        buffer.prependAll(newHead)

        @tailrec
        def updateListNode(head: ListNode, b: mutable.ListBuffer[Int]): Unit =
          if (head != null) {
            head.x = b.head
            updateListNode(head.next, b.tail)
          }
        updateListNode(head, buffer)

        head
      }
    }

    def rotateRightRecSize(head: ListNode, k: Int): ListNode =
      if (head == null || head.next == null) head
      else {
        @tailrec
        def getSize(h: ListNode, rsf: Int): Int =
          if (h == null) rsf
          else getSize(h.next, rsf + 1)

        val size = getSize(head, 0)
        val realK = k % size

        @tailrec
        def rotateOneRight(c: ListNode, p: ListNode, h: ListNode): ListNode = {
          if (c.next == null) {
            p.next = null
            c.next = h
            c
          } else rotateOneRight(c.next, c, h)
        }
        @tailrec
        def loop(h: ListNode, i: Int): ListNode =
          if (i == 0) h
          else loop(rotateOneRight(h.next, h, h), i - 1)

        loop(head, realK)
      }

    def rotateRight(head: ListNode, k: Int): ListNode =
      if (head == null || head.next == null) head
      else {
        @tailrec
        def getSize(h: ListNode, rsf: Int): Int =
          if (h == null) rsf
          else getSize(h.next, rsf + 1)

        val size = getSize(head, 0)
        val realK = k % size

        if (realK == 0) head
        else {
          val newHeadIndex = size - realK - 1

          @tailrec
          def rotateInPlace(h: ListNode, hIdx: Int, newHead: ListNode = null): ListNode = {
            if (h.next == null) {
              h.next = head
              newHead
            } else if (hIdx == newHeadIndex) {
              val next = h.next
              h.next = null
              rotateInPlace(next, hIdx + 1, next)
            } else rotateInPlace(h.next, hIdx + 1, newHead)
          }

          rotateInPlace(head, 0)
        }
      }

  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = s"ln($x, $next)"
  }
  private def ln(x: Int, n: ListNode = null) = new ListNode(x, n)

  "Example 1" in {
    val l = ln(1, ln(2, ln(3, ln(4, ln(5)))))
    val expected = ln(4, ln(5, ln(1, ln(2, ln(3)))))

    Solution.rotateRight(l, 2).toString shouldBe expected.toString
    // Explanation:
    //   rotate 1 steps to the right: 5->1->2->3->4->NULL
    //   rotate 2 steps to the right: 4->5->1->2->3->NULL
  }
  "Example 2" in {
    val l = ln(0, ln(1, ln(2)))
    val expected = ln(2, ln(0, ln(1)))

    Solution.rotateRight(l, 4).toString shouldBe expected.toString
    // Explanation:
    //   rotate 1 steps to the right: 2->0->1->NULL
    //   rotate 2 steps to the right: 1->2->0->NULL
    //   rotate 3 steps to the right: 0->1->2->NULL
    //   rotate 4 steps to the right: 2->0->1->NULL
  }

  "My test: empty list" in {
    Solution.rotateRight(null, 2) shouldBe null
  }
  "My test: 1 el list" in {
    Solution.rotateRight(ln(1), 2).toString shouldBe ln(1).toString
  }
}
