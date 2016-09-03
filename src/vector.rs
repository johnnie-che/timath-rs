//
// Created by Eugene Cherkasov on 03.09.2016.
// Copyright (c) 2014-2016 stage team. All rights reserved.
//

use Number;

#[derive(PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub struct Vector2<T: Number> {
    pub x: T,
    pub y: T,
}


#[cfg(test)]
mod tests {
    use vector::*;

    const VECTOR_1: Vector2<f32> = Vector2 { x: 10.0, y: 20.0 };
    const VECTOR_2: Vector2<f32> = Vector2 { x: 20.0, y: 20.0 };

    #[test]
    fn test_equal() {
        let vec = VECTOR_1;
        assert_eq!( VECTOR_1 == VECTOR_2, false );
        assert_eq!( VECTOR_1 == vec, true );
    }
}
