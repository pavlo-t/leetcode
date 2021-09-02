package challenge.c2020.c2020_04.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d2020_04_09 extends AnyWordSpec with Matchers {

  /**
   * Backspace String Compare
   *
   * Given two strings `S` and `T`, return if they are equal when both are typed into empty text editors.
   * `#` means a backspace character.
   *
   * Note that after backspacing an empty text, the text will continue empty.
   *
   *
   * <b>Note:</b><ul>
   * <li> `1 <= S.length <= 200`
   * <li> `1 <= T.length <= 200`
   * <li> `S` and `T` only contain lowercase letters and `'#'` characters.
   * </ul>
   *
   * <b>Follow up:</b><ul>
   * <li> Can you solve it in `O(N)` time and `O(1)` space?
   * </ul>
   */
  //noinspection DuplicatedCode
  object Solution {
    def backspaceCompare(S: String, T: String): Boolean = {
      def bsp(s: String) = {
        val buffer = collection.mutable.ListBuffer[Char]()
        for (c <- s) {
          if (c == '#') buffer.dropRightInPlace(1)
          else buffer.addOne(c)
        }
        buffer.toList
      }
      bsp(S) == bsp(T)
    }

    def backspaceCompare2(S: String, T: String): Boolean = {
      import scala.annotation.tailrec

      @tailrec
      def nextChar(si: Iterator[Char], bs: Int = 0): Option[Char] =
        si.nextOption() match {
          case None              => None
          case Some('#')         => nextChar(si, bs + 1)
          case Some(_) if bs > 0 => nextChar(si, bs - 1)
          case s                 => s
        }

      @tailrec
      def loop(s: Iterator[Char], t: Iterator[Char]): Boolean = {
        (nextChar(s), nextChar(t)) match {
          case (None, None)     => true
          case (l, r) if l != r => false
          case _                => loop(s, t)
        }
      }

      loop(S.reverseIterator, T.reverseIterator)
    }

    def backspaceCompare3(S: String, T: String): Boolean = {
      import scala.annotation.tailrec

      var iS = S.length - 1
      var iT = T.length - 1
      var skip = 0

      var result = true

      @tailrec
      def nextSChar(): Unit =
        if (iS >= 0) {
          if (S(iS) == '#') {
            iS -= 1
            skip += 1
            nextSChar()
          } else if (skip > 0) {
            iS -= 1
            skip -= 1
            nextSChar()
          }
        }

      @tailrec
      def nextTChar(): Unit =
        if (iT >= 0) {
          if (T(iT) == '#') {
            iT -= 1
            skip += 1
            nextTChar()
          } else if (skip > 0) {
            iT -= 1
            skip -= 1
            nextTChar()
          }
        }

      while (result && (iS >= 0 || iT >= 0)) {
        nextSChar()
        nextTChar()
        if (result && ((iS >= 0 != iT >= 0) || (iS >= 0 && S(iS) != T(iT))))
          result = false

        iS -= 1
        iT -= 1
      }

      result
    }
  }

  "Example 1" in {
    Solution.backspaceCompare("ab#c", "ad#c") shouldBe true
    //Explanation: Both S and T become "ac".
  }
  "Example 2" in {
    Solution.backspaceCompare("ab##", "c#d#") shouldBe true
    //Explanation: Both S and T become "".
  }
  "Example 3" in {
    Solution.backspaceCompare("a##c", "#a#c") shouldBe true
    //Explanation: Both S and T become "c".
  }
  "Example 4" in {
    Solution.backspaceCompare("a#c", "b") shouldBe false
    //Explanation: S becomes "c" while T becomes "b".
  }

  "Test 104" in {
    Solution.backspaceCompare("bxj##tw", "bxj###tw") shouldBe false
    //Explanation: S becomes "c" while T becomes "b".
  }
}
