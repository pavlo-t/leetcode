package challenge.c2021.c2021_01

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/explore/featured/card/january-leetcoding-challenge-2021/580/week-2-january-8th-january-14th/3598/]]
 */
//noinspection DuplicatedCode
class c2021_01_09 extends AnyWordSpec with Matchers {
  /**
   * === Word Ladder ===
   *
   * Given two words `beginWord` and `endWord`, and a dictionary `wordList`,
   * return ''the length of the shortest transformation sequence from'' `beginWord` ''to'' `endWord`, ''such that:''
   *  - Only one letter can be changed at a time.
   *  - Each transformed word must exist in the word list.
   *
   * Return `0` if there is no such transformation sequence.
   *
   * '''Constraints:'''
   *  - `1 <= beginWord.length <= 100`
   *  - `endWord.length == beginWord.length`
   *  - `1 <= wordList.length <= 5000`
   *  - `wordList[i].length == beginWord.length`
   *  - `beginWord`, `endWord`, and `wordList[i]` consist of lowercase English letters.
   *  - `beginWord != endWord`
   *  - All the strings in `wordList` are '''unique'''.
   */
  object Solution {
    def ladderLength(beginWord: String, endWord: String, wordList: List[String]): Int = {
      @scala.annotation.tailrec
      def is1charDiff(fw: String, tw: String, i: Int = 0, diff: Int = 0): Boolean =
        if (i == fw.length) diff == 1
        else if (diff > 1) false
        else is1charDiff(fw, tw, i + 1, if (fw(i) == tw(i)) diff else diff + 1)

      @scala.annotation.tailrec
      def bfs(todo: Seq[(String, Int)], words: Seq[String]): Int = todo match {
        case Nil                        => 0
        case (`endWord`, rsf) :: _      => rsf
        case _ :: rest if words.isEmpty => bfs(rest, words)
        case (w, rsf) :: rest           =>
          val (moreWork, moreWords) = words.partition { tw => is1charDiff(w, tw) }
          bfs(rest ++ moreWork.map((_, rsf + 1)), moreWords)
      }

      bfs(Seq((beginWord, 1)), wordList)
    }
  }

  import Solution.ladderLength

  """Example 1: (beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log","cog"]) -> 5""" in {
    val beginWord = "hit"
    val endWord = "cog"
    val wordList = List("hot", "dot", "dog", "lot", "log", "cog")

    ladderLength(beginWord, endWord, wordList) shouldBe 5
    // Explanation: As one shortest transformation is "hit" -> "hot" -> "dot" -> "dog" -> "cog", return its length 5.
  }
  """Example 2: (beginWord = "hit", endWord = "cog", wordList = ["hot","dot","dog","lot","log"]) -> 0""" in {
    val beginWord = "hit"
    val endWord = "cog"
    val wordList = List("hot", "dot", "dog", "lot", "log")

    ladderLength(beginWord, endWord, wordList) shouldBe 0
    //Explanation: The endWord "cog" is not in wordList, therefore no possible transformation.
  }

  """(beginWord = "a", endWord = "b", wordList = ["b"]) -> 2""" in {
    val beginWord = "a"
    val endWord = "b"
    val wordList = List("b")

    ladderLength(beginWord, endWord, wordList) shouldBe 2
  }
  """(beginWord = "a" * 100, endWord = "b" * 100, wordList = [...] 100) -> 101""" in {
    val beginWord = "a" * 100
    val endWord = "b" * 100
    val wordList = (0 until 100).map(i => "a" * i + "b" * (100 - i)).toList

    ladderLength(beginWord, endWord, wordList) shouldBe 101
  }
  """(beginWord = "a" * 100, endWord = "b" * 100, wordList = [...] 5000) -> 101""" in {
    val beginWord = "a" * 100
    val endWord = "b" * 100
    val wordList =
      (0 until 100).map(i => "a" * i + "b" * (100 - i))
        .flatMap(w => Seq.fill(50)(w))
        .toList

    ladderLength(beginWord, endWord, wordList) shouldBe 101
  }

}
