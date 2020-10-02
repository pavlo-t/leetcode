package c2020_04.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec


class d02 extends AnyWordSpec with Matchers {

  object Solution {
    def isHappy(n: Int): Boolean = {
      import scala.annotation.tailrec

      @tailrec
      def loop(i: Int, acc: Set[Int]): Boolean =
        if (i == 1) true
        else if (acc.contains(i)) false
        else loop(i.toString.foldLeft(0)((r, c) => r + math.pow(c.asDigit, 2).toInt), acc + i)

      loop(n, Set())
    }
  }

  "Solution" should {
    "19" in {
      Solution.isHappy(19) shouldBe true
    }

    "2" in {
      Solution.isHappy(2) shouldBe false
    }
  }
}
