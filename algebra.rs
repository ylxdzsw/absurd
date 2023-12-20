/// A `Magma` is a set with a closed binary operation
pub trait Magma {
    type T;
    fn op(lhs: Self::T, rhs: Self::T) -> Self::T;
}

pub trait MagmaInplace {
    fn op(&mut self, rhs: &Self);
}

/// A `SemiGroup` is a `Magma` where the binary operation is associative
pub trait SemiGroup: Magma {}

pub trait SemiGroupInplace: MagmaInplace {}

/// A `Monoid` is a `SemiGroup` with an identity element, such that `a * identity() == a`
pub trait Monoid: SemiGroup {
    fn identity() -> Self::T;
}

pub trait MonoidInplace: SemiGroupInplace {
    fn identity() -> Self;
}

/// A `Group` is a `Monoid` with an inverse element, such that `a * a.inverse() == identity()`
pub trait Group: Monoid {
    fn inverse(x: Self::T) -> Self::T;
}

pub trait GroupInplace: MonoidInplace {
    fn inverse(&mut self);
}

/// An `AbelianGroup` is a `Group` where the binary operation is commutative
pub trait AbelianGroup: Group {}

pub trait AbelianGroupInplace: GroupInplace {}

/// A `Ring` is an `AbelianGroup` with an additional binary operation that is associative, distributive over the first operation, and has an identity element
pub trait Ring {
    type T;
    fn zero() -> Self::T;
    fn add(lhs: Self::T, rhs: Self::T) -> Self::T;
    fn neg(x: Self::T) -> Self::T;
    fn sub(lhs: Self::T, rhs: Self::T) -> Self::T {
        Self::add(lhs, Self::neg(rhs))
    }
    fn one() -> Self::T;
    fn mul(lhs: Self::T, rhs: Self::T) -> Self::T;
}

pub trait RingInplace {
    fn zero() -> Self;
    fn add(&mut self, rhs: &Self);
    fn neg(&mut self);
    fn sub(&mut self, rhs: &Self);
    fn one() -> Self;
    fn mul(&mut self, rhs: &Self);
}

/// A `CommutativeRing` is a `Ring` where the second binary operation is also commutative
pub trait CommutativeRing: Ring {}

pub trait CommutativeRingInplace: RingInplace {}

/// A `Field` is a `CommutativeRing` where the second binary operation has an inverse element
pub trait Field: CommutativeRing {
    fn inverse(x: Self::T) -> Self::T;
    fn div(lhs: Self::T, rhs: Self::T) -> Self::T {
        Self::mul(lhs, Self::inverse(rhs))
    }
}

pub trait FieldInplace: CommutativeRingInplace {
    fn inverse(&mut self);
    fn div(&mut self, rhs: &Self);
}

/// An R-Module is an Abelian group that can be scaled by elements of a ring R
pub trait Module<R: Ring>: AbelianGroup {
    fn scale(lhs: R::T, rhs: Self::T) -> Self::T;
}

pub trait ModuleInplace<R: Ring>: AbelianGroupInplace {
    fn scale(&mut self, lhs: R::T);
}

/// A `VectorSpace` is a `Module` where the scaling ring is a `Field`
pub trait VectorSpace<F: Field>: Module<F> {}

pub trait VectorSpaceInplace<F: Field>: ModuleInplace<F> {}
