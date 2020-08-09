//! Render impls for numeric primitives

use crate::Render;
use std::fmt::{Result, Write};

macro_rules! simple_render_impl {
    ($t:ty) => {
        impl Render for $t {
            fn render_into<W: Write>(self, writer: &mut W) -> Result {
                write!(writer, "{}", self)
            }
        }
    };
}

simple_render_impl!(f32);
simple_render_impl!(f64);
simple_render_impl!(i128);
simple_render_impl!(i16);
simple_render_impl!(i32);
simple_render_impl!(i64);
simple_render_impl!(i8);
simple_render_impl!(isize);
simple_render_impl!(u128);
simple_render_impl!(u16);
simple_render_impl!(u32);
simple_render_impl!(u64);
simple_render_impl!(u8);
simple_render_impl!(usize);
