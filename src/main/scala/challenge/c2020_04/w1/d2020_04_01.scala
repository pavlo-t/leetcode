package challenge.c2020_04.w1

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class d2020_04_01 extends AnyWordSpec with Matchers {

  object Solution {
    def singleNumber(nums: Array[Int]): Int = {
      nums.reduce(_ ^ _)
    }

    def singleNumberMy(nums: Array[Int]): Int = {
      val seen = collection.mutable.Set[Int]()
      for (i <- nums) {
        if (seen.contains(i))
          seen.remove(i)
        else
          seen.addOne(i)
      }
      seen.head
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
      val max = 100000001
      val last = max / 2
      val a = Array.ofDim[Int](max)
      for (i <- 0 to last)
        if (i == last)
          a(i) = i
        else {
          a(i) = i
          a(i + last + 1) = i
        }
      Solution.singleNumber(a) shouldBe last
    }
  }
}
