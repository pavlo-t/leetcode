package c2020_04.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d06 extends AnyWordSpec with Matchers {

  /**
   * Group Anagrams
   *
   * Given an array of strings `strs`, group <b>the anagrams</b> together.
   * You can return the answer in <b>any order</b>.
   *
   * An <b>Anagram</b> is a word or phrase formed by rearranging the letters of a different word or phrase,
   * typically using all the original letters exactly once.
   *
   *
   * <b>Constraints:</b><ul>
   * <li> <code>1 <= strs.length <= 10<sup>4</sup></code>
   * <li> `0 <= strs[i].length <= 100`
   * <li> `strs[i]` consists of lower-case English letters.
   */
  object Solution {
    import collection.mutable

    def groupAnagrams1(strs: Array[String]): List[List[String]] = {
      val result = mutable.Map[String, mutable.ListBuffer[String]]()

      for (s <- strs) {
        val k = s.toSeq.sorted.unwrap

        if (result.contains(k)) result(k).addOne(s)
        else result.update(k, mutable.ListBuffer(s))
      }

      result.values.map(_.toList).toList
    }

    def groupAnagrams(strs: Array[String]): List[List[String]] = {
      val result = mutable.Map[String, mutable.ListBuffer[String]]()
      val counts = Array.ofDim[Int](26)

      for (s <- strs) {
        counts.mapInPlace(_ => 0)
        for (c <- s) counts(c - 'a') += 1
        val sb = new mutable.StringBuilder
        for (c <- counts) sb.addAll("#" + c)
        val k = sb.toString()

        if (result.contains(k)) result(k).addOne(s)
        else result.update(k, mutable.ListBuffer(s))
      }

      result.values.map(_.toList).toList
    }
  }

  "Example 1" in {
    val strs = Array("eat", "tea", "tan", "ate", "nat", "bat")
    val expected = Set(Set("eat", "tea", "ate"), Set("bat"), Set("tan", "nat"))
    Solution.groupAnagrams(strs).map(_.toSet).toSet shouldBe expected
  }
  "Example 2" in {
    val strs = Array("")
    val expected = List(List(""))
    Solution.groupAnagrams(strs) shouldBe expected
  }
  "Example 3" in {
    val strs = Array("a")
    val expected = List(List("a"))
    Solution.groupAnagrams(strs) shouldBe expected
  }

  "My test: max size" in {
    val length = 40000
    val arr = Array.ofDim[String](length)
    for (i <- 0 until length) arr(i) = util.Random.nextString(util.Random.nextInt(100))

    Solution.groupAnagrams(arr).size should be <= 40000
  }
}
