use std::cell::RefCell;

/// # Design Circular Queue
///
/// Design your implementation of the circular queue.
/// The circular queue is a linear data structure in which the operations are performed based on
/// FIFO (First In First Out) principle and the last position is connected back to the first
/// position to make a circle.
/// It is also called "Ring Buffer".
///
/// One of the benefits of the circular queue is that we can make use of the spaces in front of the
/// queue.
/// In a normal queue, once the queue becomes full,
/// we cannot insert the next element even if there is a space in front of the queue.
/// But using the circular queue, we can use the space to store new values.
///
/// Implement the `MyCircularQueue` class:
///
/// - `MyCircularQueue(k)` Initializes the object with the size of the queue to be `k`.
/// - `int Front()` Gets the front item from the queue. If the queue is empty, return `-1`.
/// - `int Rear()` Gets the last item from the queue. If the queue is empty, return `-1`.
/// - `boolean enQueue(int value)` Inserts an element into the circular queue.
///   Return `true` if the operation is successful.
/// - `boolean deQueue()` Deletes an element from the circular queue.
///   Return `true` if the operation is successful.
/// - `boolean isEmpty()` Checks whether the circular queue is empty or not.
/// - `boolean isFull()` Checks whether the circular queue is full or not.
///
/// __Constraints:__
///
/// - `1 <= k <= 1000`
/// - `0 <= value <= 1000`
/// - At most `3000` calls will be made to `enQueue`, `deQueue`, `Front`, `Rear`, `isEmpty`, and `isFull`.
///
/// __Follow up:__ Could you solve the problem without using the built-in queue?
///
/// https://leetcode.com/explore/featured/card/april-leetcoding-challenge-2021/593/week-1-april-1st-april-7th/3696/
#[derive(Debug)]
struct MyCircularQueue {
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
    data: Vec<i32>,
}
impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let capacity = k as usize;
        Self {
            head: 0,
            tail: 0,
            size: 0,
            capacity,
            data: vec![-1; capacity],
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            if !self.is_empty() {
                if self.tail == self.capacity - 1 {
                    self.tail = 0;
                } else {
                    self.tail += 1;
                }
            }
            self.data[self.tail] = value;
            self.size += 1;
            true
        }
    }
    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.data[self.head] = -1;
            self.size -= 1;
            if !self.is_empty() {
                if self.head == self.capacity - 1 {
                    self.head = 0;
                } else {
                    self.head += 1;
                }
            }
            true
        }
    }
    fn front(&self) -> i32 {
        self.data[self.head]
    }
    fn rear(&self) -> i32 {
        self.data[self.tail]
    }
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

struct MyCircularQueueLinearDequeue {
    capacity: usize,
    data: RefCell<Vec<i32>>,
}
impl MyCircularQueueLinearDequeue {
    fn new(k: i32) -> Self {
        let capacity = k as usize;
        Self {
            capacity,
            data: RefCell::new(Vec::with_capacity(capacity)),
        }
    }
    fn en_queue(&self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.data.borrow_mut().push(value);
            true
        }
    }
    fn de_queue(&self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.data.borrow_mut().remove(0);
            true
        }
    }
    fn front(&self) -> i32 {
        self.data.borrow().first().unwrap_or(&-1).to_owned()
    }
    fn rear(&self) -> i32 {
        self.data.borrow().last().unwrap_or(&-1).to_owned()
    }
    fn is_empty(&self) -> bool {
        self.data.borrow().is_empty()
    }
    fn is_full(&self) -> bool {
        self.data.borrow().len() >= self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut q = MyCircularQueue::new(3);
        assert!(q.is_empty());
        assert!(q.en_queue(1));
        assert!(!q.is_empty());
        assert!(q.en_queue(2));
        assert!(q.en_queue(3));
        assert!(!q.en_queue(4));
        assert_eq!(q.front(), 1);
        assert_eq!(q.rear(), 3);
        assert!(q.is_full());
        assert!(q.de_queue());
        assert!(q.en_queue(4));
        assert_eq!(q.front(), 2);
        assert_eq!(q.rear(), 4);
    }

    #[test]
    fn test11() {
        let mut q = MyCircularQueue::new(8);
        assert!(q.en_queue(3));
        assert!(q.en_queue(9));
        assert!(q.en_queue(5));
        assert!(q.en_queue(0));
        assert!(q.de_queue());
        assert!(q.de_queue());
        assert!(!q.is_empty());
        assert!(!q.is_empty());
        assert_eq!(q.rear(), 0);
        assert!(q.de_queue());
        assert!(q.de_queue());
        assert!(!q.de_queue());
    }
    #[test]
    fn test31() {
        let mut q = MyCircularQueue::new(6);
        assert!(q.en_queue(6));
        assert_eq!(q.rear(), 6);
        assert_eq!(q.rear(), 6);
        assert!(q.de_queue());
        assert!(q.en_queue(5));
        assert_eq!(q.rear(), 5);
        assert!(q.de_queue());
        assert_eq!(q.front(), -1);
        assert!(!q.de_queue());
        assert!(!q.de_queue());
        assert!(!q.de_queue());
    }
    #[test]
    fn test33() {
        let mut q = MyCircularQueue::new(3);
        assert!(q.en_queue(3));
        assert!(q.de_queue());
        assert_eq!(q.front(), -1);
        assert!(!q.de_queue());
        assert_eq!(q.front(), -1);
        assert_eq!(q.rear(), -1);
        assert!(q.en_queue(0));
        assert!(!q.is_full());
        assert!(q.de_queue());
        assert_eq!(q.rear(), -1);
        assert!(q.en_queue(3));
    }

    mod performance {
        use super::*;

        #[test]
        fn test_enqueue_deque() {
            let max = 100_000;
            let mut q = MyCircularQueue::new(max);
            for i in 0..max {
                assert!(q.en_queue(i));
            }
            for i in 0..max {
                assert!(q.de_queue());
            }
        }
    }
}
