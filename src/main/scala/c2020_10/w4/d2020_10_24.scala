package c2020_10.w4

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class d2020_10_24 extends AnyWordSpec with Matchers {

  /**
   * <h3>Bag of Tokens</h3>
   *
   * You have an initial <b>power</b> of `P`, an initial <b>score</b> of `0`,
   * and a bag of `tokens` where `tokens[i]` is the value of the `i`th token (0-indexed).
   *
   * Your goal is to maximize your total <b>score</b> by potentially playing each token in one of two ways:<ul>
   * <li>If your current <b>power</b> is at least `tokens[i]`, you may play the `i`th token face up,
   * losing `tokens[i]` <b>power</b> and gaining `1` <b>score</b>.
   * <li>If your current <b>score</b> is at least `1`, you may play the `i`th token face down,
   * gaining `tokens[i]` <b>power</b> and losing `1` <b>score</b>.
   * </ul>
   *
   * Each token may be played <b>at most</b> once and <b>in any order</b>.
   * You do <b>not</b> have to play all the tokens.
   *
   * Return <em>the largest possible <b>score</b> you can achieve after playing any number of tokens</em>.
   *
   * <b>Constraints:</b><ul>
   * <li> `0 <= tokens.length <= 1000`
   * <li> `0 <= tokens[i], P < 10_000`
   * </ul>
   *
   * @see [[https://leetcode.com/problems/bag-of-tokens/solution/]]
   */
  object Solution {
    import scala.annotation.tailrec

    def bagOfTokensScore(tokens: Array[Int], P: Int): Int = {
      tokens.sortInPlace()
      @tailrec
      def loop(f: Int, l: Int, p: Int, s: Int): Int = {
        if (f > l) s
        else if (tokens(f) <= p)
          loop(f + 1, l, p - tokens(f), s + 1)
        else if (f != l && s > 0 && tokens(f) < (p + tokens(l)))
          loop(f, l - 1, p + tokens(l), s - 1)
        else s
      }

      loop(0, tokens.length - 1, P, 0)
    }
  }

  import Solution.bagOfTokensScore

  "Example 1: ([100], P = 50) -> 0" in {
    val tokens = Array(100)
    bagOfTokensScore(tokens, 5) shouldBe 0
    // Explanation:
    //   Playing the only token in the bag is impossible
    //   because you either have too little power or too little score.
  }
  "Example 2: ([100,200], P = 150) -> 1" in {
    val tokens = Array(100, 200)
    bagOfTokensScore(tokens, 150) shouldBe 1
    //Explanation: Play the 0th token (100) face up, your power becomes 50 and score becomes 1.
    //There is no need to play the 1st token since you cannot play it face up to add to your score.
  }
  "Example 3: ([100,200,300,400], P = 200) -> 2" in {
    val tokens = Array(100, 200, 300, 400)
    bagOfTokensScore(tokens, 200) shouldBe 2
    //Explanation: Play the tokens in this order to get a score of 2:
    //1. Play the 0th token (100) face up, your power becomes 100 and score becomes 1.
    //2. Play the 3rd token (400) face down, your power becomes 500 and score becomes 0.
    //3. Play the 1st token (200) face up, your power becomes 300 and score becomes 1.
    //4. Play the 2nd token (300) face up, your power becomes 0 and score becomes 2.
  }

  "Test 42: ([81,91,31], P = 73) -> 1" in {
    val tokens = Array(81, 91, 31)
    bagOfTokensScore(tokens, 73) shouldBe 1
  }
  "Test 137: ([71,55,82], P = 54) -> 0" in {
    val tokens = Array(71, 55, 82)
    bagOfTokensScore(tokens, 54) shouldBe 0
  }

}
