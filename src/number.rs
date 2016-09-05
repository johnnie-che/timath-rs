//
// Created by Eugene Cherkasov on 01.09.2016.
// Copyright (c) 2014-2016 stage team. All rights reserved.
//

use std::ops::{Add, Sub, Mul};

pub trait Number: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> +
                  Copy + Clone { }

impl Number for f32 { }
impl Number for f64 { }
impl Number for i8  { }
impl Number for i16 { }
impl Number for i32 { }
impl Number for i64 { }
impl Number for u8  { }
impl Number for u16 { }
impl Number for u32 { }
impl Number for u64 { }

pub trait Float: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> +
                 Copy + Clone {
    fn sqrt(self) -> Self;
}

impl Float for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}

impl Float for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }
}
