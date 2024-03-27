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

pub trait AddAssign<Rhs = Self> {
    fn add_assign(&mut self, rhs: Rhs);
}

pub trait SubAssign<Rhs = Self> {
    fn sub_assign(&mut self, rhs: Rhs);
}

pub trait MulAssign<Rhs = Self> {
    fn mul_assign(&mut self, rhs: Rhs);
}

pub trait DivAssign<Rhs = Self> {
    fn div_assign(&mut self, rhs: Rhs);
}

macro_rules! impl_add {
    ($($z:ty)*) => {
        $(
            impl Add<$z> for $z {
                type Output = $z;

                fn add(self, rhs: $z) -> Self::Output {
                    self + rhs
                }
            }

            impl<'a> Add<&'a $z> for &'a $z {
                type Output = $z;

                fn add(self, rhs: &'a $z) -> Self::Output {
                    self + rhs
                }
            }

            impl AddAssign<$z> for $z {
                fn add_assign(&mut self, rhs: $z) {
                    *self += rhs
                }
            }
        )*
    };
}

macro_rules! impl_sub {
    ($($z:ty)*) => {
        $(
            impl Sub<$z> for $z {
                type Output = $z;

                fn sub(self, rhs: $z) -> Self::Output {
                    self - rhs
                }
            }

            impl<'a> Sub<&'a $z> for &'a $z {
                type Output = $z;

                fn sub(self, rhs: &'a $z) -> Self::Output {
                    self - rhs
                }
            }

            impl SubAssign<$z> for $z {
                fn sub_assign(&mut self, rhs: $z) {
                    *self -= rhs
                }
            }
        )*
    };
}

macro_rules! impl_mul {
    ($($z:ty)*) => {
        $(
            impl Mul<$z> for $z {
                type Output = $z;

                fn mul(self, rhs: $z) -> Self::Output {
                    self * rhs
                }
            }

            impl<'a> Mul<&'a $z> for &'a $z {
                type Output = $z;

                fn mul(self, rhs: &'a $z) -> Self::Output {
                    self * rhs
                }
            }

            impl MulAssign<$z> for $z {
                fn mul_assign(&mut self, rhs: $z) {
                    *self *= rhs
                }
            }
        )*
    };
}

macro_rules! impl_div {
    ($($z:ty)*) => {
        $(
            impl Div<$z> for $z {
                type Output = $z;

                fn div(self, rhs: $z) -> Self::Output {
                    self / rhs
                }
            }

            impl<'a> Div<&'a $z> for &'a $z {
                type Output = $z;

                fn div(self, rhs: &'a $z) -> Self::Output {
                    self / rhs
                }
            }

            impl DivAssign<$z> for $z {
                fn div_assign(&mut self, rhs: $z) {
                    *self /= rhs
                }
            }
        )*
    };
}

impl_add! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}
impl_sub! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}
impl_mul! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}
impl_div! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64}

pub trait RefAdd = where for<'a> &'a Self: Sized + Add<&'a Self, Output = Self>;
pub trait RefSub = where for<'a> &'a Self: Sized + Sub<&'a Self, Output = Self>;
pub trait RefNeg = where for<'a> &'a Self: Sized + Neg<Output = Self>;

pub trait RefMul = where for<'a> &'a Self: Sized + Mul<&'a Self, Output = Self>;
pub trait RefDiv = where for<'a> &'a Self: Sized + Div<&'a Self, Output = Self>;
pub trait RefInv = where for<'a> &'a Self: Sized + Inv<Output = Self>;
