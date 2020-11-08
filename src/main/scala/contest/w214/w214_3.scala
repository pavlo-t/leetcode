package contest.w214

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

//noinspection DuplicatedCode
class w214_3 extends AnyWordSpec with Matchers {

  /**
   * <h3>5563. Sell Diminishing-Valued Colored Balls</h3>
   *
   * You have an `inventory` of different colored balls,
   * and there is a customer that wants `orders` balls of <b>any</b> color.
   *
   * The customer weirdly values the colored balls.
   * Each colored ball's value is the number of balls <b>of that color</b> you currently have in your `inventory`.
   * For example, if you own `6` yellow balls, the customer would pay `6` for the first yellow ball.
   * After the transaction, there are only `5` yellow balls left, so the next yellow ball is then valued at `5`
   * (i.e., the value of the balls decreases as you sell more to the customer).
   *
   * You are given an integer array, `inventory`,
   * where `inventory[i]` represents the number of balls of the `i`th color that you initially own.
   * You are also given an integer `orders`, which represents the total number of balls that the customer wants.
   * You can sell the balls <b>in any order</b>.
   *
   * Return <em>the <b>maximum</b> total value that you can attain after selling `orders` colored balls<em>.
   * As the answer may be too large, return it <b>modulo</b> `1_000_000_007`.
   *
   * <b>Constraints:</b><ul>
   * <li> `1 <= inventory.length <= 100_000`
   * <li> `1 <= inventory[i] <= 1_000_000_000`
   * <li> `1 <= orders <= min(sum(inventory[i]), 1_000_000_000)`
   * </ul>
   */
  object Solution {
    val Modulo = 1_000_000_007

    def maxProfit(inventory: Array[Int], orders: Int): Int = {
      def countAbove(t: Int): Long = inventory.foldLeft(0L) { (acc, v) => if (v > t) acc - t + v else acc }

      @scala.annotation.tailrec
      def getLowestPrice(l: Int, r: Int): Int =
        if (l > r) l
        else {
          val mid = (l + r) / 2
          if (countAbove(mid) < orders) getLowestPrice(l, mid - 1)
          else getLowestPrice(mid + 1, r)
        }
      val lowestPrice = getLowestPrice(0, 1_000_000_000)

      val (aboveLowestPrice, count) = inventory.foldLeft((0L, 0)) { case (acc@(sum, count), v) =>
        if (v > lowestPrice) {
          val vSum = (v.toLong + lowestPrice + 1) * (v - lowestPrice) / 2
          ((sum + vSum % Modulo) % Modulo, count - lowestPrice + v)
        } else acc
      }

      ((aboveLowestPrice + (orders.toLong - count) * lowestPrice) % Modulo).toInt
    }
  }

  object SolutionPyt {
    val Modulo = 1_000_000_007

    def maxProfit(inventory: Array[Int], orders: Int): Int = {
      def fun(x: Int): BigInt = inventory.map(a => if (a >= x) BigInt(a - x + 1) else BigInt(0)).sum

      var l = 0
      var r = 1_000_000_000
      while (l < r) {
        val mid = (l + r + 1) / 2
        val tmp = fun(mid)
        if (tmp >= orders) l = mid
        else r = mid - 1
      }
      //println(s" after WHILE: l = $l")
      var ans = BigInt(0)
      var os = BigInt(orders)
      for (a <- inventory) {
        if (a >= l + 1) {
          ans += (BigInt(a) + l + 1) * (BigInt(a) - l) / 2
          //ans %= Modulo
          os -= (a - l)
        }
      }
      ans += os * l
      (ans % Modulo).toInt
    }
  }
  object SolutionBruteForceTailRec {
    val Modulo = 1_000_000_007

    def maxProfit(inventory: Array[Int], orders: Int): Int = {
      def addTimesMod(a: Int, b: Int, times: Int): Int =
        (1 to times).fold(a)((acc, _) => (acc + b) % Modulo)

      @scala.annotation.tailrec
      def getMaxProfit(o: Int, rsf: Int = 0): Int = {
        if (o == 0) rsf
        else {
          val max = inventory.max
          val no = inventory.count(_ == max)
          if (no > o) addTimesMod(rsf, max, o)
          else {
            inventory.mapInPlace(i => if (i == max) max - 1 else i)
            getMaxProfit(o - no, addTimesMod(rsf, max, no))
          }
        }
      }

      getMaxProfit(orders) % Modulo
    }
  }

  import Solution.maxProfit

  "Example 1: (inventory = [2,5], orders = 4) -> 14" in {
    maxProfit(Array(2, 5), 4) shouldBe 14
    // Explanation: Sell the 1st color 1 time (2) and the 2nd color 3 times (5 + 4 + 3).
    // The maximum total value is 2 + 5 + 4 + 3 = 14.
  }
  "Example 2: (inventory = [3,5], orders = 6) -> 19" in {
    maxProfit(Array(3, 5), 6) shouldBe 19
    // Explanation: Sell the 1st color 2 times (3 + 2) and the 2nd color 4 times (5 + 4 + 3 + 2).
    // The maximum total value is 3 + 2 + 5 + 4 + 3 + 2 = 19.
  }
  "Example 3: (inventory = [2,8,4,10,6], orders = 20) -> 110" in {
    maxProfit(Array(2, 8, 4, 10, 6), 20) shouldBe 110
  }

  "(inventory = [1,2,3], orders = 6) -> 10" in {
    maxProfit(Array(1, 2, 3), 6) shouldBe 10
  }
  "(inventory = [1,2,3], orders = 5) -> 9" in {
    maxProfit(Array(1, 2, 3), 5) shouldBe 9
  }
  "(inventory = [1,2,3], orders = 4) -> 8" in {
    maxProfit(Array(1, 2, 3), 4) shouldBe 8
  }
  "(inventory = [1,2,3], orders = 3) -> 7" in {
    maxProfit(Array(1, 2, 3), 3) shouldBe 7
  }
  "(inventory = [1,2,3], orders = 2) -> 5" in {
    maxProfit(Array(1, 2, 3), 2) shouldBe 5
  }
  "(inventory = [1,2,3], orders = 1) -> 3" in {
    maxProfit(Array(1, 2, 3), 1) shouldBe 3
  }
  "(inventory = [1,1,5], orders = 1) -> 5" in {
    maxProfit(Array(1, 1, 5), 1) shouldBe 5
  }
  "(inventory = [1,1,5], orders = 2) -> 9" in {
    maxProfit(Array(1, 1, 5), 2) shouldBe 9
  }
  "(inventory = [1,1,5], orders = 3) -> 12" in {
    maxProfit(Array(1, 1, 5), 3) shouldBe 12
  }
  "(inventory = [1,1,5], orders = 4) -> 14" in {
    maxProfit(Array(1, 1, 5), 4) shouldBe 14
  }
  "(inventory = [1,1,5], orders = 5) -> 15" in {
    maxProfit(Array(1, 1, 5), 5) shouldBe 15
  }
  "(inventory = [1,1,5], orders = 6) -> 16" in {
    maxProfit(Array(1, 1, 5), 6) shouldBe 16
  }
  "(inventory = [1,1,5], orders = 7) -> 17" in {
    maxProfit(Array(1, 1, 5), 7) shouldBe 17
  }
  "(inventory = [5,3,2,2,1], orders = 7) -> 17" in {
    maxProfit(Array(5, 3, 2, 2, 1), 7) shouldBe 21
  }

  "Example 4: (inventory = [1_000_000_000], orders = 1_000_000_000) -> 21" in {
    maxProfit(Array(1_000_000_000), 1_000_000_000) shouldBe 21
    // Explanation: Sell the 1st color 1000000000 times for a total value of 500000000500000000.
    // 500000000500000000 modulo 109 + 7 = 21.
  }

  "(inventory = [1_000_000_000], orders = 999_999_998) -> 18" in {
    maxProfit(Array(1_000_000_000), 999_999_998) shouldBe 18
  }
  "(inventory = [1_000_000_000, 500_000_000], orders = 1_000_000_000) -> 562500028" in {
    maxProfit(Array(1_000_000_000, 500_000_000), 1_000_000_000) shouldBe 562500028
  }
  "(inventory = [100_000 of 1_000_000_000], orders = 1_000_000_000) -> 500035049" in {
    val inventory = Array.fill(100_000)(1_000_000_000)
    maxProfit(inventory, 1_000_000_000) shouldBe 500035049
  }
  "(inventory = [100_000 of 1_000_000_000], orders = 100_000) -> 500035049" in {
    val inventory = Array.fill(100_000)(1_000_000_000)
    maxProfit(inventory, 100_000) shouldBe 999300007
  }
}
