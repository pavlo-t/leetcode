package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

import scala.io.Source

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/567/week-4-november-22nd-november-28th/3542/]]
 */
//noinspection DuplicatedCode
class d2020_11_24 extends AnyWordSpec with Matchers {

  /**
   * === Basic Calculator II ===
   *
   * Implement a basic calculator to evaluate a simple expression string.
   *
   * The expression string contains only '''non-negative''' integers, `+`, `-`, `*`, `/` operators and empty spaces ` `.
   * The integer division should truncate toward zero.
   *
   * '''Note:'''
   *  - You may assume that the given expression is always valid.
   *  - '''Do not''' use the `eval` built-in library function.
   */
  object Solution {
    def calculate(s: String): Int = if (s == null) 0 else {
      @scala.annotation.tailrec
      def calculate(i: Int, a: Int, op: Char, b: Int, rsf: Int): Int = {
        if (i >= s.length) rsf + (op match {
          case '+' => a + b
          case '-' => a - b
          case '*' => a * b
          case '/' => a / b
        })
        else {
          s(i) match {
            case ' ' => calculate(i + 1, a, op, b, rsf)

            case d if d.isDigit => calculate(i + 1, a, op, b * 10 + (d - '0'), rsf)

            case nop if op == '+' => calculate(i + 1, b, nop, 0, rsf + a)
            case nop if op == '-' => calculate(i + 1, -b, nop, 0, rsf + a)
            case nop if op == '*' => calculate(i + 1, a * b, nop, 0, rsf)
            case nop if op == '/' => calculate(i + 1, a / b, nop, 0, rsf)

            case c => throw new IllegalStateException(s"Unexpected char: $c; state: $i, $op, $a, $b, $rsf")
          }
        }
      }
      calculate(0, 0, '+', 0, 0)
    }
  }

  object Solution1 {
    def calculate(s: String): Int = {
      @scala.annotation.tailrec
      def calculate(nums: Seq[Int], ops: Seq[Char]): Int =
        if (ops.isEmpty) nums.head
        else {
          ops.indexWhere(c => c == '*' || c == '/') match {
            case -1 =>
              val a = nums.head
              val b = nums.tail.head
              val c = if (ops.head == '+') a + b else a - b
              calculate(c +: nums.tail.tail, ops.tail)
            case i  =>
              val a = nums(i)
              val b = nums(i + 1)
              val c = if (ops(i) == '*') a * b else a / b
              calculate(nums.patch(i, Seq(c), 2), ops.patch(i, Nil, 1))
          }
        }

      val (nums, ops) = s.foldLeft((Seq[Int](0), Seq[Char]())) {
        case (acc, ' ') => acc

        case (acc, d@('0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9')) => acc match {
          case (nums :+ n, ops) if nums.size >= ops.size => (nums :+ (n * 10 + (d - '0')), ops)
          case (nums, ops)                               => (nums :+ (d - '0'), ops)
        }

        case ((nums, ops), op@('+' | '-'))                  => (Seq(calculate(nums, ops)), Seq(op))
        case ((nums :+ a :+ b, ops :+ '*'), op@('*' | '/')) => (nums :+ (a * b), ops :+ op)
        case ((nums :+ a :+ b, ops :+ '/'), op@('*' | '/')) => (nums :+ (a / b), ops :+ op)
        case ((nums, ops), op@('*' | '/'))                  => (nums, ops :+ op)

        case s => throw new IllegalStateException(s"Unexpected state: $s")
      }

      calculate(nums, ops)
    }
  }

  import Solution.calculate

  """Example 1: ("3+2*2") -> 7""" in {
    calculate("3+2*2") shouldBe 7
  }
  """Example 2: (" 3/2 ") -> 1""" in {
    calculate(" 3/2 ") shouldBe 1
  }
  """Example 3: (" 3+5 / 2 ") -> 5""" in {
    calculate(" 3+5 / 2 ") shouldBe 5
  }

  """(" 1 0 / 2 ") -> 5""" in {
    calculate(" 1 0 / 2 ") shouldBe 5
  }

  """("  ") -> 0""" in {
    calculate("  ") shouldBe 0
  }
  """(null) -> 0""" in {
    calculate(null) shouldBe 0
  }
  """(" 10 / 2 ") -> 5""" in {
    calculate(" 10 / 2 ") shouldBe 5
  }
  """(" 10 * 3 / 6 ") -> 5""" in {
    calculate(" 1 0 * 3 / 6 ") shouldBe 5
  }
  """(" 10 / 6 * 3 ") -> 3""" in {
    calculate(" 10 / 6 * 3 ") shouldBe 3
  }
  """("1 + 10 * 3 / 6 - 4") -> 2""" in {
    calculate("1 + 10 * 3 / 6 - 4") shouldBe 2
  }
  """("1 + 10 / 6 * 3 - 5") -> -1""" in {
    calculate("1 + 10 / 6 * 3 - 5") shouldBe -1
  }
  """("1 + 10 - 6 * 3 - 5") -> -12""" in {
    calculate("1 + 10 - 6 * 3 - 5") shouldBe -12
  }

  """(calculation of 1000 numbers) -> ???""" in {
    val opChars = "*+-/ "
    val s = {
      val sb = new StringBuilder
      for (_ <- 1 to 1000) {
        sb.addAll((util.Random.nextInt(100) + 1).toString)
        sb.addOne(opChars(util.Random.nextInt(opChars.length)))
      }
      sb.addAll((util.Random.nextInt(100) + 1).toString)
      sb.toString()
    }

    calculate(s) should be <= Int.MaxValue
  }
  """(very large calculation of 100000 numbers) -> ???""" in {
    val opChars = "*+-/ "
    val s = {
      val sb = new StringBuilder
      for (_ <- 1 to 100000) {
        sb.addAll((util.Random.nextInt(100) + 1).toString)
        sb.addOne(opChars(util.Random.nextInt(opChars.length)))
      }
      sb.addAll((util.Random.nextInt(100) + 1).toString)
      sb.toString()
    }

    calculate(s) should be <= Int.MaxValue
  }

  """Test 109: (very large calculation) -> ???""" in {
    val s = Source.fromResource("challenge/c2020_11/d2020_11_24.txt").getLines().toSeq.head
    calculate(s) shouldBe 199
  }

}
