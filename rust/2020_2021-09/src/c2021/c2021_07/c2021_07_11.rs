#![allow(dead_code)]
use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
/// Find Median from Data Stream
/// ============================
///
/// The median is the middle value in an ordered integer list.
/// If the size of the list is even, there is no middle value and the median is the mean of the two middle values.
///
/// - For example, for arr = [2,3,4], the median is 3.
/// - For example, for arr = [2,3], the median is (2 + 3) / 2 = 2.5.
///
/// Implement the MedianFinder class:
///
/// - MedianFinder() initializes the MedianFinder object.
/// - void addNum(int num) adds the integer num from the data stream to the data structure.
/// - double findMedian() returns the median of all elements so far. Answers within 10-5 of the actual answer will be accepted.
///
/// __Constraints:__
///
/// - `-100_000 <= num <= 100_000`
/// - There will be at least one element in the data structure before calling `findMedian`.
/// - At most `50_000` calls will be made to `addNum` and `findMedian`.
///
/// __Follow up:__
///
/// - If all integer numbers from the stream are in the range `[0, 100]`, how would you optimize your solution?
/// - If `99%` of all integer numbers from the stream are in the range `[0, 100]`, how would you optimize your solution?
///
/// https://leetcode.com/explore/featured/card/july-leetcoding-challenge-2021/609/week-2-july-8th-july-14th/3810/
struct MedianFinderBH {
    l: RefCell<BinaryHeap<i32>>,
    r: RefCell<BinaryHeap<Reverse<i32>>>,
}
impl MedianFinderBH {
    fn new() -> Self {
        let (l, r) = (
            RefCell::new(BinaryHeap::new()),
            RefCell::new(BinaryHeap::new()),
        );
        Self { l, r }
    }

    fn add_num(&self, num: i32) {
        let (mut l, mut r) = (self.l.borrow_mut(), self.r.borrow_mut());
        if l.len() == 0 {
            l.push(num);
        } else if l.len() == r.len() {
            let &Reverse(rv) = r.peek().unwrap();
            if rv < num {
                let Reverse(lv) = r.pop().unwrap();
                l.push(lv);
                r.push(Reverse(num));
            } else {
                l.push(num);
            }
        } else {
            let &lv = l.peek().unwrap();
            if lv > num {
                let rv = l.pop().unwrap();
                l.push(num);
                r.push(Reverse(rv));
            } else {
                r.push(Reverse(num));
            }
        }
    }

    fn find_median(&self) -> f64 {
        let (l, r) = (self.l.borrow(), self.r.borrow());
        if l.len() == r.len() {
            let (&lv, &Reverse(rv)) = (l.peek().unwrap(), r.peek().unwrap());
            (lv + rv) as f64 / 2.0
        } else {
            l.peek().unwrap().to_owned() as f64
        }
    }
}

/// https://leetcode.com/problems/find-median-from-data-stream/solution/
struct MedianFinder {
    data: RefCell<Vec<i32>>,
}
impl MedianFinder {
    fn new() -> Self {
        let data = RefCell::new(vec![]);
        Self { data }
    }

    fn add_num(&self, num: i32) {
        let mut data = self.data.borrow_mut();
        data.push(num);
        data.sort_unstable();
    }

    fn find_median(&self) -> f64 {
        let d = self.data.borrow();
        if d.len() % 2 == 0 {
            let r = d.len() / 2;
            (d[r - 1] + d[r]) as f64 / 2.0
        } else {
            d[d.len() / 2] as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mf = MedianFinder::new();
        mf.add_num(1);
        assert_eq!(mf.find_median(), 1.0);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 1.5);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 2.0);
        mf.add_num(4);
        assert_eq!(mf.find_median(), 2.5);
        mf.add_num(5);
        assert_eq!(mf.find_median(), 3.0);
        mf.add_num(6);
        assert_eq!(mf.find_median(), 3.5);
    }
    #[test]
    fn test7() {
        let mf = MedianFinder::new();
        mf.add_num(6);
        assert_eq!(mf.find_median(), 6.0);
        mf.add_num(10);
        assert_eq!(mf.find_median(), 8.0);
        mf.add_num(2);
        assert_eq!(mf.find_median(), 6.0);
        mf.add_num(6);
        assert_eq!(mf.find_median(), 6.0);
        mf.add_num(5);
        assert_eq!(mf.find_median(), 6.0);
        mf.add_num(0);
        assert_eq!(mf.find_median(), 5.5);
        mf.add_num(6);
        assert_eq!(mf.find_median(), 6.0);
        mf.add_num(3);
        assert_eq!(mf.find_median(), 5.5);
        mf.add_num(1);
        assert_eq!(mf.find_median(), 5.0);
        mf.add_num(0);
        assert_eq!(mf.find_median(), 4.0);
        mf.add_num(0);
        assert_eq!(mf.find_median(), 3.0);
    }
}
