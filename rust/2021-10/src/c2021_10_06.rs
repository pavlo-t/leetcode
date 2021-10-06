#![allow(dead_code)]
/// 442. Find All Duplicates in an Array
/// ====================================
///
/// Given an integer array `nums` of length `n`
/// where all the integers of nums are in the range `[1, n]`
/// and each integer appears __once__ or __twice__,
/// return _an array of all the integers that appears __twice___.
///
/// You must write an algorithm that runs in `O(n)` time and uses only constant extra space.
///
/// __Constraints:__
///
/// - `1 <= nums.length <= 100_000`
/// - `1 <= nums[i] <= nums.length`
/// - Each element in nums appears __once__ or __twice__.
///
/// https://leetcode.com/problems/find-all-duplicates-in-an-array/
struct Solution;
impl Solution {
    pub fn find_duplicates(mut ns: Vec<i32>) -> Vec<i32> {
        //println!("find_duplicates({:?})", ns);
        Self::radix_sort(&mut ns, 10);
        let mut result = vec![];
        for i in 1..ns.len() {
            if ns[i - 1] == ns[i] {
                result.push(ns[i]);
            }
        }
        result
    }
    /// In-place radix sort
    /// https://en.wikipedia.org/wiki/Radix_sort#In-place_MSD_radix_sort_implementations
    fn radix_sort(ns: &mut [i32], radix: usize) {
        fn sort_at_exp(ns: &mut [i32], max: i32, exp: i32, radix: usize) {
            if exp > 0 {
                let bin = |n: i32| ((n / exp) as usize) % radix;

                // count elements for bins:
                let mut counts = vec![0usize; radix];
                for &n in ns.iter() {
                    counts[bin(n)] += 1;
                }

                // get bin left indexes:
                let mut currs = vec![0usize; radix];
                for i in 0..radix - 1 {
                    currs[i + 1] = currs[i] + counts[i];
                }
                let starts = currs.clone();

                // reorder elements:
                let mut i = 0;
                while i < ns.len() {
                    let b = bin(ns[i]);
                    if i == currs[b] {
                        currs[b] += 1;
                        i += 1;
                    } else if i >= starts[b] && i < currs[b] {
                        i += 1;
                    } else {
                        ns.swap(i, currs[b]);
                        currs[b] += 1;
                    }
                }

                // recursively sort bins:
                let n_exp = exp / radix as i32;
                for i in 0..radix {
                    if counts[i] > 1 {
                        sort_at_exp(&mut ns[starts[i]..currs[i]], max, n_exp, radix);
                    }
                }
            }
        }

        let &max = ns.iter().max().unwrap();
        let mut exp = 1;
        while max / exp >= 1 {
            exp *= radix as i32;
        }

        sort_at_exp(ns, max, exp, radix);
    }
    fn radix_sort_const(ns: &mut [i32]) {
        const RADIX: usize = 10;
        fn sort_at_exp(ns: &mut [i32], max: i32, exp: i32) {
            if exp > 0 {
                let bin = |n: i32| ((n / exp) as usize) % RADIX;

                // count elements for bins:
                let mut counts = vec![0usize; RADIX];
                for &n in ns.iter() {
                    counts[bin(n)] += 1;
                }

                // get bin left indexes:
                let mut currs = vec![0usize; RADIX];
                for i in 0..RADIX - 1 {
                    currs[i + 1] = currs[i] + counts[i];
                }
                let starts = currs.clone();

                // reorder elements:
                let mut i = 0;
                while i < ns.len() {
                    let b = bin(ns[i]);
                    if i == currs[b] {
                        currs[b] += 1;
                        i += 1;
                    } else if i >= starts[b] && i < currs[b] {
                        i += 1;
                    } else {
                        ns.swap(i, currs[b]);
                        currs[b] += 1;
                    }
                }

                // recursively sort bins:
                let n_exp = exp / RADIX as i32;
                for i in 0..RADIX {
                    if counts[i] > 1 {
                        sort_at_exp(&mut ns[starts[i]..currs[i]], max, n_exp);
                    }
                }
            }
        }

        let &max = ns.iter().max().unwrap();
        let mut exp = 1;
        while max / exp >= 1 {
            exp *= RADIX as i32;
        }

        sort_at_exp(ns, max, exp);
    }
    fn radix_sort_10(ns: &mut [i32]) {
        fn sort_at_exp(ns: &mut [i32], max: i32, exp: i32) {
            if exp > 0 {
                let bin = |n: i32| (n / exp) as usize % 10;

                // count elements for bins:
                let mut counts = [0usize; 10];
                for &n in ns.iter() {
                    counts[bin(n)] += 1;
                }

                // get bin left indexes:
                let mut currs = [0usize; 10];
                for i in 0..9 {
                    currs[i + 1] = currs[i] + counts[i];
                }
                let starts = currs.clone();

                // reorder elements:
                let mut i = 0;
                while i < ns.len() {
                    let b = bin(ns[i]);
                    if i == currs[b] {
                        currs[b] += 1;
                        i += 1;
                    } else if i >= starts[b] && i < currs[b] {
                        i += 1;
                    } else {
                        ns.swap(i, currs[b]);
                        currs[b] += 1;
                    }
                }

                // recursively sort bins:
                let n_exp = exp / 10;
                for i in 0..10 {
                    if counts[i] > 1 {
                        sort_at_exp(&mut ns[starts[i]..currs[i]], max, n_exp);
                    }
                }
            }
        }

        let &max = ns.iter().max().unwrap();
        let mut exp = 1;
        while max / exp >= 1 {
            exp *= 10;
        }

        sort_at_exp(ns, max, exp);
    }

    pub fn find_duplicates_cyclic_sort(mut ns: Vec<i32>) -> Vec<i32> {
        println!("find_duplicates({:?})", ns);
        let mut res = vec![];
        //// Same code with a while loop:
        //let mut i = 0;
        //while i < ns.len() {
        //    if ns[i] != ns[(ns[i] - 1) as usize] {
        //        let j = (ns[i] - 1) as usize;
        //        ns.swap(i, j);
        //    } else {
        //        i += 1;
        //    }
        //}
        for i in 0..ns.len() {
            while ns[i] != ns[(ns[i] - 1) as usize] {
                let j = (ns[i] - 1) as usize;
                ns.swap(i, j);
            }
        }
        for i in 0..ns.len() {
            if ns[i] != (i + 1) as i32 {
                res.push(ns[i]);
            }
        }
        res
    }
    /// Approach 4: Mark Visited Elements in the Input Array itself
    /// https://leetcode.com/problems/find-all-duplicates-in-an-array/solution/
    pub fn find_duplicates_mark_with_minus(mut ns: Vec<i32>) -> Vec<i32> {
        println!("find_duplicates({:?})", ns);
        let mut res = vec![];
        for i in 0..ns.len() {
            let n = ns[i].abs();
            let j = (n - 1) as usize;
            println!(" i:{},n:{},j:{},ns:{:?}", i, n, j, ns);
            if ns[j] < 0 {
                res.push(n);
            } else {
                ns[j] = -ns[j];
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_4_3_2_7_8_2_3_1() {
        let nums = vec![4, 3, 2, 7, 8, 2, 3, 1];
        let e = [2, 3];
        assert_eq!(Solution::find_duplicates(nums), e);
    }
    //#[ignore]
    #[test]
    fn n_1_1_2() {
        let nums = vec![1, 1, 2];
        let e = [1];
        assert_eq!(Solution::find_duplicates(nums), e);
    }
    //#[ignore]
    #[test]
    fn n_1() {
        let nums = vec![1];
        let e: Vec<i32> = vec![];
        assert_eq!(Solution::find_duplicates(nums), e);
    }
    //#[ignore]
    #[test]
    fn n_2_2() {
        let nums = vec![2, 2];
        let e = [2];
        assert_eq!(Solution::find_duplicates(nums), e);
    }
    #[test]
    fn n_1_2_3_4_5_6_7_8_9_10_11_12() {
        let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        let e: Vec<i32> = vec![];
        assert_eq!(Solution::find_duplicates(nums), e);
    }
    #[test]
    fn n_12_11_10_9_8_7_6_5_4_3_2_1() {
        let nums = vec![12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let e: Vec<i32> = vec![];
        assert_eq!(Solution::find_duplicates(nums), e);
    }
    #[test]
    fn n_111_112_108_12_11_10_9_8_7_6_5_4_3_2_1() {
        let nums = vec![111, 112, 108, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let e: Vec<i32> = vec![];
        assert_eq!(Solution::find_duplicates(nums), e);
    }

    mod test_radix_sort {
        use super::*;

        //#[ignore]
        #[test]
        fn radix_inline_ns_100000to1() {
            let mut ns: Vec<i32> = (1..=100000).rev().collect();
            let e: Vec<i32> = (1..=100000).collect();
            Solution::radix_sort_10(&mut ns);
            assert_eq!(ns, e);
        }
        //#[ignore]
        #[test]
        fn radix_inline_ns_10000to1_repeated_2() {
            let mut ns: Vec<i32> = (1..=10000).chain(1..=10000).rev().collect();
            let mut e = ns.clone();
            e.sort_unstable();
            Solution::radix_sort_10(&mut ns);
            assert_eq!(ns, e);
        }

        //#[ignore]
        #[test]
        fn radix_2_ns_100000to1_radix_2() {
            let mut ns: Vec<i32> = (1..=100000).rev().collect();
            let e: Vec<i32> = (1..=100000).collect();
            Solution::radix_sort(&mut ns, 2);
            assert_eq!(ns, e);
        }
        //#[ignore]
        #[test]
        fn radix_2_ns_10000to1_repeated_2_radix_2() {
            let mut ns: Vec<i32> = (1..=10000).chain(1..=10000).rev().collect();
            let mut e = ns.clone();
            e.sort_unstable();
            Solution::radix_sort(&mut ns, 2);
            assert_eq!(ns, e);
        }

        //#[ignore]
        #[test]
        fn radix_10_ns_100000to1_radix_2() {
            let mut ns: Vec<i32> = (1..=100000).rev().collect();
            let e: Vec<i32> = (1..=100000).collect();
            Solution::radix_sort(&mut ns, 10);
            assert_eq!(ns, e);
        }
        //#[ignore]
        #[test]
        fn radix_10_ns_10000to1_repeated_2_radix_2() {
            let mut ns: Vec<i32> = (1..=10000).chain(1..=10000).rev().collect();
            let mut e = ns.clone();
            e.sort_unstable();
            Solution::radix_sort(&mut ns, 10);
            assert_eq!(ns, e);
        }

        //#[ignore]
        #[test]
        fn radix_const_ns_100000to1() {
            let mut ns: Vec<i32> = (1..=100000).rev().collect();
            let e: Vec<i32> = (1..=100000).collect();
            Solution::radix_sort_const(&mut ns);
            assert_eq!(ns, e);
        }
        //#[ignore]
        #[test]
        fn radix_const_ns_10000to1_repeated_2() {
            let mut ns: Vec<i32> = (1..=10000).chain(1..=10000).rev().collect();
            let mut e = ns.clone();
            e.sort_unstable();
            Solution::radix_sort_const(&mut ns);
            assert_eq!(ns, e);
        }
    }
}
