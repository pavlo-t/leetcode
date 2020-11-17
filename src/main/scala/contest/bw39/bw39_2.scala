package contest.bw39

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

/**
 * [[https://leetcode.com/contest/biweekly-contest-39/problems/minimum-deletions-to-make-string-balanced/]]
 */
//noinspection DuplicatedCode
class bw39_2 extends AnyWordSpec with Matchers {
  /**
   * === 5551. Minimum Deletions to Make String Balanced ===
   *
   * You are given a string `s` consisting only of characters `'a'` and `'b'`.
   *
   * You can delete any number of characters in `s` to make `s` '''balanced'''.
   * `s` is '''balanced''' if there is no pair of indices `(i,j)` such that `i < j` and `s[i] = 'b'` and `s[j]= 'a'`.
   *
   * Return ''the '''minimum''' number of deletions needed to make ''`s`'' '''balanced'''''.
   *
   * '''Constraints:'''
   *   - `1 <= s.length <= 100_000`
   *   - `s[i]` is `'a'` or `'b'`.
   */
  object Solution {
    def minimumDeletions(s: String): Int = {
      @scala.annotation.tailrec
      def loop(s: List[Char], as: Int, bs: Int, rsf: Int): Int = s match {
        case Nil         => rsf min bs
        case 'b' :: tail => loop(tail, as, bs + 1, rsf min (as + bs))
        case _ :: tail   => loop(tail, as - 1, bs, rsf)
      }
      loop(s.toList, s.count(_ == 'a'), 0, Int.MaxValue)
    }
  }
  object SolutionRecIdx {
    def minimumDeletions(s: String): Int = {
      @scala.annotation.tailrec
      def loop(i: Int, as: Int, bs: Int, rsf: Int): Int = {
        if (i >= s.length) rsf min bs
        else if (s(i) == 'b') loop(i + 1, as, bs + 1, rsf min (as + bs))
        else loop(i + 1, as - 1, bs, rsf)
      }
      loop(0, s.count(_ == 'a'), 0, Int.MaxValue)
    }
  }
  object SolutionRecString {
    def minimumDeletions(s: String): Int = {
      @scala.annotation.tailrec
      def loop(s: String, as: Int, bs: Int, rsf: Int): Int = {
        if (s.isEmpty) rsf min bs
        else if (s.head == 'b') loop(s.tail, as, bs + 1, rsf min (as + bs))
        else loop(s.tail, as - 1, bs, rsf)
      }
      loop(s, s.count(_ == 'a'), 0, Int.MaxValue)
    }
  }
  object SolutionImperative {
    def minimumDeletions(s: String): Int = {
      var result = Int.MaxValue
      var as = s.count(_ == 'a')
      var bs = 0
      for (c <- s) {
        if (c == 'b') {
          result = result min (bs + as)
          bs += 1
        } else {
          as -= 1
        }
      }

      result min bs
    }
  }

  object SolutionBruteForceIdxTodoListBreadthFirst {
    import collection.mutable

    def minimumDeletions(s: String): Int = {
      @scala.annotation.tailrec
      def loop(todo: mutable.Queue[((Int, Int), Int)]): Int = {
        val ((l, r), rsf) = todo.dequeue

        val firstB = s.indexOf('b', l)
        val lastA = s.lastIndexOf('a', r)
        if (firstB == -1 || lastA == -1 || lastA < firstB) rsf
        else {
          todo.enqueue(((firstB + 1, lastA), rsf + 1))
          todo.enqueue(((firstB, lastA - 1), rsf + 1))

          loop(todo)
        }
      }
      loop(mutable.Queue(((0, s.length), 0)))
    }
  }
  object SolutionBruteForceIdxWithCache {
    def minimumDeletions(s: String): Int = {
      val cache = collection.mutable.Map[(Int, Int), Int]()
      def loop(l: Int, r: Int, rsf: Int = 0): Int = {
        if (cache.contains((l, r))) cache((l, r))
        else {
          val firstB = s.indexOf('b', l)
          val lastA = s.lastIndexOf('a', r)
          if (firstB == -1 || lastA == -1 || lastA < firstB) {
            cache((l, r)) = rsf
            cache((l, r))
          } else {
            cache((l, r)) = loop(firstB + 1, lastA, rsf + 1) min loop(firstB, lastA - 1, rsf + 1)
            cache((l, r))
          }
        }
      }
      loop(0, s.length)
    }
  }
  object SolutionBruteForceIdx {
    def minimumDeletions(s: String): Int = {
      def loop(l: Int, r: Int, rsf: Int = 0): Int = {
        val firstB = s.indexOf('b', l)
        val lastA = s.lastIndexOf('a', r)
        if (firstB == -1 || lastA == -1 || lastA < firstB)
          rsf
        else
          loop(firstB + 1, lastA, rsf + 1) min loop(firstB, lastA - 1, rsf + 1)
      }
      loop(0, s.length)
    }
  }
  object SolutionBruteForceWithCache {
    def minimumDeletions(s: String): Int = {
      val cache = collection.mutable.Map[String, Int]()
      def loop(s: String, rsf: Int = 0): Int = {
        if (cache.contains(s)) cache(s)
        else {
          val firstB = s.indexOf('b')
          val lastA = s.lastIndexOf('a')
          if (firstB == -1 || lastA == -1 || lastA < firstB) {
            cache(s) = rsf
            cache(s)
          } else {
            cache(s) = loop(s.substring(firstB + 1), rsf + 1) min loop(s.substring(0, lastA), rsf + 1)
            cache(s)
          }
        }
      }
      loop(s)
    }
  }
  object SolutionBruteForceRecursion {
    def minimumDeletions(s: String): Int = {
      def loop(s: String, rsf: Int = 0): Int = {
        val firstB = s.indexOf('b')
        val lastA = s.lastIndexOf('a')
        if (firstB == -1 || lastA == -1 || lastA < firstB)
          rsf
        else
          loop(s.substring(firstB + 1), rsf + 1) min loop(s.substring(0, lastA), rsf + 1)
      }
      loop(s)
    }
  }

  import Solution.minimumDeletions

  """Example 1: (s = "aababbab") -> 2""" in {
    minimumDeletions("aababbab") shouldBe 2
    //Explanation: You can either:
    // Delete the characters at 0-indexed positions 2 and 6 ("aababbab" -> "aaabbb"), or
    // Delete the characters at 0-indexed positions 3 and 6 ("aababbab" -> "aabbbb").
  }
  """Example 2: (s = "bbaaaaabb") -> 2""" in {
    minimumDeletions("bbaaaaabb") shouldBe 2
    //Explanation: The only solution is to delete the first two characters.
  }

  """(s = "a") -> 0""" in {
    minimumDeletions("a") shouldBe 0
  }
  """(s = "b") -> 0""" in {
    minimumDeletions("b") shouldBe 0
  }
  """(s = "aa") -> 0""" in {
    minimumDeletions("aa") shouldBe 0
  }
  """(s = "bb") -> 0""" in {
    minimumDeletions("bb") shouldBe 0
  }
  """(s = "ab") -> 0""" in {
    minimumDeletions("ab") shouldBe 0
  }
  """(s = "ba") -> 1""" in {
    minimumDeletions("ba") shouldBe 1
  }
  """(s = "aaaaabb") -> 0""" in {
    minimumDeletions("aaaaabb") shouldBe 0
  }
  """(s = 10_000) -> >= 0""" in {
    val s = (1 to 10_000).map(i => if (i % 2 == 0) 'a' else 'b').mkString("")
    minimumDeletions(s) shouldBe 5000
  }
  """(s = 100_000) -> >= 0""" in {
    val s = (1 to 100_000).map(i => if (i % 2 == 0) 'a' else 'b').mkString("")
    minimumDeletions(s) shouldBe 50000
  }
}
