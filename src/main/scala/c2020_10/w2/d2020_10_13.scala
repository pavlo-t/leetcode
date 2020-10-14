package c2020_10.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


//noinspection DuplicatedCode
class d2020_10_13 extends AnyWordSpec with Matchers {

  /**
   * Sort List
   *
   * Given the head of a linked list, return the list after sorting it in ascending order.
   *
   * <b>Follow up</b>: Can you sort the linked list in <code>O(n log<sub>n</sub>)</code> time
   * and <code>O(1) memory</code> (i.e. constant space)?
   *
   *
   * <b>Constraints:</b><ul>
   * <li> The number of nodes in the list is in the range <code>[0, 5 * 10<sup>4</sup>]</code>.
   * <li> <code>-10<sup>5</sup> <= Node.val <= 10<sup>5</sup></code>
   * </ul>
   */
  object Solution {
    import scala.annotation.tailrec

    def sortList(head: ListNode): ListNode =
      if (head == null || head.next == null) head
      else {
        val (left, right) = split(head)
        merge(sortList(left), sortList(right))
      }

    private def split(xs: ListNode): (ListNode, ListNode) = {
      var middle = xs
      @tailrec
      def findMiddle(ln: ListNode): Unit = {
        if (ln.next != null && ln.next.next != null) {
          middle = middle.next
          findMiddle(ln.next.next)
        } else {
          val rest = middle.next
          middle.next = null
          middle = rest
        }
      }
      findMiddle(xs)
      (xs, middle)
    }

    private def merge(xs: ListNode, ys: ListNode): ListNode = {
      @tailrec
      def loop(ls: ListNode, rs: ListNode,
               head: ListNode, current: ListNode): ListNode = {
        if (ls == null) {
          current.next = rs
          head
        } else if (rs == null) {
          current.next = ls
          head
        } else if (ls.x < rs.x) {
          val next = ls
          current.next = next
          loop(ls.next, rs, head, next)
        } else {
          val next = rs
          current.next = next
          loop(ls, rs.next, head, next)
        }
      }

      if (xs == null) ys
      else if (ys == null) xs
      else if (xs.x <= ys.x) {
        val head = xs
        loop(xs.next, ys, head, head)
      } else {
        val head = ys
        loop(xs, ys.next, head, head)
      }
    }
  }

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = s"$x,$next"
  }
  private def ln(x: Int, next: ListNode = null) =
    new ListNode(x, next)

  "Example 1: [4,2,1,3]" in {
    val input = ln(4, ln(2, ln(1, ln(3))))
    val expected = ln(1, ln(2, ln(3, ln(4))))
    Solution.sortList(input).toString shouldBe expected.toString
  }
  "Example 2: [-1,5,3,4,0]" in {
    val input = ln(-1, ln(5, ln(3, ln(4, ln(0)))))
    val expected = ln(-1, ln(0, ln(3, ln(4, ln(5)))))
    Solution.sortList(input).toString shouldBe expected.toString
  }
  "Example 3: []" in {
    Solution.sortList(null) shouldBe null
  }

  "My test: max size" in {
    import util.Random

    def arrayToLn(arr: Array[Int]): ListNode = {
      val root = ln(arr(0))
      var current = root

      for (i <- 1 until arr.length) {
        val next = ln(arr(i))
        current.next = next
        current = next
      }

      root
    }

    //val length = 29362
    val length = 50000
    val arr = Array.ofDim[Int](length)
    for (i <- 1 until length) {
      arr(i) = Random.nextInt(200001) - 100000
    }
    val input = arrayToLn(arr)
    val result = Solution.sortList(input)

    var current = result
    for (x <- arr.sorted) {
      current.x shouldBe x
      current = current.next
    }
    current shouldBe null
  }

  object SolutionWithMutableBufferAndMutateInput {
    import collection.mutable
    import scala.annotation.tailrec

    def sortList(head: ListNode): ListNode =
      if (head == null || head.next == null) head
      else {
        @tailrec
        def lnToBuffer(ln: ListNode, rsf: mutable.ArrayBuffer[Int]): mutable.ArrayBuffer[Int] = {
          if (ln == null) rsf
          else lnToBuffer(ln.next, rsf.addOne(ln.x))
        }
        val buffer = lnToBuffer(head, mutable.ArrayBuffer[Int]())
        buffer.sortInPlace()

        @tailrec
        def updateHead(i: Int, currentNode: ListNode): Unit = {
          if (i < buffer.size) {
            currentNode.x = buffer(i)
            updateHead(i + 1, currentNode.next)
          }
        }
        updateHead(0, head)

        head
      }
  }

  object SolutionWithMutableBuffer {
    import collection.mutable
    import scala.annotation.tailrec

    def sortList(head: ListNode): ListNode =
      if (head == null || head.next == null) head
      else {
        @tailrec
        def lnToBuffer(ln: ListNode, rsf: mutable.ArrayBuffer[Int]): mutable.ArrayBuffer[Int] = {
          if (ln == null) rsf
          else lnToBuffer(ln.next, rsf.addOne(ln.x))
        }
        val buffer = lnToBuffer(head, mutable.ArrayBuffer[Int]())
        buffer.sortInPlace()

        @tailrec
        def bufferToLn(i: Int, rsf: ListNode): ListNode = {
          if (i < 0) rsf
          else bufferToLn(i - 1, new ListNode(buffer(i), rsf))
        }

        bufferToLn(buffer.length - 1, null)
      }
  }

  object SolutionMergeImmutable {
    import scala.annotation.tailrec

    def sortList(head: ListNode): ListNode =
      if (head == null || head.next == null) head
      else {
        val x = size(head) / 2
        merge(sortList(take(x, head)), sortList(drop(x, head)))
      }

    private def take(idx: Int, xs: ListNode): ListNode = {
      val head = new ListNode(xs.x)
      var current = head
      @tailrec
      def loop(i: Int, rest: ListNode): Unit = {
        if (i > 0 && rest != null) {
          current.next = new ListNode(rest.x)
          current = current.next
          loop(i - 1, rest.next)
        }
      }
      loop(idx - 1, xs.next)
      head
    }

    @tailrec
    private def drop(count: Int, xs: ListNode): ListNode = {
      if (xs == null) null
      else if (count > 0) drop(count - 1, xs.next)
      else take(Int.MaxValue, xs)
    }

    @tailrec
    private def size(xs: ListNode, rsf: Int = 0): Int =
      if (xs == null) rsf
      else size(xs.next, rsf + 1)

    private def merge(xs: ListNode, ys: ListNode): ListNode = {
      if (xs == null) ys
      else if (ys == null) xs
      else if (xs.x <= ys.x)
        new ListNode(xs.x, merge(xs.next, ys))
      else
        new ListNode(ys.x, merge(xs, ys.next))
    }
  }

  object SolutionBottomUp {
    private var tail: ListNode = new ListNode()
    private var nextSublist: ListNode = new ListNode()

    def sortList(head: ListNode): ListNode =
      if (head == null || head.next == null) head
      else {
        val n = getSize(head)
        var start: ListNode = head
        val dummyHead: ListNode = new ListNode()
        var size: Int = 1
        while (size < n) {
          tail = dummyHead
          while (start != null && start.next != null) {
            val mid: ListNode = split(start, size)
            merge(start, mid)
            start = nextSublist
          }
          if (start != null && start.next == null) {
            tail.next = start
          }
          start = dummyHead.next

          size = size * 2
        }

        dummyHead.next
      }

    private def split(start: ListNode, size: Int): ListNode = {
      var midPrev = start
      var end: ListNode = start.next
      // use fast and slow approach to find middle and end of second linked list
      var i = 1
      for (_ <- 1 until size if midPrev.next != null || end.next != null) {
        if (end.next != null)
          end = if (end.next.next != null) end.next.next else end.next
        if (midPrev.next != null)
          midPrev = midPrev.next
      }
      val mid = midPrev.next
      midPrev.next = null
      nextSublist = end.next
      end.next = null
      // return the start of second linked list
      mid
    }

    private def merge(xs: ListNode, ys: ListNode): Unit = {
      val dummyHead: ListNode = new ListNode()
      var newTail = dummyHead
      var list1 = xs
      var list2 = ys
      while (list1 != null && list2 != null) {
        if (list1.x < list2.x) {
          newTail.next = list1
          list1 = list1.next
          newTail = newTail.next
        }
        else {
          newTail.next = list2
          list2 = list2.next
          newTail = newTail.next
        }
      }
      newTail.next = if (list1 != null) list1 else list2
      // traverse till the end of merged list to get the newTail
      while (newTail.next != null)
        newTail = newTail.next
      // link the old tail with the head of merged list
      tail.next = dummyHead.next
      // update the old tail to the new tail of merged list
      tail = newTail
    }

    private def getSize(head: ListNode): Int = {
      var cnt = 0
      var ptr = head
      while (ptr != null) {
        ptr = ptr.next
        cnt += 1
      }
      cnt
    }
  }

}
