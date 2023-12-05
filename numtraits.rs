use core::ops::{Add, Sub, Mul, Div, Rem};
use core::fmt::Debug;

pub trait FullBitPrimitive {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for u8 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for i8 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for u16 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for i16 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for u32 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for i32 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for u64 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for i64 {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for usize {}

#[cfg(any(target_arch="x86", target_arch="x86_64", target_arch="arm", target_arch="aarch64", target_arch="wasm32"))]
impl FullBitPrimitive for isize {}

pub trait One { fn one() -> Self; }
pub fn one<T: One>() -> T { T::one() }
impl One for u8 { fn one() -> Self { 1 } }
impl One for i8 { fn one() -> Self { 1 } }
impl One for u16 { fn one() -> Self { 1 } }
impl One for i16 { fn one() -> Self { 1 } }
impl One for u32 { fn one() -> Self { 1 } }
impl One for i32 { fn one() -> Self { 1 } }
impl One for u64 { fn one() -> Self { 1 } }
impl One for i64 { fn one() -> Self { 1 } }
impl One for u128 { fn one() -> Self { 1 } }
impl One for i128 { fn one() -> Self { 1 } }
impl One for usize { fn one() -> Self { 1 } }
impl One for isize { fn one() -> Self { 1 } }
impl One for f32 { fn one() -> Self { 1.0 } }
impl One for f64 { fn one() -> Self { 1.0 } }

pub trait Zero { fn zero() -> Self; }
pub fn zero<T: Zero>() -> T { T::zero() }
impl Zero for u8 { fn zero() -> Self { 0 } }
impl Zero for i8 { fn zero() -> Self { 0 } }
impl Zero for u16 { fn zero() -> Self { 0 } }
impl Zero for i16 { fn zero() -> Self { 0 } }
impl Zero for u32 { fn zero() -> Self { 0 } }
impl Zero for i32 { fn zero() -> Self { 0 } }
impl Zero for u64 { fn zero() -> Self { 0 } }
impl Zero for i64 { fn zero() -> Self { 0 } }
impl Zero for u128 { fn zero() -> Self { 0 } }
impl Zero for i128 { fn zero() -> Self { 0 } }
impl Zero for usize { fn zero() -> Self { 0 } }
impl Zero for isize { fn zero() -> Self { 0 } }
impl Zero for f32 { fn zero() -> Self { 0.0 } }
impl Zero for f64 { fn zero() -> Self { 0.0 } }

pub trait Real:
    Sized +
    Debug +
    One +
    Zero +
    Add<Output=Self> +
    Sub<Output=Self> +
    Mul<Output=Self> +
    Div<Output=Self> +
    PartialOrd +
    PartialEq
{}

impl Real for u8 {}
impl Real for i8 {}
impl Real for u16 {}
impl Real for i16 {}
impl Real for u32 {}
impl Real for i32 {}
impl Real for u64 {}
impl Real for i64 {}
impl Real for u128 {}
impl Real for i128 {}
impl Real for usize {}
impl Real for isize {}
impl Real for f32 {}
impl Real for f64 {}

pub trait Integer:
    Real +
    Rem<Output=Self>
{}
