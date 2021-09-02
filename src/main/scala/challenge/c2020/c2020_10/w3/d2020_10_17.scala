package challenge.c2020.c2020_10.w3

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

// noinspection DuplicateCode
class d2020_10_17 extends AnyWordSpec with Matchers {

  /**
   * <h3>Repeated DNA Sequences</h3>
   *
   * All DNA is composed of a series of nucleotides abbreviated as `'A'`, `'C'`, `'G'`, and `'T'`,
   * for example: `"ACGAATTCCG"`.
   * When studying DNA, it is sometimes useful to identify repeated sequences within the DNA.
   *
   * Write a function to find all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule.
   *
   * <b>Constraints:</b><ul>
   * <li> `0 <= s.length <= 100000`
   * <li> `s[i]` is `'A'`, `'C'`, `'G'`, or `'T'`.
   * </ul>
   */
  object Solution {
    import collection.mutable

    private val Size = 10
    private val dnaBaseToInt: Char => Int = Map('A' -> 0, 'C' -> 1, 'G' -> 2, 'T' -> 3)

    def findRepeatedDnaSequences(s: String): List[String] = {
      if (s.length < Size + 1) List()
      else {
        val ss = mutable.Set[Int]()
        val result = mutable.Set[String]()
        var current = 0
        for (i <- 0 until Size) {
          current = current << 2 | dnaBaseToInt(s(i))
        }
        ss += current
        for (i <- Size until s.length) {
          // 262143 = (2 << 17) - 1
          current = ((current & 262143) << 2) | dnaBaseToInt(s(i))
          if (ss.contains(current)) result += s.substring(i - Size + 1, i + 1)
          else ss += current
        }
        result.toList
      }
    }
  }
  object Solution2StringSets {
    import collection.mutable

    def findRepeatedDnaSequences(s: String): List[String] = {
      val Size = 10

      if (s.length < Size + 1) List()
      else {
        val ss = mutable.Set[String]()
        val result = mutable.Set[String]()
        for (i <- 0 to (s.length - Size)) {
          val str = s.substring(i, i + Size)
          if (ss.contains(str))
            result += str
          else ss += str
        }
        result.toList
      }
    }
  }
  object SolutionWithMap {
    def findRepeatedDnaSequences(s: String): List[String] = {
      if (s.length < 11) List()
      else {
        val counts = collection.mutable.Map[String, Int]().withDefaultValue(0)
        for (i <- 0 to (s.length - 10)) {
          val ss = s.substring(i, i + 10)
          counts(ss) += 1
        }
        counts.filterInPlace((_, c) => c > 1)
        counts.keys.toList
      }
    }
  }

  import Solution.findRepeatedDnaSequences

  """Example 1: ("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT") -> ["AAAAACCCCC","CCCCCAAAAA"]""" in {
    val s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
    val expected = List("AAAAACCCCC", "CCCCCAAAAA")
    findRepeatedDnaSequences(s) shouldBe expected
  }
  """Example 2: ("AAAAAAAAAAAAA") -> ["AAAAAAAAAA"]""" in {
    val s = "AAAAAAAAAAAAA"
    val expected = List("AAAAAAAAAA")
    findRepeatedDnaSequences(s) shouldBe expected
  }

  """("") -> []""" in {
    findRepeatedDnaSequences("") shouldBe List()
  }
  """("A" * 10) -> []""" in {
    findRepeatedDnaSequences("A" * 10) shouldBe List()
  }
  """("A" * 100000) -> ["AAAAAAAAAA"]""" in {
    val expected = List("AAAAAAAAAA")
    findRepeatedDnaSequences("A" * 100000) shouldBe expected
  }

  //"""a test""" in {
  //  val sdf = collection.immutable.
  //}

}
