#![allow(dead_code)]
/// Powerful Integers
/// =================
///
/// Given three integers `x`, `y`, and `bound`, return
/// _a list of all the __powerful integers__ that have a value less than or equal to_ `bound`.
///
/// An integer is __powerful__ if it can be represented as
/// `x^i + y^j` for some integers `i >= 0` and `j >= 0`.
///
/// You may return the answer in __any order__.
/// In your answer, each value should occur __at most once__.
///
/// __Constraints:__
///
/// - `1 <= x, y <= 100`
/// - `0 <= bound <= 1_000_000`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/597/week-5-april-29th-april-30th/3726/
struct Solution;
impl Solution {
    pub fn powerful_integers(x: i32, y: i32, bound: i32) -> Vec<i32> {
        use std::collections::HashSet;
        use std::hash::Hash;

        let bf = bound as f64;
        let max_p = |v: i32| if v == 1 { 0 } else { bf.log(v as f64) as u32 };
        fn inserted<T: Eq + Hash>(mut s: HashSet<T>, v: T) -> HashSet<T> {
            s.insert(v);
            s
        }

        (0..=max_p(x))
            .flat_map(|xp| (0..=max_p(y)).map(move |yp| (xp, yp)))
            .fold(HashSet::new(), |acc, (xp, yp)| {
                match x.pow(xp) + y.pow(yp) {
                    v if v <= bound => inserted(acc, v),
                    _ => acc,
                }
            })
            .into_iter()
            .collect()
    }

    pub fn powerful_integers_leetcode(x: i32, y: i32, bound: i32) -> Vec<i32> {
        use std::collections::HashSet;

        let b = bound as f64;
        let get_max_p = |v| if v == 1.0 { 0 } else { b.log(v) as u32 };

        let xm = get_max_p(x as f64);
        let ym = get_max_p(y as f64);
        let mut r = HashSet::new();

        for xp in 0..=xm {
            for yp in 0..=ym {
                let v = x.pow(xp) + y.pow(yp);
                if v <= bound {
                    r.insert(v);
                }
            }
        }
        r.into_iter().collect()
    }

    pub fn powerful_integers_my(x: i32, y: i32, bound: i32) -> Vec<i32> {
        if bound < 2 {
            Vec::new()
        } else if x == 1 && y == 1 {
            vec![2]
        } else {
            let (x, y) = (x.min(y), x.max(y));

            if x == 1 {
                let mut r = Vec::new();
                let mut yp = 1;
                while yp + 1 <= bound {
                    r.push(yp + 1);
                    yp *= y;
                }
                r
            } else {
                use std::collections::HashSet;

                let mut r = HashSet::new();
                let mut xp = 1;
                let mut yp = 1;
                while yp + 1 <= bound {
                    while xp + yp <= bound {
                        r.insert(xp + yp);
                        xp *= x;
                    }
                    xp = 1;
                    yp *= y;
                }
                r.into_iter().collect()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_x2y3b10_produces_2_3_4_5_7_9_10() {
        let mut r = Solution::powerful_integers(2, 3, 10);
        r.sort_unstable();
        let e = [2, 3, 4, 5, 7, 9, 10];
        assert_eq!(r, e);
        // Explanation:
        //  2 = 2^0 + 3^0
        //  3 = 2^1 + 3^0
        //  4 = 2^0 + 3^1
        //  5 = 2^1 + 3^1
        //  7 = 2^2 + 3^1
        //  9 = 2^3 + 3^0
        // 10 = 2^0 + 3^2
    }
    #[test]
    fn example2_x3y5b15_produces_2_4_6_8_10_14() {
        let mut r = Solution::powerful_integers(3, 5, 15);
        r.sort_unstable();
        let e = [2, 4, 6, 8, 10, 14];
        assert_eq!(r, e);
    }

    #[test]
    fn x1y1b0_produces_empty() {
        assert_eq!(Solution::powerful_integers(1, 1, 0), []);
    }
    #[test]
    fn x1y1b1_produces_empty() {
        assert_eq!(Solution::powerful_integers(1, 1, 1), []);
    }
    #[test]
    fn x1y1b2_produces_2() {
        assert_eq!(Solution::powerful_integers(1, 1, 2), [2]);
    }
    #[test]
    fn x1y1b1000000_produces_2() {
        assert_eq!(Solution::powerful_integers(1, 1, 1_000_000), [2]);
    }
    #[test]
    fn x1y2b10_produces_2_3_5_9() {
        let mut r = Solution::powerful_integers(1, 2, 10);
        r.sort_unstable();
        assert_eq!(r, [2, 3, 5, 9]);
    }
    #[test]
    fn x2y1b10_produces_2_3_5_9() {
        let mut r = Solution::powerful_integers(2, 1, 10);
        r.sort_unstable();
        assert_eq!(r, [2, 3, 5, 9]);
    }

    mod performance {
        use super::*;

        #[test]
        fn x2y2b1000000() {
            let mut r = Solution::powerful_integers(2, 2, 1_000_000);
            r.sort_unstable();
            let e = [
                2, 3, 4, 5, 6, 8, 9, 10, 12, 16, 17, 18, 20, 24, 32, 33, 34, 36, 40, 48, 64, 65,
                66, 68, 72, 80, 96, 128, 129, 130, 132, 136, 144, 160, 192, 256, 257, 258, 260,
                264, 272, 288, 320, 384, 512, 513, 514, 516, 520, 528, 544, 576, 640, 768, 1024,
                1025, 1026, 1028, 1032, 1040, 1056, 1088, 1152, 1280, 1536, 2048, 2049, 2050, 2052,
                2056, 2064, 2080, 2112, 2176, 2304, 2560, 3072, 4096, 4097, 4098, 4100, 4104, 4112,
                4128, 4160, 4224, 4352, 4608, 5120, 6144, 8192, 8193, 8194, 8196, 8200, 8208, 8224,
                8256, 8320, 8448, 8704, 9216, 10240, 12288, 16384, 16385, 16386, 16388, 16392,
                16400, 16416, 16448, 16512, 16640, 16896, 17408, 18432, 20480, 24576, 32768, 32769,
                32770, 32772, 32776, 32784, 32800, 32832, 32896, 33024, 33280, 33792, 34816, 36864,
                40960, 49152, 65536, 65537, 65538, 65540, 65544, 65552, 65568, 65600, 65664, 65792,
                66048, 66560, 67584, 69632, 73728, 81920, 98304, 131072, 131073, 131074, 131076,
                131080, 131088, 131104, 131136, 131200, 131328, 131584, 132096, 133120, 135168,
                139264, 147456, 163840, 196608, 262144, 262145, 262146, 262148, 262152, 262160,
                262176, 262208, 262272, 262400, 262656, 263168, 264192, 266240, 270336, 278528,
                294912, 327680, 393216, 524288, 524289, 524290, 524292, 524296, 524304, 524320,
                524352, 524416, 524544, 524800, 525312, 526336, 528384, 532480, 540672, 557056,
                589824, 655360, 786432,
            ];
            assert_eq!(r, e);
        }
        #[test]
        fn x2y3b1000000() {
            let mut r = Solution::powerful_integers(2, 3, 1_000_000);
            r.sort_unstable();
            let e = [
                2, 3, 4, 5, 7, 9, 10, 11, 13, 17, 19, 25, 28, 29, 31, 33, 35, 41, 43, 59, 65, 67,
                73, 82, 83, 85, 89, 91, 97, 113, 129, 131, 137, 145, 155, 209, 244, 245, 247, 251,
                257, 259, 265, 275, 283, 307, 337, 371, 499, 513, 515, 521, 539, 593, 730, 731,
                733, 737, 745, 755, 761, 793, 857, 985, 1025, 1027, 1033, 1051, 1105, 1241, 1267,
                1753, 2049, 2051, 2057, 2075, 2129, 2188, 2189, 2191, 2195, 2203, 2219, 2251, 2291,
                2315, 2443, 2699, 2777, 3211, 4097, 4099, 4105, 4123, 4177, 4235, 4339, 4825, 6283,
                6562, 6563, 6565, 6569, 6577, 6593, 6625, 6689, 6817, 7073, 7585, 8193, 8195, 8201,
                8219, 8273, 8435, 8609, 8921, 10379, 10657, 14753, 16385, 16387, 16393, 16411,
                16465, 16627, 17113, 18571, 19684, 19685, 19687, 19691, 19699, 19715, 19747, 19811,
                19939, 20195, 20707, 21731, 22945, 23779, 27875, 32769, 32771, 32777, 32795, 32849,
                33011, 33497, 34955, 36067, 39329, 52451, 59050, 59051, 59053, 59057, 59065, 59081,
                59113, 59177, 59305, 59561, 60073, 61097, 63145, 65537, 65539, 65545, 65563, 65617,
                65779, 66265, 67241, 67723, 72097, 75433, 85219, 91817, 124585, 131073, 131075,
                131081, 131099, 131153, 131315, 131801, 133259, 137633, 150755, 177148, 177149,
                177151, 177155, 177163, 177179, 177211, 177275, 177403, 177659, 178171, 179195,
                181243, 185339, 190121, 193531, 209915, 242683, 262145, 262147, 262153, 262171,
                262225, 262387, 262873, 264331, 268705, 281827, 308219, 321193, 439291, 524289,
                524291, 524297, 524315, 524369, 524531, 525017, 526475, 530849, 531442, 531443,
                531445, 531449, 531457, 531473, 531505, 531569, 531697, 531953, 532465, 533489,
                535537, 539633, 543971, 547825, 564209, 583337, 596977, 662513, 701435, 793585,
            ];
            assert_eq!(r, e);
        }
    }
}
