package c2020_10.w4

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class d2020_10_25 extends AnyWordSpec with Matchers {

  /**
   * <h3>Stone Game IV</h3>
   *
   * Alice and Bob take turns playing a game, with Alice starting first.
   *
   * Initially, there are `n` stones in a pile.
   * On each player's turn, that player makes a move consisting of removing <b>any</b> non-zero <b>square number</b>
   * of stones in the pile.
   *
   * Also, if a player cannot make a move, he/she loses the game.
   *
   * Given a positive integer `n`.
   * Return `True` if and only if Alice wins the game otherwise return `False`, assuming both players play optimally.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= n <= 100_000`
   * </ul>
   */
  object Solution {
    import collection.mutable

    def winnerSquareGame(n: Int): Boolean = {
      val dp = mutable.Map[Int, Boolean]().withDefaultValue(false)
      for (i <- 0 to n) {
        if (!dp(i)) {
          var k = 1
          while ((i + k * k) <= n) {
            dp(i + k * k) = true

            k += 1
          }
        }
      }
      dp(n)
    }
  }
  object SolutionMy {
    import scala.annotation.tailrec
    import collection.mutable

    def winnerSquareGame(n: Int): Boolean = {
      val results = mutable.Map[Int, Boolean](0 -> false, 1 -> true, 2 -> false)

      //def isSquare(i: Int): Boolean = {
      //  val rt = math.sqrt(i)
      //  (rt - rt.toInt) == 0
      //}
      //def getSquares(i: Int): Seq[Int] = {
      //  (1 to i).filter(isSquare)
      //}

      //def getSquares(i: Int): Seq[Int] = {
      //  if (i == 1)  Seq(i)
      //  else {
      //    val rt = math.sqrt(i).toInt
      //    1 to rt
      //  }
      //}
      //
      //def getSquares(i: Int): Seq[Int] = {
      //  if (i == 1) Seq(i)
      //  else if (squares.contains(i)) squares(i)
      //  else {
      //    val rt = math.sqrt(i)
      //    if ((rt - rt.toInt) == 0) {
      //      squares(i) = Seq(i) ++ getSquares(i - 1)
      //      squares(i)
      //    } else {
      //      squares(i) = getSquares(i - 1)
      //      squares(i)
      //    }
      //  }
      //}

      @tailrec
      def getSquares(i: Int, rsf: Seq[Int] = Seq()): Seq[Int] = {
        if (i == 1) rsf.appended(i)
        else {
          val rt = math.sqrt(i)
          if ((rt - rt.toInt) == 0) getSquares(i - 1, rsf.appended(i))
          else getSquares(i - 1, rsf)
        }
      }

      //def loop(i: Int): Boolean = {
      //  if (results.contains(i)) results(i)
      //  else {
      //    val squares = getSquares(i)
      //    if (squares.contains(i)) true
      //    else {
      //      results(i) = squares.exists(j => !loop(i - j))
      //      results(i)
      //    }
      //  }
      //}

      for (i <- 1 to n) {
        results(i) = loop(i)
      }

      def loop(i: Int): Boolean = {
        if (results.contains(i)) results(i)
        else {
          val squares = getSquares(i)
          if (squares.contains(i)) true
          else {
            results(i) = squares.exists(j => !loop(i - j))
            results(i)
          }
        }
      }

      results(n)
    }
  }

  import Solution.winnerSquareGame

  "Example 1: (1) => true" in {
    winnerSquareGame(1) shouldBe true
    // Explanation: Alice can remove 1 stone winning the game because Bob doesn't have any moves.
  }
  "Example 2: (2) => false" in {
    winnerSquareGame(2) shouldBe false
    // Explanation: Alice can only remove 1 stone, after that Bob removes the last one winning the game (2 -> 1 -> 0).
  }
  "Example 3: (4) = true" in {
    winnerSquareGame(4) shouldBe true
    //Explanation: n is already a perfect square, Alice can win with one move, removing 4 stones (4 -> 0).
  }
  "Example 4: (7) -> false" in {
    winnerSquareGame(7) shouldBe false
    // Explanation: Alice can't win the game if Bob plays optimally.
    //   If Alice starts removing 4 stones, Bob will remove 1 stone then Alice should remove only 1 stone and
    //   finally Bob removes the last one (7 -> 3 -> 2 -> 1 -> 0).
    //   If Alice starts removing 1 stone, Bob will remove 4 stones then Alice only can remove 1 stone and
    //   finally Bob removes the last one (7 -> 6 -> 2 -> 1 -> 0).
  }
  "Example 5: (17) -> false" in {
    winnerSquareGame(17) shouldBe false
    //Explanation: Alice can't win the game if Bob plays optimally.
  }

  "Test 64: (9520) => true" in {
    winnerSquareGame(9520) shouldBe true
  }
  "Test 71: (74497) => true" in {
    winnerSquareGame(74497) shouldBe false
  }

  "(3) => true" in {
    winnerSquareGame(3) shouldBe true
  }
  "(100000) => true" in {
    winnerSquareGame(100000) shouldBe true
  }
}
