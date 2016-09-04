//
// Created by Eugene Cherkasov on 03.09.2016.
// Copyright (c) 2014-2016 stage team. All rights reserved.
//

//! Math vector with basic operations
//! For now implemented for 2-dimensional space only.

use std::ops::{Add, Sub, Mul};
use Number;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vector2<T: Number> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Vector2<T> where T: Number, <T as Add>::Output: Number {
    type Output = Vector2<<T as Add>::Output>;

    fn add(self, rhs: Vector2<T>) -> Vector2<<T as Add>::Output> {
        Vector2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T> Sub for Vector2<T> where T: Number, <T as Sub>::Output: Number {
    type Output = Vector2<<T as Sub>::Output>;

    fn sub(self, rhs: Vector2<T>) -> Vector2<<T as Sub>::Output> {
        Vector2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T> Mul<T> for Vector2<T> where T: Number, <T as Mul>::Output: Number {
    type Output = Vector2<<T as Mul>::Output>;

    fn mul(self, rhs: T) -> Vector2<<T as Mul>::Output> {
        Vector2 { x: self.x * rhs, y: self.y * rhs }
    }
}


#[cfg(test)]
mod tests {
    use vector::*;

    const VECTOR_1: Vector2<f32> = Vector2 { x: 10.0, y: 20.0 };
    const VECTOR_2: Vector2<f32> = Vector2 { x: 20.0, y: 20.0 };

    const SCALE: f32 = 2.0;

    #[test]
    fn test_equal() {
        let vec = VECTOR_1;
        assert_eq!( VECTOR_1 == VECTOR_2, false );
        assert_eq!( VECTOR_1 == vec, true );
    }

    #[test]
    fn test_ops() {
        let v_add = Vector2 { x: VECTOR_1.x + VECTOR_2.x, y: VECTOR_1.y + VECTOR_2.y };
        let v_sub = Vector2 { x: VECTOR_1.x - VECTOR_2.x, y: VECTOR_1.y - VECTOR_2.y };
        let v_mul = Vector2 { x: VECTOR_1.x * SCALE, y: VECTOR_1.y * SCALE };
        assert_eq!(v_add, VECTOR_1 + VECTOR_2);
        assert_eq!(v_sub, VECTOR_1 - VECTOR_2);
        assert_eq!(v_mul, VECTOR_1 * SCALE);
    }
}
