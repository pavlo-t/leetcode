package c2020_04.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class d01 extends AnyWordSpec with Matchers {

  object Solution {
    def singleNumber(nums: Array[Int]): Int = {
      nums
    }
  }

  "Solution" should {
    "1" in {
      Solution.singleNumber(Array(2, 2, 1)) shouldBe 1
    }

    "2" in {
      Solution.singleNumber(Array(4, 1, 2, 1, 2)) shouldBe 4
    }

    "3" in {
      Solution.singleNumber(Array(4, 1, 2, 1, 2)) shouldBe 4
    }
  }
}
