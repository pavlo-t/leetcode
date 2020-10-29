package c2020_10.w4

import org.scalatest.matchers.should.Matchers
import org.scalatest.wordspec.AnyWordSpec

class d2020_10_26 extends AnyWordSpec with Matchers {

  /**
   * <h3>Champagne Tower</h3>
   *
   * We stack glasses in a pyramid, where the <b>first</b> row has `1` glass, the <b>second</b> row has `2` glasses,
   * and so on until the 100th row.
   * Each glass holds one cup of champagne.
   *
   * Then, some champagne is poured into the first glass at the top.
   * When the topmost glass is full, any excess liquid poured will fall equally to the glass immediately to the left
   * and right of it.
   * When those glasses become full, any excess champagne will fall equally to the left and right of those glasses,
   * and so on.
   * (A glass at the bottom row has its excess champagne fall on the floor.)
   *
   * For example, after one cup of champagne is poured, the top most glass is full.
   * After two cups of champagne are poured, the two glasses on the second row are half full.
   * After three cups of champagne are poured, those two cups become full - there are 3 full glasses total now.
   * After four cups of champagne are poured, the third row has the middle glass half full, and the two outside
   * glasses are a quarter full.
   *
   * Now after pouring some non-negative integer cups of champagne, return how full the `j`th glass in the `i`th row is
   * (both `i` and `j` are 0-indexed.)
   *
   * <b>Constraints:</b><ul>
   * <li> `0 <= poured <= 10**9`
   * <li> `0 <= query_glass <= query_row < 100`
   * </ul>
   */
  object Solution {
    def champagneTower(p: Int, r: Int, g: Int): Double = {
      val t = newTower(r + 1)

      t(0)(0) = p
      for (cr <- 0 to r) {
        for (cg <- 0 to cr) {
          val e = (t(cr)(cg) - 1.0) / 2
          if (e > 0) {
            t(cr + 1)(cg) += e
            t(cr + 1)(cg + 1) += e
          }
        }
      }

      math.min(1.0, t(r)(g))
    }

    def newTower(lr: Int): Array[Array[Double]] = {
      val t = Array.ofDim[Array[Double]](lr + 1)

      for (r <- 0 to lr) {
        t(r) = Array.ofDim[Double](r + 1)
        for (g <- 0 to r) {
          t(r)(g) = 0.0
        }
      }

      t
    }
  }

  import Solution.champagneTower

  "Example 1: (poured = 1, query_row = 1, query_glass = 1) -> 0.0" in {
    champagneTower(1, 1, 1) shouldBe 0.0
    // Explanation: We poured 1 cup of champagne to the top glass of the tower (which is indexed as (0, 0)).
    //   There will be no excess liquid so all the glasses under the top glass will remain empty.
  }
  "Example 2: (poured = 2, query_row = 1, query_glass = 1) -> 0.5" in {
    champagneTower(2, 1, 1) shouldBe 0.5
    // Explanation: We poured 2 cups of champagne to the top glass of the tower (which is indexed as (0, 0)).
    //   There is one cup of excess liquid.
    //   The glass indexed as (1, 0) and the glass indexed as (1, 1) will share the excess liquid equally,
    //   and each will get half cup of champange.
  }
  "Example 3: (poured = 100000009, query_row = 33, query_glass = 17) -> 1.0" in {
    champagneTower(100000009, 33, 17) shouldBe 1.0
  }

}
