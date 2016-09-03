//
// Created by Eugene Cherkasov on 26.08.2016.
// Copyright (c) 2014-2016 stage team. All rights reserved.
//

//! Point contains coordinates only. Supports basic math operations.
//! For now implemented for 2-dimensional space only.

use std::ops::{Add, Sub};
use Number;

/// A point in 2-dimensional space
#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Point2<T: Number> {
    pub x: T,
    pub y: T
}

impl<T> Add for Point2<T> where T: Number, <T as Add>::Output: Number {
    type Output = Point2<<T as Add>::Output>;

    fn add(self, rhs: Point2<T>) -> Point2<<T as Add>::Output> {
        Point2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T> Sub for Point2<T> where T: Number, <T as Sub>::Output: Number {
    type Output = Point2<<T as Sub>::Output>;

    fn sub(self, rhs: Point2<T>) -> Point2<<T as Sub>::Output> {
        Point2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}


#[cfg(test)]
mod tests {
    use point::*;

    const POINT_1: Point2<f32> = Point2{ x: 10.0, y: 20.0 };
    const POINT_2: Point2<f32> = Point2{ x: 20.0, y: 20.0 };

    #[test]
    fn test_equal() {
        assert_eq!( POINT_1 == POINT_2, false );
    }

    #[test]
    fn test_copy() {
        let p = POINT_1;
        assert_eq!(p, POINT_1);
    }

    #[test]
    fn test_ops() {
        let p_add = Point2 { x: POINT_1.x + POINT_2.x, y: POINT_1.y + POINT_2.y };
        let p_sub = Point2 { x: POINT_1.x - POINT_2.x, y: POINT_1.y - POINT_2.y };
        assert_eq!(p_add, POINT_1 + POINT_2);
        assert_eq!(p_sub, POINT_1 - POINT_2);
    }
}
