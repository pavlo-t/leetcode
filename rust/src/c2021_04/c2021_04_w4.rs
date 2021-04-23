#![allow(dead_code)]
/// Missing Number In Arithmetic Progression
/// ========================================
///
/// In some array `arr`, the values were in arithmetic progression:
/// the values `arr[i+1] - arr[i]` are all equal for every `0 <= i < arr.length - 1`.
///
/// Then, a value from `arr` was removed that __was not the first or last value in the array__.
///
/// Return the removed value.
///
/// __Constraints:__
///
/// - `3 <= arr.length <= 1000`
/// - `0 <= arr[i] <= 100_000`
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/596/week-4-april-22nd-april-28th/3716/
struct Solution;
impl Solution {
    pub fn missing_number(arr: Vec<i32>) -> i32 {
        let step = (arr[arr.len() - 1] - arr[0]) / arr.len() as i32;
        let mut l = 0;
        let mut r = arr.len() - 1;
        while l < r {
            let mid = (l + r) / 2;
            if arr[mid] == arr[0] + step * mid as i32 {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        arr[0] + step * l as i32
    }

    pub fn missing_number_linear(arr: Vec<i32>) -> i32 {
        let step = match (arr[1] - arr[0], arr[arr.len() - 1] - arr[arr.len() - 2]) {
            (0, 0) => return arr[0],
            (l, r) if l == r => l,
            (l, r) if l.abs() > r.abs() => return arr[0] + r,
            (l, _) => return arr[arr.len() - 1] - l,
        };
        for w in arr.windows(2) {
            let (l, r) = (w[0], w[1]);
            if l + step != r {
                return l + step;
            }
        }
        unreachable!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_arr5_7_11_13_produces_9() {
        let arr = vec![5, 7, 11, 13];
        assert_eq!(Solution::missing_number(arr), 9);
        // Explanation: The previous array was [5,7,9,11,13].
    }
    #[test]
    fn example2_arr15_13_12_produces_14() {
        let arr = vec![15, 13, 12];
        assert_eq!(Solution::missing_number(arr), 14);
        // Explanation: The previous array was [15,14,13,12].
    }
    #[test]
    fn arr15_14_12_produces_13() {
        let arr = vec![15, 14, 12];
        assert_eq!(Solution::missing_number(arr), 13);
    }

    #[test]
    fn test4_arr00000_produces_0() {
        let arr = vec![0, 0, 0, 0, 0];
        assert_eq!(Solution::missing_number(arr), 0);
    }

    mod performance {
        use super::*;

        #[test]
        fn arr1to100_000_produces_99_999() {
            let mut arr = (1..=100_000).collect::<Vec<_>>();
            arr.remove(99_998);
            assert_eq!(Solution::missing_number(arr), 99_999);
        }
        #[test]
        fn arr1to100_000_produces_99_998() {
            let mut arr = (1..=100_000).collect::<Vec<_>>();
            arr.remove(99_997);
            assert_eq!(Solution::missing_number(arr), 99_998);
        }
    }
}
