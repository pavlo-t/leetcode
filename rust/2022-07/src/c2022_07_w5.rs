#![allow(dead_code)]
//! \#251. Flatten 2D Vector
//! ========================
//!
//! <https://leetcode.com/problems/flatten-2d-vector>
//!
//! Design an iterator to flatten a 2D vector.
//! It should support the `next` and `hasNext` operations.
//!
//! Implement the `Vector2D` class:
//!
//! - `Vector2D(int[][] vec)` initializes the object with the 2D vector `vec`.
//! - `next()`                returns the next element from the 2D vector and moves the pointer one step forward.
//!                           You may assume that all the calls to `next` are valid.
//! - `hasNext()`             returns `true` if there are still some elements in the vector, and `false` otherwise.
//!
//! __Constraints:__
//!
//! - `0 <= vec.length <= 200`
//! - `0 <= vec[i].length <= 500`
//! - `-500 <= vec[i][j] <= 500`
//! - At most `100_000` calls will be made to `next` and `hasNext`.
//!
//! __Follow up:__ As an added challenge, try to code it using only [iterators in C++] or [iterators in Java].
//!
//! [iterators in C++]:http://www.cplusplus.com/reference/iterator/iterator
//! [iterators in Java]:http://docs.oracle.com/javase/7/docs/api/java/util/Iterator.html
//!
//! # Examples
//!
//! ```
//! # use c2022_07::c2022_07_w5::Vector2D;
//! # macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }
//! let mut vector2D = Vector2D::new(vv![[1, 2], [3], [4]]);
//! assert_eq!(vector2D.has_next(), true);
//! assert_eq!(vector2D.next(), 1);
//! assert_eq!(vector2D.has_next(), true);
//! assert_eq!(vector2D.next(), 2);
//! assert_eq!(vector2D.has_next(), true);
//! assert_eq!(vector2D.next(), 3);
//! assert_eq!(vector2D.has_next(), true);
//! assert_eq!(vector2D.next(), 4);
//! assert_eq!(vector2D.has_next(), false);
//! ```
//!
//! ```
//! # use c2022_07::c2022_07_w5::Vector2D;
//! let mut vector2D = Vector2D::new(vec![]);
//! assert_eq!(vector2D.has_next(), false);
//! ```
//!
//! ```
//! # use c2022_07::c2022_07_w5::Vector2D;
//! let mut vector2D = Vector2D::new(vec![vec![]]);
//! assert_eq!(vector2D.has_next(), false);
//! ```
//!
//! ```
//! # use c2022_07::c2022_07_w5::Vector2D;
//! # macro_rules! vv { ($($x:tt),*) => { vec![$(vec!$x),*] }; }
//! let mut vector2D = Vector2D::new(vv![[1], []]);
//! assert_eq!(vector2D.has_next(), true);
//! assert_eq!(vector2D.next(), 1);
//! assert_eq!(vector2D.has_next(), false);
//! ```
//!
//! ```
//! # use c2022_07::c2022_07_w5::Vector2D;
//! let mut vector2D = Vector2D::new(vec![vec![1; 500]; 200]);
//! for _ in 0..100_000 {
//!     assert_eq!(vector2D.has_next(), true);
//!     assert_eq!(vector2D.next(), 1);
//! }
//! assert_eq!(vector2D.has_next(), false);
//! ```

use std::cell::RefCell;
use std::iter::Peekable;
use std::vec::IntoIter;

pub struct Vector2D {
    it1: RefCell<Peekable<IntoIter<Vec<i32>>>>,
    it2: RefCell<Option<Peekable<IntoIter<i32>>>>,
}

impl Vector2D {
    pub fn new(vec: Vec<Vec<i32>>) -> Self {
        let mut it1 = vec.into_iter().peekable();
        let it2 = it1.next().map(|it| it.into_iter().peekable());

        Vector2D {
            it1: RefCell::new(it1),
            it2: RefCell::new(it2),
        }
    }

    pub fn next(&self) -> i32 {
        while !self.it2_has_next() && self.it1_has_next() {
            *self.it2.borrow_mut() = self.it1_next();
        }
        self.it2_next().unwrap()
    }

    pub fn has_next(&self) -> bool {
        while !self.it2_has_next() && self.it1_has_next() {
            *self.it2.borrow_mut() = self.it1_next();
        }
        self.it2_has_next()
    }

    fn it1_next(&self) -> Option<Peekable<IntoIter<i32>>> {
        self.it1
            .borrow_mut()
            .next()
            .map(|it| it.into_iter().peekable())
    }

    fn it1_has_next(&self) -> bool {
        self.it1.borrow_mut().peek().is_some()
    }

    fn it2_next(&self) -> Option<i32> {
        self.it2.borrow_mut().as_mut().and_then(|it| it.next())
    }

    fn it2_has_next(&self) -> bool {
        self.it2
            .borrow_mut()
            .as_mut()
            .map(|it| it.peek().is_some())
            .unwrap_or(false)
    }
}

/// Using iterators without internal mutability
pub mod v2 {
    use std::iter::Peekable;
    use std::vec::IntoIter;

    pub struct Vector2D {
        it1: Peekable<IntoIter<Vec<i32>>>,
        it2: Option<Peekable<IntoIter<i32>>>,
    }

    impl Vector2D {
        pub fn new(vec: Vec<Vec<i32>>) -> Self {
            let mut it1 = vec.into_iter().peekable();
            let it2 = it1.next().map(|it| it.into_iter().peekable());
            Vector2D { it1, it2 }
        }

        pub fn next(&mut self) -> i32 {
            while !self.it2_has_next() && self.it1_has_next() {
                self.it2 = self.it1_next();
            }
            self.it2_next().unwrap()
        }

        pub fn has_next(&mut self) -> bool {
            while !self.it2_has_next() && self.it1_has_next() {
                self.it2 = self.it1_next();
            }
            self.it2_has_next()
        }

        fn it1_next(&mut self) -> Option<Peekable<IntoIter<i32>>> {
            self.it1.next().map(|it| it.into_iter().peekable())
        }

        fn it1_has_next(&mut self) -> bool {
            self.it1.peek().is_some()
        }

        fn it2_next(&mut self) -> Option<i32> {
            self.it2.as_mut().and_then(|it| it.next())
        }

        fn it2_has_next(&mut self) -> bool {
            self.it2
                .as_mut()
                .map(|it| it.peek().is_some())
                .unwrap_or(false)
        }
    }
}

/// Using 2 pointers
pub mod v1 {
    use std::cell::RefCell;

    pub struct Vector2D {
        i1: RefCell<usize>,
        i2: RefCell<usize>,
        vec: Vec<Vec<i32>>,
    }

    impl Vector2D {
        pub fn new(vec: Vec<Vec<i32>>) -> Self {
            Vector2D {
                i1: RefCell::new(0),
                i2: RefCell::new(0),
                vec,
            }
        }

        pub fn next(&self) -> i32 {
            let i1 = *self.i1.borrow();
            let i2 = *self.i2.borrow();

            let result = self.vec[i1][i2];

            if i2 < self.vec[i1].len().saturating_sub(1) {
                *self.i2.borrow_mut() += 1;
            } else {
                *self.i1.borrow_mut() += 1;
                *self.i2.borrow_mut() = 0;
            }

            result
        }

        pub fn has_next(&self) -> bool {
            let i1 = *self.i1.borrow();
            let i2 = *self.i2.borrow();
            i1 < self.vec.len().saturating_sub(1)
                || (i1 < self.vec.len() && i2 < self.vec[i1].len())
        }
    }
}
