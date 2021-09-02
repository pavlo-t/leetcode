package challenge.c2020.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class d2020_11_02 extends AnyWordSpec with Matchers {

  /**
   * <h3>Insertion Sort List</h3>
   *
   * Sort a linked list using insertion sort.
   *
   * The partial sorted list initially contains only the first element in the list.
   * With each iteration one element is removed from the input data and inserted in-place into the sorted list.
   *
   * <b>Algorithm of Insertion Sort:</b><ol>
   * <li> Insertion sort iterates, consuming one input element each repetition, and growing a sorted output list.
   * <li> At each iteration, insertion sort removes one element from the input data,
   * finds the location it belongs within the sorted list, and inserts it there.
   * <li> It repeats until no input elements remain.
   * </ol>
   */
  object Solution {
    def insertionSortList(head: ListNode): ListNode = {
      if (head == null) null
      else {
        @scala.annotation.tailrec
        def sort(h: ListNode, rsf: ListNode = null): ListNode =
          if (h == null) rsf
          else sort(h.next, insert(h.x, rsf))

        def insert(x: Int, rsf: ListNode): ListNode = {
          if (rsf == null || rsf.x >= x) new ListNode(x, rsf)
          else {
            @scala.annotation.tailrec
            def _insert(p: ListNode, c: ListNode): Unit = {
              if (c == null || c.x >= x) p.next = new ListNode(x, c)
              else _insert(c, c.next)
            }
            _insert(rsf, rsf.next)
            rsf
          }
        }

        sort(head)
      }
    }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = s"L($x, $next)"
  }
  private def L(x: Int, n: ListNode = null): ListNode = new ListNode(x, n)

  import Solution.insertionSortList

  "Example 1: (4->2->1->3): 1->2->3->4" in {
    val head = L(4, L(2, L(1, L(3))))
    val expected = L(1, L(2, L(3, L(4))))

    insertionSortList(head).toString shouldBe expected.toString
  }

  "Example 2: (-1->5->3->4->0): -1->0->3->4->5" in {
    val head = L(-1, L(5, L(3, L(4, L(0)))))
    val expected = L(-1, L(0, L(3, L(4, L(5)))))

    insertionSortList(head).toString shouldBe expected.toString
  }

  "(null): null" in {
    insertionSortList(null) shouldBe null
  }
  "(1): 1" in {
    insertionSortList(L(1)).toString shouldBe L(1).toString
  }
  "(1->1): 1->1" in {
    insertionSortList(L(1, L(1))).toString shouldBe L(1, L(1)).toString
  }
  "(2->1): 1->2" in {
    insertionSortList(L(2, L(1))).toString shouldBe L(1, L(2)).toString
  }
}
