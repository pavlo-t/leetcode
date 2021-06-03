/// # Advantage Shuffle
///
/// Given two arrays `A` and `B` of equal size,
/// the _advantage of `A` with respect to `B`_ is the number of indices `i` for which `A[i] > B[i]`.
///
/// Return __any__ permutation of `A` that maximizes its advantage with respect to `B`.
///
/// __Note:__
///
/// - `1 <= A.length = B.length <= 10000`
/// - `0 <= A[i] <= 10^9`
/// - `0 <= B[i] <= 10^9`
struct Solution;
impl Solution {
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        a.sort_unstable();
        let mut b = b
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect::<Vec<_>>();
        b.sort_unstable();

        let mut l = 0;
        let mut r = a.len() - 1;
        let mut result = vec![-1; a.len()];
        for i in (0..a.len()).rev() {
            if b[i].0 < a[r] {
                result[b[i].1] = a[r];
                r = r.saturating_sub(1);
            } else {
                result[b[i].1] = a[l];
                l += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let a = vec![2, 7, 11, 15];
        let b = vec![1, 10, 4, 11];
        let e = vec![2, 11, 7, 15];
        assert_eq!(Solution::advantage_count(a, b), e);
    }
    #[test]
    fn example2() {
        let a = vec![12, 24, 8, 32];
        let b = vec![13, 25, 32, 11];
        let e = vec![24, 32, 8, 12];
        assert_eq!(Solution::advantage_count(a, b), e);
    }

    #[test]
    fn test_a_all_lower() {
        let a = vec![4, 3, 2, 1];
        let b = vec![9, 8, 7, 6];
        let e = vec![1, 2, 3, 4];
        assert_eq!(Solution::advantage_count(a, b), e);
    }

    mod performance {
        use super::*;

        #[test]
        fn test10_000_elements() {
            let a = (1..=10_000).rev().collect();
            let b = (0..10_000).rev().collect();
            let e = (1..=10_000).rev().collect::<Vec<_>>();
            assert_eq!(Solution::advantage_count(a, b), e);
        }
    }
}
