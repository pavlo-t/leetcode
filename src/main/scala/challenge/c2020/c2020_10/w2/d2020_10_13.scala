package challenge.c2020.c2020_10.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


//noinspection DuplicatedCode
class d2020_10_13 extends AnyWordSpec with Matchers {

  /**
   * <h3>Sort List</h3>
   *
   * Given the `head` of a linked list, return <em>the list after sorting it in <b>ascending order</b></em>.
   *
   * <b>Follow up</b>: Can you sort the linked list in `O(n log n)` time and `O(1)` memory (i.e. constant space)?
   *
   * <b>Constraints:</b><ul>
   * <li> The number of nodes in the list is in the range `[0, 50_000]`.
   * <li> `-100_000 <= Node.val <= 100_000`
   * </ul>
   */
  // Bottom-up merge sort: O(n log n) time, O(1) memory
  object Solution {
    /** O(n log n) time, O(1) space */
    def sortList(head: ListNode): ListNode =
      if (head == null || head.next == null) head
      else {
        val listSize = head.size()

        @scala.annotation.tailrec
        def sort(size: Int, head: ListNode): ListNode =
          if (size >= listSize) head
          else sort(size * 2, splitMerge(head, size))

        sort(1, head)
      }

    /**
     * Split and merge list using chunks of `size`
     *
     * @return [new head]
     */
    private def splitMerge(list: ListNode, size: Int): ListNode = {
      @scala.annotation.tailrec
      def _splitMerge(n: ListNode, head: ListNode = null, prev: ListNode = null): ListNode = {
        if (n == null) head
        else if (n.next == null) {
          prev.next = n
          head
        } else {
          val (mid, nextSublist) = split(n, size)
          val (merged, newPrev) = merge(n, mid)
          if (head == null) _splitMerge(nextSublist, merged, newPrev)
          else {
            prev.next = merged
            _splitMerge(nextSublist, head, newPrev)
          }
        }
      }
      _splitMerge(list)
    }

    /**
     * Split list into 2 chunks of `size`
     *
     * 1. Find `middle` and `end` of the list
     * 2. Disconnect `middle` from the previous node
     * 3. Disconnect `end` from the next node
     *
     * INVARIANT: `start != null && start.next != null`
     *
     * @return ([middle of the list], [next sublist])
     */
    private def split(start: ListNode, size: Int): (ListNode, ListNode) = {
      require(start != null)
      require(start.next != null)

      @scala.annotation.tailrec
      def _findMidPrevWithEnd(i: Int, slow: ListNode, fast: ListNode): (ListNode, ListNode) = {
        if (i <= 1 || slow.next == null) (slow, fast)
        else {
          val nextFast =
            if (fast.next == null) fast
            else if (fast.next.next == null) fast.next
            else fast.next.next
          _findMidPrevWithEnd(i - 1, slow.next, nextFast)
        }
      }

      val (midPrev, end) = _findMidPrevWithEnd(size, start, start.next)

      val mid = midPrev.next
      midPrev.next = null

      val nextSublist = end.next
      end.next = null

      (mid, nextSublist)
    }

    /**
     * Merge 2 sorted lists in place, updating `next` pointers
     *
     * INVARIANT: `list1 != null || list2 != null`
     *
     * @return ([merged list head], [merged list last])
     */
    private def merge(list1: ListNode, list2: ListNode): (ListNode, ListNode) = {
      require(list1 != null || list2 != null)

      @scala.annotation.tailrec
      def _merge(l1: ListNode, l2: ListNode, head: ListNode = null, prev: ListNode = null): (ListNode, ListNode) = {
        if (l1 == null && l2 == null) (head, prev)
        else if (l2 == null && head == null) (l1, l1.last())
        else if (l2 == null) {
          prev.next = l1
          (head, l1.last())
        }
        else if (l1 == null) _merge(l2, l1, head, prev)
        else if (l1.x <= l2.x && head == null) _merge(l1.next, l2, l1, l1)
        else if (l1.x <= l2.x) {
          prev.next = l1
          _merge(l1.next, l2, head, prev.next)
        }
        else _merge(l2, l1, head, prev)
      }
      _merge(list1, list2)
    }

    implicit class ListNodeSize(head: ListNode) {
      /** O(n) time, O(1) space */
      def size(): Int = _size(head, 0)
      @scala.annotation.tailrec
      private def _size(n: ListNode, rsf: Int): Int =
        if (n == null) rsf
        else _size(n.next, rsf + 1)

      /** O(n) time, O(1) space */
      def last(): ListNode = _last(head)
      @scala.annotation.tailrec
      private def _last(current: ListNode): ListNode = {
        if (current == null || current.next == null) current
        else _last(current.next)
      }
    }
  }

  // Top-down merge sort: O(n log n) time, O(log n) memory
  object SolutionTopDown {
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

  // Bottom-up merge sort: O(n log n) time, O(1) memory
  object SolutionBottomUp {
    def sortList(head: ListNode): ListNode =
      if (head == null || head.next == null) head
      else {
        val n = getSize(head)
        var start: ListNode = head
        val dummyHead: ListNode = new ListNode()
        var size: Int = 1
        while (size < n) {
          var tail = dummyHead
          while (start != null && start.next != null) {
            val (mid, nextSublist) = split(start, size)
            val (merged, newTail) = merge(start, mid)
            tail.next = merged
            tail = newTail
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

    private def split(start: ListNode, size: Int): (ListNode, ListNode) = {
      var midPrev = start
      var end: ListNode = start.next
      // use fast and slow approach to find middle and end of second linked list
      for (_ <- 1 until size if midPrev.next != null) {
        if (end.next != null)
          end = if (end.next.next != null) end.next.next else end.next
        midPrev = midPrev.next
      }
      val mid = midPrev.next
      midPrev.next = null
      val nextSublist = end.next
      end.next = null
      // return the start of second linked list and next sublist
      (mid, nextSublist)
    }

    private def merge(xs: ListNode, ys: ListNode): (ListNode, ListNode) = {
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
      (dummyHead.next, newTail)
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

  class ListNode(_x: Int = 0, _next: ListNode = null) {
    var next: ListNode = _next
    var x: Int = _x

    override def toString: String = s"$x,$next"
  }
  private def ln(x: Int, next: ListNode = null) =
    new ListNode(x, next)

  private def arrayToLn(arr: Array[Int]): ListNode = {
    val root = ln(arr(0))
    var current = root

    for (i <- 1 until arr.length) {
      val next = ln(arr(i))
      current.next = next
      current = next
    }

    root
  }

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
  "[1]" in {
    Solution.sortList(ln(1)).toString shouldBe ln(1).toString
  }

  private val MaxSize = 500_000

  s"[random repeat $MaxSize] check only first" in {
    val arr = Array.ofDim[Int](MaxSize)
    for (i <- 1 until MaxSize) arr(i) = util.Random.nextInt(200001) - 100000
    val input = arrayToLn(arr)

    val result = Solution.sortList(input)

    result.x shouldBe arr.min
  }

  s"[random repeat $MaxSize]" in {
    val arr = Array.ofDim[Int](MaxSize)
    for (i <- 1 until MaxSize) arr(i) = util.Random.nextInt(200001) - 100000
    val input = arrayToLn(arr)

    val result = Solution.sortList(input)

    var current = result
    for (x <- arr.sorted) {
      current.x shouldBe x
      current = current.next
    }
    current shouldBe null
  }
  s"[1..$MaxSize] best case" in {
    val arr = (1 to MaxSize).toArray
    val input = arrayToLn(arr)
    val result = Solution.sortList(input)

    var current = result
    for (x <- 1 to MaxSize) {
      current.x shouldBe x
      current = current.next
    }
    current shouldBe null
  }
  s"[$MaxSize..1] worst case" in {
    val arr = (MaxSize to 1 by -1).toArray
    val input = arrayToLn(arr)
    val result = Solution.sortList(input)

    var current = result
    for (x <- 1 to MaxSize) {
      current.x shouldBe x
      current = current.next
    }
    current shouldBe null
  }
}
