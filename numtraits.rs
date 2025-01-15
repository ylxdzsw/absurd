use core::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitOr, BitXor, Not, Shl, Shr, BitAndAssign, BitOrAssign, BitXorAssign, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign, ShlAssign, ShrAssign};
use core::fmt::Debug;

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

pub trait Arithmetic:
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
    Rem<Output=Self> +
    RemAssign +
    PartialOrd +
    PartialEq
{}

impl<T> Arithmetic for T where T:
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
    Rem<Output=Self> +
    RemAssign +
    PartialOrd +
    PartialEq
{}

pub trait BitOps:
    Sized +
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

impl<T> BitOps for T where T:
    Sized +
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

pub trait Integer:
    Arithmetic +
    BitOps +
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

mod tests {
    #[test]
    fn test_u128() {
        let a = 0b1010u128;
        let b = 0b1100u128;
        assert_eq!(a & b, 0b1000u128);
        assert_eq!(a | b, 0b1110u128);
        assert_eq!(a ^ b, 0b0110u128);
        assert_eq!((a << 99) >> 100, 0b101u128);

        assert_eq!(a.next_multiple_of(3), 12);
        assert_eq!(a.next_multiple_of(10), 10);
        assert_eq!(a.div_ceil(3), 4);
        assert_eq!(super::Integer::div_floor(a, 3), 3);
    }
}
