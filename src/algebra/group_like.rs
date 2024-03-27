pub use self::{additive::*, multiplicative::*};

///Traits for group-like structures using addition
pub mod additive {
    use {
        super::{repeated_doubling, repeated_doubling_neg},
        crate::algebra::{IntegerSubset, Natural, Ring, Semiring},
        core::{convert::From, ops::Mul},
    };
    pub use {
        core::ops::{Add, AddAssign, Neg, Sub, SubAssign},
        num_traits::Zero,
    };

    #[allow(unused_imports)]
    use crate::algebra::Integer;

    pub trait AddAssociative {}

    pub trait AddCommutative {}

    pub trait MulN: AddSemigroup + Zero {
        #[inline]
        fn mul_n<N: Natural>(self, n: N) -> Self {
            trait Helper1<Z: Natural>: AddSemigroup + Zero {
                fn _mul1(self, n: Z) -> Self;
            }
            impl<H: AddSemigroup + Zero, Z: Natural> Helper1<Z> for H {
                #[inline]
                default fn _mul1(self, n: Z) -> Self {
                    self._mul2(n)
                }
            }
            impl<H: AddSemigroup + Zero + Mul<Z, Output = H>, Z: Natural> Helper1<Z> for H {
                #[inline]
                fn _mul1(self, n: Z) -> Self {
                    self * n
                }
            }

            trait Helper2<Z: Natural>: AddSemigroup + Zero {
                fn _mul2(self, n: Z) -> Self;
            }
            impl<H: AddSemigroup + Zero, Z: Natural> Helper2<Z> for H {
                #[inline]
                default fn _mul2(self, n: Z) -> Self {
                    repeated_doubling(self, n)
                }
            }
            impl<H: Semiring + From<Z>, Z: Natural> Helper2<Z> for H {
                #[inline]
                fn _mul2(self, n: Z) -> Self {
                    H::from(n) * self
                }
            }

            self._mul1(n)
        }
    }

    impl<G: AddSemigroup + Zero> MulN for G {}

    pub trait MulZ: AddMonoid + Negatable {
        #[inline]
        fn mul_z<N: IntegerSubset>(self, n: N) -> Self {
            trait Helper1<Z: IntegerSubset>: AddMonoid + Negatable {
                fn _mul1(self, n: Z) -> Self;
            }
            impl<H: AddMonoid + Negatable, Z: IntegerSubset> Helper1<Z> for H {
                #[inline]
                default fn _mul1(self, n: Z) -> Self {
                    self._mul2(n)
                }
            }
            impl<H: AddMonoid + Negatable + Mul<Z, Output = H>, Z: IntegerSubset> Helper1<Z> for H {
                #[inline]
                fn _mul1(self, n: Z) -> Self {
                    self * n
                }
            }

            trait Helper2<Z: IntegerSubset>: AddSemigroup + Zero {
                fn _mul2(self, n: Z) -> Self;
            }
            impl<H: AddMonoid + Negatable, Z: IntegerSubset> Helper2<Z> for H {
                #[inline]
                default fn _mul2(self, n: Z) -> Self {
                    repeated_doubling_neg(self, n)
                }
            }
            impl<H: Ring + From<Z>, Z: IntegerSubset> Helper2<Z> for H {
                #[inline]
                fn _mul2(self, n: Z) -> Self {
                    H::from(n) * self
                }
            }

            self._mul1(n)
        }
    }

    impl<G: AddMonoid + Negatable> MulZ for G {}

    ///A set with an fully described additive inverse
    pub trait Negatable =
        Sized + Clone + Neg<Output = Self> + Sub<Self, Output = Self> + SubAssign<Self>;

    ///A set with an addition operation
    pub trait AddMagma = Sized + Clone + Add<Self, Output = Self> + AddAssign<Self>;

    ///An associative additive magma
    pub trait AddSemigroup = AddMagma + AddAssociative;
    ///An additive semigroup with an identity element
    pub trait AddMonoid = AddSemigroup + Zero + MulN;
    ///An additive magma with an inverse operation and identity
    pub trait AddLoop = AddMagma + Negatable + Zero;
    ///An additive monoid with an inverse operation
    pub trait AddGroup = AddMagma + AddAssociative + Negatable + Zero + MulZ;
    ///A commutative additive group
    pub trait AddAbelianGroup = AddGroup + AddCommutative;
}

///Traits for group-like structures using Multiplication
pub mod multiplicative {
    use {
        super::{repeated_squaring, repeated_squaring_inv},
        crate::algebra::{IntegerSubset, Natural},
        num_traits::Pow,
    };
    pub use {
        core::ops::{Div, DivAssign, Mul, MulAssign},
        num_traits::{Inv, One},
    };

    #[allow(unused_imports)]
    use crate::algebra::Integer;

    pub trait MulAssociative {}
    pub trait MulCommutative {}

    pub trait PowN: MulSemigroup + One {
        #[inline]
        fn pow_n<N: Natural>(self, n: N) -> Self {
            trait Helper<Z: Natural>: MulSemigroup + One {
                fn _pow_n(self, n: Z) -> Self;
            }
            impl<G: MulSemigroup + One, Z: Natural> Helper<Z> for G {
                #[inline]
                default fn _pow_n(self, n: Z) -> Self {
                    repeated_squaring(self, n)
                }
            }
            impl<G: MulSemigroup + One + Pow<Z, Output = Self>, Z: Natural> Helper<Z> for G {
                #[inline]
                fn _pow_n(self, n: Z) -> Self {
                    self.pow(n)
                }
            }

            self._pow_n(n)
        }
    }
    impl<G: MulSemigroup + One> PowN for G {}

    pub trait PowZ: MulMonoid + Invertable {
        #[inline]
        fn pow_z<Z: IntegerSubset>(self, n: Z) -> Self {
            trait Helper<N: IntegerSubset>: MulMonoid + Invertable {
                fn _pow_z(self, n: N) -> Self;
            }
            impl<G: MulMonoid + Invertable, N: IntegerSubset> Helper<N> for G {
                #[inline]
                default fn _pow_z(self, n: N) -> Self {
                    repeated_squaring_inv(self, n)
                }
            }
            impl<G: MulMonoid + Invertable + Pow<N, Output = Self>, N: IntegerSubset> Helper<N> for G {
                #[inline]
                fn _pow_z(self, n: N) -> Self {
                    self.pow(n)
                }
            }

            self._pow_z(n)
        }
    }
    impl<G: MulMonoid + Invertable> PowZ for G {}

    ///A set with an fully described multiplicative inverse
    pub trait Invertable =
        Sized + Clone + Inv<Output = Self> + Div<Self, Output = Self> + DivAssign<Self>;

    ///A set with a multiplication operation
    pub trait MulMagma = Sized + Clone + Mul<Self, Output = Self> + MulAssign<Self>;

    ///An associative multiplicative magma
    pub trait MulSemigroup = MulMagma + MulAssociative;
    ///A multiplicative semigroup with an identity element
    pub trait MulMonoid = MulSemigroup + One + PowN;
    ///A multiplicative magma with an inverse operation and identity
    pub trait MulLoop = MulMagma + Invertable + One;
    ///A multiplicative monoid with an inverse operation
    pub trait MulGroup = MulMagma + MulAssociative + Invertable + One + PowZ;
    ///A commutative multiplicative group
    pub trait MulAbelianGroup = MulGroup + MulCommutative;
}

use crate::algebra::{IntegerSubset, Natural};

trait IsZero: Sized {
    fn _is_zero(&self) -> bool;
}
impl<Z> IsZero for Z {
    #[inline(always)]
    default fn _is_zero(&self) -> bool {
        false
    }
}
impl<Z: Zero> IsZero for Z {
    #[inline(always)]
    fn _is_zero(&self) -> bool {
        self.is_zero()
    }
}

fn mul_pow_helper<E: Natural, R: Clone, Op: Fn(R, R) -> R>(mut b: R, mut p: E, op: Op) -> R {
    //repeated squaring/doubling
    let mut res = b.clone();
    p -= E::one();
    while !p.is_zero() {
        if p.even() {
            //if the exponent (or multiple) is even, we can factor out the two
            //ie b^2p = (b^2)^p or 2p*b = p*2b
            b = op(b.clone(), b);
            p = p.div_two();
        } else {
            //else b^(p+1)=(b^p)*b or (p+1)*b = p*b + b
            res = op(res, b.clone());
            p -= E::one();
        }
    }
    res
}

#[inline]
pub fn repeated_squaring_inv<E: IntegerSubset, R: MulGroup + Clone>(b: R, p: E) -> R {
    if p.negative() {
        repeated_squaring(b, p.abs_unsigned()).inv()
    } else {
        repeated_squaring(b, p.as_unsigned())
    }
}

#[inline]
pub fn repeated_squaring<E: Natural, R: MulMonoid + Clone>(b: R, p: E) -> R {
    if p.is_zero() {
        if b._is_zero() {
            panic!("Attempted to raise 0^0")
        }
        R::one()
    } else {
        mul_pow_helper(b, p, R::mul)
    }
}

///Multiplies a [monoid](AddMonoid) by a positive integer using negation and repeated doublings
#[inline]
pub fn repeated_doubling_neg<E: IntegerSubset, R: AddGroup>(b: R, p: E) -> R {
    if p.negative() {
        -repeated_doubling(b, p.abs_unsigned())
    } else {
        repeated_doubling(b, p.as_unsigned())
    }
}

///Multiplies a [monoid](AddMonoid) by a positive integer using repeated doublings
#[inline]
pub fn repeated_doubling<E: Natural, R: AddMonoid>(b: R, p: E) -> R {
    if p.is_zero() {
        R::zero()
    } else {
        mul_pow_helper(b, p, R::add)
    }
}

macro_rules! impl_props {
    ($($z:ty)*; $($f:ty)*) => {
        $(impl_props!(@int $z);)*
        $(impl_props!(@float $f);)*
    };

    (@int $z:ty) => {
        impl_props!(@props $z);
        impl_props!(@props core::num::Wrapping<$z>);
    };
    (@float $f:ty) => {impl_props!(@props $f);};
    (@props $t:ty) => {

        impl AddAssociative for $t {}
        impl AddCommutative for $t {}
        impl MulAssociative for $t {}
        impl MulCommutative for $t {}
    };
}

impl_props! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128; f32 f64 &f32 &f64}

#[cfg(std)]
impl<'a> AddAssociative for ::std::borrow::Cow<'a, str> {}
