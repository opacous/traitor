use {crate::prefix::*, traitor_macros::traitor_ops};

/// Basically newtyping for builtin ops, to allow us to provide blanket impls

pub trait Add<Other = Self> {
    type Output;

    fn add(self, rhs: Other) -> Self::Output;
}

pub trait Sub<Other = Self> {
    type Output;

    fn sub(self, rhs: Other) -> Self::Output;
}

pub trait Neg {
    type Output;

    fn neg(self) -> Self::Output;
}

pub trait Inv {
    type Output;

    fn inv(self) -> Self::Output;
}

pub trait Mul<Other = Self> {
    type Output;

    fn mul(self, rhs: Other) -> Self::Output;
}

pub trait Div<Other = Self> {
    type Output;

    fn div(self, rhs: Other) -> Self::Output;
}

pub trait RefAdd = where for<'a> &'a Self: Sized + Add<&'a Self, Output = Self>;
pub trait RefSub = where for<'a> &'a Self: Sized + Sub<&'a Self, Output = Self>;
pub trait RefNeg = where for<'a> &'a Self: Sized + Neg<Output = Self>;

pub trait RefMul = where for<'a> &'a Self: Sized + Mul<&'a Self, Output = Self>;
pub trait RefDiv = where for<'a> &'a Self: Sized + Div<&'a Self, Output = Self>;
pub trait RefInv = where for<'a> &'a Self: Sized + Inv<Output = Self>;
