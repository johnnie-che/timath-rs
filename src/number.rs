//
// Created by Eugene Cherkasov on 01.09.2016.
// Copyright (c) 2014-2016 stage team. All rights reserved.
//

use std::ops::{Add, Sub};

pub trait Number: Add + Sub + Copy + Clone { }

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

