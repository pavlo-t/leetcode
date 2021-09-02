package challenge.c2021.c2021_02

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/february-leetcoding-challenge-2021/585/week-2-february-8th-february-14th/3633/]]
 */
//noinspection AccessorLikeMethodIsEmptyParen,DuplicatedCode
class c2021_02_08 extends AnyWordSpec with Matchers {
  /**
   * === Peeking Iterator ===
   *
   * Given an Iterator class interface with methods: `next()` and `hasNext()`,
   * design and implement a PeekingIterator that support the `peek()` operation --
   * it essentially `peek()` at the element that will be returned by the next call to `next()`.
   *
   * '''Follow up:''' How would you extend your design to be generic and work with all types, not just integer?
   *
   * [[https://www.scala-lang.org/api/2.12.2/scala/collection/Iterator.html Scala Iterator reference]]
   */
  class PeekingIterator[T](_iterator: Iterator[T]) {
    private var peeked: Option[T] = None

    def peek(): T = peeked match {
      case None    =>
        this.peeked = Some(_iterator.next())
        this.peeked.get
      case Some(v) => v
    }

    def next(): T = peeked match {
      case None    => _iterator.next()
      case Some(v) =>
        this.peeked = None
        v
    }

    def hasNext(): Boolean = peeked.nonEmpty || _iterator.hasNext
  }

  class PeekingIteratorInt(_iterator: Iterator[Int]) {
    private var peeked: Option[Int] = None

    def peek(): Int = peeked match {
      case None    =>
        this.peeked = Some(_iterator.next())
        this.peeked.get
      case Some(v) => v
    }

    def next(): Int = peeked match {
      case None    => _iterator.next()
      case Some(v) =>
        this.peeked = None
        v
    }

    def hasNext(): Boolean = peeked.nonEmpty || _iterator.hasNext
  }

  /**
   * Your PeekingIterator object will be instantiated and called as such:
   * var obj = new PeekingIterator(arr)
   * var param_1 = obj.next()
   * var param_2 = obj.peek()
   * var param_3 = obj.hasNext()
   */

  "Example: (_iterator = [1,2,3])" in {
    val pi = new PeekingIterator(Iterator(1, 2, 3))

    pi.hasNext() shouldBe true
    pi.next() shouldBe 1
    pi.hasNext() shouldBe true
    pi.peek() shouldBe 2
    pi.hasNext() shouldBe true
    pi.next() shouldBe 2
    pi.hasNext() shouldBe true
    pi.next() shouldBe 3
    pi.hasNext() shouldBe false
    //Assume that the iterator is initialized to the beginning of the list: [1,2,3].
    //
    //Call next() gets you 1, the first element in the list.
    //Now you call peek() and it returns 2, the next element. Calling next() after that still return 2.
    //You call next() the final time and it returns 3, the last element.
    //Calling hasNext() after that should return false.
  }

}
