pub use core::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::{algebra::*, analysis::ComplexRing};

pub trait SesquilinearForm<R: UnitalRing, M: RingModule<R>> {
    fn product_of(&self, v1: M, v2: M) -> R;
    ///
    #[inline]
    fn sigma(&self, x: R) -> R {
        x
    }

    #[inline]
    fn sigma_inv(&self, x: R) -> R {
        x
    }

    #[inline]
    fn square(&self, x: M) -> R {
        self.product_of(x.clone(), x)
    }

    #[inline]
    fn is_null(&self, x: M) -> bool {
        self.square(x).is_zero()
    }

    ///
    #[inline]
    fn orthogonal(&self, x: M, y: M) -> bool {
        self.product_of(x, y).is_zero()
    }
    ///
    #[inline]
    fn orth_comp(&self, x: M, y: M) -> M
    where
        R: DivisionRing,
    {
        y.clone() - self.par_comp(x, y)
    }

    ///
    #[inline]
    fn par_comp(&self, x: M, y: M) -> M
    where
        R: DivisionRing,
    {
        let l = self.product_of(y, x.clone()) * self.square(x.clone()).inv();
        x * l
    }
}

///A [SesquilinearForm] where `x•y = 0` implies `y•x = 0`
pub trait ReflexiveForm<R: UnitalRing, M: RingModule<R>>: SesquilinearForm<R, M> {}
///
pub trait SymSesquilinearForm<R: UnitalRing, M: RingModule<R>>: ReflexiveForm<R, M> {}
///
pub trait SkewSesquilinearForm<R: UnitalRing, M: RingModule<R>>: ReflexiveForm<R, M> {}

pub trait BilinearForm<R: UnitalRing, M: RingModule<R>>: SesquilinearForm<R, M> {}

pub trait SymmetricForm<R, M> = BilinearForm<R, M> + SymSesquilinearForm<R, M>
where
    R: UnitalRing,
    M: RingModule<R>;

pub trait SkewSymmetricForm<R, M> = BilinearForm<R, M> + SkewSesquilinearForm<R, M>
where
    R: UnitalRing,
    M: RingModule<R>;

pub trait ComplexSesquilinearForm<R: ComplexRing, M: RingModule<R>>:
    SesquilinearForm<R, M>
{
}

pub trait HermitianForm<R, M> = ComplexSesquilinearForm<R, M> + SymSesquilinearForm<R, M>
where
    R: ComplexRing,
    M: RingModule<R>;

pub trait SkewHermitianForm<R, M> = ComplexSesquilinearForm<R, M> + SkewSesquilinearForm<R, M>
where
    R: ComplexRing,
    M: RingModule<R>;

pub trait RingModule<K: UnitalRing> =
    AddAbelianGroup + Mul<K, Output = Self> + MulAssign<K> + Distributive<K>;

pub trait RingAlgebra<K: UnitalRing> = RingModule<K> + MulMagma + Distributive;

pub trait UnitalRingAlgebra<K: UnitalRing> = RingAlgebra<K> + One;

pub trait AssociativeRingAlgebra<K: UnitalRing> = RingAlgebra<K> + MulAssociative;

pub trait VectorSpace<K: Field> = RingModule<K> + Div<K, Output = Self> + DivAssign<K>;

pub trait Algebra<K: Field> = VectorSpace<K> + MulMagma + Distributive;

pub trait UnitalAlgebra<K: Field> = Algebra<K> + One;

pub trait AssociativeAlgebra<K: Field> = Algebra<K> + MulAssociative;

pub trait AffineSpace<K: Field, V: VectorSpace<K>> = Sized
    + Clone
    + Sub<Self, Output = V>
    + Add<V, Output = Self>
    + Sub<V, Output = Self>
    + SubAssign<V>
    + AddAssign<V>;
