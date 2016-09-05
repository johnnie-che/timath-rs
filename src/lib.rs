//
// Created by Eugene Cherkasov on 25.08.2016.
// Copyright (c) 2014-2016 stage team. All rights reserved.
//

//! Contains primitive types
//!

pub mod point;
pub mod vector;
pub mod number;

pub use self::point::Point2;
pub use self::vector::Vector2;
pub use self::number::{Number, Float};