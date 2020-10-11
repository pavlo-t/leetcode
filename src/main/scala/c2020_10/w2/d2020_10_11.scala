package c2020_10.w2

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


//noinspection DuplicatedCode
class d2020_10_11 extends AnyWordSpec with Matchers {

  /**
   * Remove Duplicate Letters
   *
   * Given a string `s`, remove duplicate letters so that every letter appears once and only once.
   * You must make sure your result is <b>the smallest in lexicographical order</b> among all possible results.
   *
   * <b>Note</b>: This question is the same as 1081: [[https://leetcode.com/problems/smallest-subsequence-of-distinct-characters/]]
   *
   *
   * <b>Constraints:</b><ul>
   * <li> <code>1 <= s.length <= 10<sup>4</sup></code>
   * <li> `s` consists of lowercase English letters.
   * </ul>
   */
  object Solution {
    import scala.annotation.tailrec

    def removeDuplicateLetters1(s: String): String = {
      val allChars = s.toCharArray.toSet.toSeq.sorted

      @tailrec
      def addOne(data: String, todo: Seq[Char], rsf: String): String = todo match {
        case c :: rest =>
          val cIdx = data.indexOf(c)
          val nextData = data.substring(cIdx + 1)
          if (rest.forall(nextData.contains(_)))
            loop(nextData, rest.sorted, rsf + c)
          else
            addOne(data, rest :+ c, rsf)
      }

      def loop(data: String, todo: Seq[Char], rsf: String): String = {
        todo match {
          case Nil      => rsf
          case c :: Nil => rsf + c
          case _        => addOne(data, todo, rsf)
        }
      }

      loop(s, allChars, "")
    }

    def removeDuplicateLetters3(s: String): String = {
      import collection.mutable

      val allChars = mutable.ListBuffer.from(s.toCharArray.toSet).sorted

      @tailrec
      def loop(str: String, todo: mutable.ListBuffer[Char], rsf: String): String = {
        if (todo.isEmpty) rsf
        else if (todo.size == 1) rsf + todo.head
        else {
          val cIdx = str.indexOf(todo.head)
          val nextStr = str.substring(cIdx + 1)
          if (todo.tail.forall(nextStr.contains(_)))
            loop(nextStr, todo.tail.sorted, rsf + todo.head)
          else
            loop(str, todo.tail.addOne(todo.head), rsf)

        }
      }

      loop(s, allChars, "")
    }

    def removeDuplicateLetters(s: String): String = {
      @tailrec
      def loop(str: String, todo: Seq[Char], rsf: String): String =
        todo match {
          case Nil       => rsf
          case c :: Nil  => rsf + c
          case c :: rest =>
            val cIdx = str.indexOf(c)
            val nextStr = str.substring(cIdx + 1)
            if (rest.forall(nextStr.contains(_)))
              loop(nextStr, rest.sorted, rsf + c)
            else
              loop(str, rest :+ c, rsf)
        }

      loop(s, s.toSet.toSeq.sorted, "")
    }
  }


  "Example 1" in {
    Solution.removeDuplicateLetters("bcabc") shouldBe "abc"
  }
  "Example 2" in {
    Solution.removeDuplicateLetters("cbacdcbc") shouldBe "acdb"
  }

  "Test 3" in {
    Solution.removeDuplicateLetters("cdadabcc") shouldBe "adbc"
  }
  "Test 37" in {
    Solution.removeDuplicateLetters("aabaa") shouldBe "ab"
  }
  "My test: max size" in {
    import util.Random

    val length = 10000
    val sb = new StringBuilder
    (1 to length).foreach(_ => sb.addOne((Random.nextInt('z' - 'a' + 1) + 'a').toChar))

    Solution.removeDuplicateLetters(sb.toString()).length should be <= 26
  }
}
