package c2020_04.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection AccessorLikeMethodIsEmptyParen, DuplicatedCode
class d2020_04_10 extends AnyWordSpec with Matchers {

  /**
   * <h3>Min Stack</h3>
   *
   * Design a stack that supports push, pop, top, and retrieving the minimum element in constant time.
   *
   * <ul>
   * <li> `push(x)` - Push element `x` onto stack.
   * <li> `pop()` - Removes the element on top of the stack.
   * <li> `top()` - Get the top element.
   * <li> `getMin()` - Retrieve the minimum element in the stack.
   * </ul>
   *
   * <b>Constraints:<b><ul>
   * <li> Methods `pop`, `top` and `getMin` operations will always be called on <b>non-empty</b> stacks.
   * </ul>
   *
   * Your MinStack object will be instantiated and called as such:
   *
   * <code><pre>
   * var obj = new MinStack()
   * obj.push(x)
   * obj.pop()
   * var param_3 = obj.top()
   * var param_4 = obj.getMin()
   * </pre></code>
   */
  object MinStack {
    def apply(): MinStack = new MinStack()
    def apply(xs: Int*): MinStack = {
      val ms = new MinStack()
      for (x <- xs) ms.push(x)
      ms
    }
  }

  class MinStackKeepMinInNode() {
    private case class Node(head: Int, min: Int, tail: Option[Node])

    private var node: Option[Node] = None

    def push(x: Int): Unit = node match {
      case None                  => node = Some(Node(x, x, None))
      case n@Some(Node(_, m, _)) => node = Some(Node(x, x min m, n))
    }
    def pop(): Unit = node = node.get.tail
    def top(): Int = node.get.head
    def getMin(): Int = node.get.min
  }
  class MinStackLinearPop() {
    import scala.annotation.tailrec

    private trait Node
    private case object Empty extends Node
    private case class Value(x: Int, prev: Node) extends Node {
      def min: Int = {
        @tailrec
        def loop(rest: Node, rsf: Int): Int = rest match {
          case Empty          => rsf
          case Value(y, prev) => loop(prev, rsf min y)
        }
        loop(prev, x)
      }
    }

    private var stack: Node = Empty
    private var min: Int = Int.MaxValue

    def push(x: Int): Unit = {
      stack = Value(x, stack)
      if (x < min) min = x
    }
    def pop(): Unit = {
      stack match {
        case Empty                      =>
          throw new UnsupportedOperationException("Calling pop() on an empty stack")
        case Value(_, Empty)            =>
          min = Int.MaxValue
          stack = Empty
        case Value(_, prev@Value(_, _)) =>
          min = prev.min
          stack = prev
      }
    }
    def top(): Int = stack match {
      case Empty       =>
        throw new UnsupportedOperationException("Calling top() on an empty stack")
      case Value(x, _) => x
    }
    def getMin(): Int = min
  }
  class MinStackConstantSpace() {
    import collection.mutable

    private var min: Long = 0
    private val stack = mutable.Stack[Long]()

    def push(x: Int): Unit = {
      if (stack.isEmpty) {
        stack.push(x)
        min = x
      } else if (x < min) {
        stack.push(x * 2L - min)
        min = x
      } else stack.push(x)
    }
    def pop(): Unit = {
      val x = stack.pop()
      if (x < min)
        min = 2 * min - x
    }
    def top(): Int = {
      val t = stack.top
      if (t < min) min.toInt
      else t.toInt
    }
    def getMin(): Int = min.toInt
  }
  class MinStack() {
    import collection.mutable

    private type Min = Int
    private val stack = mutable.Stack[(Int, Min)]()

    def push(x: Int): Unit =
      if (stack.isEmpty) stack.push((x, x))
      else stack.push((x, x min stack.top._2))
    def pop(): Unit = stack.pop()
    def top(): Int = stack.top._1
    def getMin(): Min = stack.top._2
  }

  "Example 1: " in {
    val minStack = MinStack()

    minStack.push(-2)
    minStack.push(0)
    minStack.push(-3)

    minStack.getMin() shouldBe -3
    minStack.pop()
    minStack.top() shouldBe 0
    minStack.getMin() shouldBe -2
  }

  "Test 10: " in {
    val minStack = MinStack()

    minStack.push(2147483646)
    minStack.push(2147483646)
    minStack.push(2147483647)

    minStack.top() shouldBe 2147483647
    minStack.pop()
    minStack.getMin() shouldBe 2147483646
    minStack.pop()
    minStack.getMin() shouldBe 2147483646
    minStack.pop()

    minStack.push(2147483647)

    minStack.top() shouldBe 2147483647
    minStack.getMin() shouldBe 2147483647

    minStack.push(-2147483648)

    minStack.top() shouldBe -2147483648
    minStack.getMin() shouldBe -2147483648

    minStack.pop()

    minStack.getMin() shouldBe 2147483647
  }

  "Test 1_000_000 elements" in {
    val max = 1_000_000
    val minStack = MinStack()

    for (i <- max to 0 by -1) {
      minStack.push(i)

      minStack.top() shouldBe i
      minStack.getMin() shouldBe i
    }

    minStack.top() shouldBe 0
    minStack.getMin() shouldBe 0

    for (i <- 1 to max) {
      minStack.pop()

      minStack.top() shouldBe i
      minStack.getMin() shouldBe i
    }

    minStack.top() shouldBe max
    minStack.getMin() shouldBe max
  }
}
