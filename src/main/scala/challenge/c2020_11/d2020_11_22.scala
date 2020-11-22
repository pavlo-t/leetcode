package challenge.c2020_11

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/challenge/card/november-leetcoding-challenge/567/week-4-november-22nd-november-28th/3540/]]
 */
//noinspection DuplicatedCode
class d2020_11_22 extends AnyWordSpec with Matchers {

  /**
   * === Unique Morse Code Words ===
   *
   * International Morse Code defines a standard encoding where each letter is mapped to a series of dots and dashes,
   * as follows: `"a"` maps to `".-"`, `"b"` maps to `"-..."`, `"c"` maps to `"-.-."`, and so on.
   *
   * For convenience, the full table for the 26 letters of the English alphabet is given below:
   * {{{
   * {
   *   "a" -> ".-",
   *   "b" -> "-...",
   *   "c" -> "-.-.",
   *   "d" -> "-..",
   *   "e" -> ".",
   *   "f" -> "..-.",
   *   "g" -> "--.",
   *   "h" -> "....",
   *   "i" -> "..",
   *   "j" -> ".---",
   *   "k" -> "-.-",
   *   "l" -> ".-..",
   *   "m" -> "--",
   *   "n" -> "-.",
   *   "o" -> "---",
   *   "p" -> ".--.",
   *   "q" -> "--.-",
   *   "r" -> ".-.",
   *   "s" -> "...",
   *   "t" -> "-",
   *   "u" -> "..-",
   *   "v" -> "...-",
   *   "w" -> ".--",
   *   "x" -> "-..-",
   *   "y" -> "-.--",
   *   "z" -> "--.."
   * }
   * }}}
   *
   * Now, given a list of words, each word can be written as a concatenation of the Morse code of each letter.
   * For example, `"cab"` can be written as `"-.-..--..."`, (which is the concatenation `"-.-."` + `".-"` + `"-..."`).
   * We'll call such a concatenation, the transformation of a word.
   *
   * Return the number of different transformations among all words we have.
   *
   * '''Note:'''
   *  - The length of `words` will be at most `100`.
   *  - Each `words[i]` will have length in range `[1, 12]`.
   *  - `words[i]` will only consist of lowercase letters.
   */
  object Solution {
    private val mcEngLettersLc = Seq(".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
      "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..")
    def toMorseCode(w: String): String = w.flatMap(c => mcEngLettersLc(c - 'a'))

    def uniqueMorseRepresentations(words: Array[String]): Int =
      words.toSet.map(toMorseCode).size
  }

  import Solution.uniqueMorseRepresentations

  "toMorseCode(gin) -> --...-." in {
    Solution.toMorseCode("gin") shouldBe "--...-."
  }

  """Example: (words = ["gin", "zen", "gig", "msg"]) -> 2""" in {
    uniqueMorseRepresentations(Array("gin", "zen", "gig", "msg")) shouldBe 2
    //Explanation:
    //The transformation of each word is:
    //"gin" -> "--...-."
    //"zen" -> "--...-."
    //"gig" -> "--...--."
    //"msg" -> "--...--."
    //
    //There are 2 different transformations, "--...-." and "--...--.".
  }

  """(words: []) -> 0""" in {
    uniqueMorseRepresentations(Array()) shouldBe 0
  }

  """(words: [100 "z" * 12 words]) -> 1""" in {
    val words = Array.fill(100)("z" * 12)
    Thread.sleep(30)
    println(words.mkString("words: [\"", "\",\"", "\"]"))

    val result = uniqueMorseRepresentations(words)
    println(s"  result: $result")

    result shouldBe 1
  }
  """(words: [100 random words]) -> ???""" in {
    val words = Array.fill(100) {
      val sb = new StringBuilder
      for (_ <- 1 to util.Random.nextInt(12) + 1) sb += (util.Random.nextInt(26) + 'a').toChar
      sb.toString()
    }
    Thread.sleep(30)
    println(words.mkString("words: [\"", "\",\"", "\"]"))

    val result = uniqueMorseRepresentations(words)
    println(s"  result: $result")

    result should be >= 1
    result should be <= 100
  }

}
