#![allow(dead_code)]
/// 875. Koko Eating Bananas
/// ========================
///
/// Koko loves to eat bananas.
/// There are `n` piles of bananas, the `i`th pile has `piles[i]` bananas.
/// The guards have gone and will come back in `h` hours.
///
/// Koko can decide her bananas-per-hour eating speed of `k`.
/// Each hour, she chooses some pile of bananas and eats `k` bananas from that pile.
/// If the pile has less than `k` bananas,
/// she eats all of them instead and will not eat any more bananas during this hour.
///
/// Koko likes to eat slowly but still wants to finish eating all the bananas before the guards return.
///
/// Return _the minimum integer `k` such that she can eat all the bananas within `h` hours_.
///
/// __Constraints:__
///
/// - `1 <= piles.length <= 10_000`
/// - `piles.length <= h <= 1_000_000_000`
/// - `1 <= piles[i] <= 1_000_000_000`
///
/// https://leetcode.com/problems/koko-eating-bananas/
struct Solution;
impl Solution {
    #[rustfmt::skip]
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (sum, max) = piles.iter().fold((0, 0), |(sum, max), &pile| (sum + pile, max.max(pile)));
        let (mut l, mut r) = ((sum / h).max(1), max);
        while l < r {
            let m = l + (r - l) / 2;
            let feedings = piles.iter().fold(0, |acc, &pile| acc + pile / m + (pile % m > 0) as i32);
            if feedings > h {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p_3_6_7_11_h_8() {
        assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
    }
    #[test]
    fn p_30_11_23_4_20_h_5() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
    }
    #[test]
    fn p_30_11_23_4_20_h_6() {
        assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
    }

    //#[ignore]
    #[test]
    fn p_1_repeat_10_000_h_1_000_000_000() {
        let p = vec![1; 10_000];
        assert_eq!(Solution::min_eating_speed(p, 1_000_000_000), 1);
    }
    //#[ignore]
    #[test]
    fn p_1_to_10_000_h_10_000() {
        let p = (1..=10_000).collect();
        assert_eq!(Solution::min_eating_speed(p, 10_000), 10_000);
    }
    //#[ignore]
    #[test]
    fn p_1_to_10_000_h_1_000_000_000() {
        let p = (1..=10_000).collect();
        assert_eq!(Solution::min_eating_speed(p, 1_000_000_000), 1);
    }
}
