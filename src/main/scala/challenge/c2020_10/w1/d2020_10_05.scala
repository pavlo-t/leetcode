package challenge.c2020_10.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d2020_10_05 extends AnyWordSpec with Matchers {

  /**
   * Complement of Base 10 Integer
   *
   * Every non-negative integer `N` has a binary representation.
   * For example, `5` can be represented as `"101"` in binary, `11` as `"1011"` in binary, and so on.
   * Note that except for `N = 0`, there are no leading zeroes in any binary representation.
   *
   * The <em>complement</em> of a binary representation is the number in binary you get
   * when changing every `1` to a `0` and `0` to a `1`.
   * For example, the complement of `"101"` in binary is `"010"` in binary.
   *
   * For a given number `N` in base-10, return the complement of it's binary representation as a base-10 integer.
   *
   *
   * Constraints:<ol>
   * <li> `0 <= N < 10^9`
   * <li> This question is the same as 476: [[https://leetcode.com/problems/number-complement/]]
   */
  object Solution {
    def bitwiseComplement1(N: Int): Int = {
      val complement = N.toBinaryString.map {
        case '1' => '0'
        case '0' => '1'
      }
      Integer.parseInt(complement, 2)
    }

    def bitwiseComplement(N: Int): Int =
      Integer.parseInt(N.toBinaryString.map(c => if (c == '0') '1' else '0'), 2)
  }

  "Example 1" in {
    Solution.bitwiseComplement(5) shouldBe 2
    // Explanation: 5 is "101" in binary, with complement "010" in binary, which is 2 in base-10.
  }
  "Example 2" in {
    Solution.bitwiseComplement(7) shouldBe 0
    // Explanation: 7 is "111" in binary, with complement "000" in binary, which is 0 in base-10.
  }
  "Example 3" in {
    Solution.bitwiseComplement(10) shouldBe 5
    // Explanation: 10 is "1010" in binary, with complement "0101" in binary, which is 5 in base-10.
  }

  "Test 0" in {
    Solution.bitwiseComplement(0) shouldBe 1
  }
}
