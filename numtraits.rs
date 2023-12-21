use core::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Not, Shl, Shr, BitAndAssign, BitOrAssign, BitXorAssign, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, ShlAssign, ShrAssign, Neg};
use core::fmt::Debug;

pub trait FullBitPrimitive: Copy {}
impl FullBitPrimitive for u8 {}
impl FullBitPrimitive for i8 {}
impl FullBitPrimitive for u16 {}
impl FullBitPrimitive for i16 {}
impl FullBitPrimitive for u32 {}
impl FullBitPrimitive for i32 {}
impl FullBitPrimitive for u64 {}
impl FullBitPrimitive for i64 {}
impl FullBitPrimitive for usize {}
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
    AddAssign +
    Sub<Output=Self> +
    SubAssign +
    Mul<Output=Self> +
    MulAssign +
    Div<Output=Self> +
    DivAssign +
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
    Rem<Output=Self> +
    RemAssign +
    Copy // TODO: Remove this?
{
    fn is_even(self) -> bool {
        self % (Self::one() + Self::one()) == zero()
    }
    fn is_odd(self) -> bool {
        self % (Self::one() + Self::one()) != zero()
    }
    fn next_multiple_of(self, rhs: Self) -> Self {
        if rhs + one() == zero() {
            return self;
        }

        let r = self % rhs;
        let m = if (r > zero() && rhs < zero()) || (r < zero() && rhs > zero()) {
            r + rhs
        } else {
            r
        };

        if m == zero() {
            self
        } else {
            self + (rhs - m)
        }
    }
    fn div_ceil(self, rhs: Self) -> Self {
        let d = self / rhs;
        let r = self % rhs;
        if (r > zero() && rhs > zero()) || (r < zero() && rhs < zero()) {
            d + one()
        } else {
            d
        }
    }
    fn div_floor(self, rhs: Self) -> Self {
        let d = self / rhs;
        let r = self % rhs;
        if (r > zero() && rhs < zero()) || (r < zero() && rhs > zero()) {
            d - one()
        } else {
            d
        }
    }
}

impl Integer for u8 {
    fn next_multiple_of(self, rhs: Self) -> Self { self.next_multiple_of(rhs) }
    fn div_ceil(self, rhs: Self) -> Self { self.div_ceil(rhs) }
    fn div_floor(self, rhs: Self) -> Self { self / rhs }
}
impl Integer for i8 {}
impl Integer for u16 {
    fn next_multiple_of(self, rhs: Self) -> Self { self.next_multiple_of(rhs) }
    fn div_ceil(self, rhs: Self) -> Self { self.div_ceil(rhs) }
    fn div_floor(self, rhs: Self) -> Self { self / rhs }
}
impl Integer for i16 {}
impl Integer for u32 {
    fn next_multiple_of(self, rhs: Self) -> Self { self.next_multiple_of(rhs) }
    fn div_ceil(self, rhs: Self) -> Self { self.div_ceil(rhs) }
    fn div_floor(self, rhs: Self) -> Self { self / rhs }
}
impl Integer for i32 {}
impl Integer for u64 {
    fn next_multiple_of(self, rhs: Self) -> Self { self.next_multiple_of(rhs) }
    fn div_ceil(self, rhs: Self) -> Self { self.div_ceil(rhs) }
    fn div_floor(self, rhs: Self) -> Self { self / rhs }
}
impl Integer for i64 {}
impl Integer for u128 {
    fn next_multiple_of(self, rhs: Self) -> Self { self.next_multiple_of(rhs) }
    fn div_ceil(self, rhs: Self) -> Self { self.div_ceil(rhs) }
    fn div_floor(self, rhs: Self) -> Self { self / rhs }
}
impl Integer for i128 {}
impl Integer for usize {
    fn next_multiple_of(self, rhs: Self) -> Self { self.next_multiple_of(rhs) }
    fn div_ceil(self, rhs: Self) -> Self { self.div_ceil(rhs) }
    fn div_floor(self, rhs: Self) -> Self { self / rhs }
}
impl Integer for isize {}

pub trait Signed:
    Real +
    Neg<Output=Self>
{}

impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for i128 {}
impl Signed for isize {}
impl Signed for f32 {}
impl Signed for f64 {}

pub trait BitOps:
    FullBitPrimitive +
    BitAnd<Output=Self> +
    BitAndAssign +
    BitOr<Output=Self> +
    BitOrAssign +
    BitXor<Output=Self> +
    BitXorAssign +
    Not<Output=Self> +
    Shl<usize, Output=Self> +
    ShlAssign +
    Shr<usize, Output=Self> +
    ShrAssign
{}

impl BitOps for u8 {}
impl BitOps for i8 {}
impl BitOps for u16 {}
impl BitOps for i16 {}
impl BitOps for u32 {}
impl BitOps for i32 {}
impl BitOps for u64 {}
impl BitOps for i64 {}
impl BitOps for usize {}
impl BitOps for isize {}
