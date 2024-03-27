use {
    crate::{
        analysis::{Metric, Real, RealExponential},
        ops::*,
        Bound, B,
    },
    core::{iter, mem::MaybeUninit},
    num_traits::ToPrimitive,
    replace_with::replace_with_or_abort,
    traitor_macros::auto_gen_impl,
};

mod metric;
pub use metric::*;

mod inline;
pub use inline::*;

mod vec;
pub use vec::*;

mod smallvec;
pub use smallvec::*;

/// An array is a thing that permits random access at integer offsets.
pub trait Array: Sized {
    type Element;

    fn nth<'a>(&'a self, n: usize) -> Option<&'a Self::Element>;
    fn len(&self) -> usize;

    #[inline(always)]
    fn fold<A>(&self, start_value: A, mut f: impl FnMut(A, &Self::Element) -> A) -> A {
        let mut accumulated = start_value;
        for val in self.iter() {
            accumulated = f(accumulated, val);
        }
        accumulated
    }

    #[inline(always)]
    /// Short-hand for a zip and a fold
    fn zip_fold<A>(
        &self,
        other: &Self,
        start_value: A,
        mut f: impl FnMut(A, (&Self::Element, &Self::Element)) -> A,
    ) -> A {
        let mut accumulated = start_value;
        for (x, y) in self.iter().zip(other.iter()) {
            accumulated = f(accumulated, (x, y));
        }
        accumulated
    }

    #[inline(always)]
    fn iter<'a>(&'a self) -> ArrayIter<'a, Self> {
        ArrayIter {
            offset: 0,
            array: self,
        }
    }
}

// an array that generates itself
pub trait GenArray: Array + ArrayMut {
    fn generate(gen: impl Iterator<Item = Self::Element>) -> Self;

    #[inline(always)]
    fn component_wise(
        &self,
        other: &Self,
        mut f: impl Fn(&Self::Element, &Self::Element) -> Self::Element,
    ) -> Self {
        Self::generate(
            self.iter()
                .take(self.len())
                .zip(other.iter().take(other.len()))
                .map(|(x, y)| f(x, y)),
        )
    }

    #[inline(always)]
    fn map(&self, mut f: impl Fn(&Self::Element) -> Self::Element) -> Self {
        Self::generate(self.iter().map(|x| f(x)))
    }
}

#[auto_gen_impl(CloneArrayConstraint)]
pub trait CloneArray: GenArray<Element: Clone> + Clone + Sized {
    #[inline(always)]
    fn from_value(value: Self::Element, len: usize) -> Self {
        Self::generate(iter::repeat(value).take(len))
    }
}

/// An array is a thing that permits random access at integer offsets.
pub trait ArrayMut: Array {
    fn nth_mut(&mut self, n: usize) -> Option<&mut Self::Element>;

    fn iter_mut<'a>(&'a mut self) -> ArrayMutIter<'a, Self> {
        ArrayMutIter {
            offset: 0,
            array: self,
        }
    }

    fn for_each(&mut self, mut f: impl FnMut(&mut Self::Element)) {
        for i in 0..self.len().to_usize().unwrap() {
            f(self.nth_mut(i).unwrap());
        }
    }

    fn map_inplace(&mut self, mut f: impl FnMut(Self::Element) -> Self::Element) {
        self.iter_mut()
            .for_each(|x| replace_with_or_abort(x, |v| f(v)))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ArrayIter<'a, A: Array> {
    offset: usize,
    array: &'a A,
}

impl<'a, A: 'a + Array> Iterator for ArrayIter<'a, A> {
    type Item = &'a A::Element;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let ind = self.offset;
        self.offset += 1;
        self.array.nth(ind)
    }
}

pub struct ArrayMutIter<'a, A: Array> {
    offset: usize,
    array: &'a mut A,
}

impl<'a, A: 'a + ArrayMut> Iterator for ArrayMutIter<'a, A> {
    type Item = &'a mut A::Element;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let ind = self.offset;
        self.offset += 1;
        unsafe {
            // convince the borrow checker that this lifetime is ok
            std::mem::transmute(self.array.nth_mut(ind))
        }
    }
}

pub trait StaticLenArray {
    type Element;

    fn len() -> usize;
    fn generate(gen: impl Iterator<Item = Self::Element>) -> Self;
}

#[auto_gen_impl(OrdArrayConstraint)]
pub trait OrdArray: GenArray + Array<Element: PartialOrd + Copy> {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .zip(other.iter())
            .fold(true, |acc, (a, b)| acc && a == b)
    }

    fn min(&self, other: &Self) -> Self {
        Self::generate(
            self.iter()
                .zip(other.iter())
                .map(|(x, y)| if x.lt(y) { *x } else { *y }),
        )
    }

    fn max(&self, other: &Self) -> Self {
        Self::generate(
            self.iter()
                .zip(other.iter())
                .map(|(x, y)| if x.gt(y) { *x } else { *y }),
        )
    }
}

#[auto_gen_impl(ArraySubConstraint)]
pub trait ArraySub: Array<Element: RefSub> + GenArray {
    #[inline(always)]
    fn array_sub(&self, other: &Self) -> Self {
        self.component_wise(other, |x, y| x.sub(y))
    }
}

#[auto_gen_impl(ArrayAddConstraint)]
pub trait ArrayAdd: Array<Element: RefAdd> + GenArray {
    #[inline(always)]
    fn array_add(&self, other: &Self) -> Self {
        self.component_wise(other, |x, y| x.add(y))
    }

    // #[inline(always)]
    // fn offset(self, offset: Self::Element) -> Self::GenOut {
    //     self.map(|x| x.add(offset))
    // }
}

#[auto_gen_impl(ArrayMulConstraint)]
pub trait ArrayMul: Array<Element: RefMul> + GenArray {
    #[inline(always)]
    fn array_mul(&self, other: &Self) -> Self {
        self.component_wise(other, |x, y| x.mul(y))
    }

    // #[inline(always)]
    // fn scale(&self, offset: &Self::Element) -> Self::GenOut {
    //     self.map(|x| x * offset)
    // }
}

#[auto_gen_impl(ArrayDivConstraint)]
pub trait ArrayDiv: Array<Element: RefDiv> + GenArray {
    #[inline(always)]
    fn array_div(&self, other: &Self) -> Self {
        self.component_wise(other, |x, y| x.div(y))
    }
}

#[auto_gen_impl(ArrayPartialEqConstraint)]
pub trait ArrayPartialEq: Array<Element: PartialEq> {
    #[inline(always)]
    fn array_eq(&self, other: &Self) -> bool {
        self.zip_fold(other, true, |acc, (x, y)| acc && (x == y))
    }
}

#[auto_gen_impl(ArrayNegConstraint)]
pub trait ArrayNeg: Array<Element: RefNeg> + GenArray {
    #[inline(always)]
    fn array_neg(&self) -> Self {
        self.map(|x| x.neg())
    }
}

#[auto_gen_impl(ArrayInvConstraint)]
pub trait ArrayInv: Array<Element: RefInv> + GenArray {
    #[inline(always)]
    fn array_inv(&self) -> Self {
        self.map(|x| x.inv())
    }
}

impl<'a, A: Array> Array for &'a A {
    type Element = A::Element;

    #[inline(always)]
    fn nth(&self, n: usize) -> Option<&Self::Element> {
        (**self).nth(n)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        (**self).len()
    }

    // #[inline(always)]
    // fn generate(
    //     len: usize,
    //     gen: impl Iterator<Item = <Self::GenOut as Array>::Element>,
    // ) -> Self::GenOut {
    //     A::generate(len, gen)
    // }
}

// impl<'a, A: Array> Array for &'a mut A {
//     type GenOut = A::GenOut;
//     type Element = A::Element;

//     #[inline(always)]
//     fn nth(&self, n: usize) -> Option<&Self::Element> {
//         (**self).nth(n)
//     }

//     #[inline(always)]
//     fn len(&self) -> usize {
//         (**self).len()
//     }

//     // #[inline(always)]
//     // fn generate(
//     //     len: usize,
//     //     gen: impl Iterator<Item = <Self::GenOut as Array>::Element>,
//     // ) -> Self::GenOut {
//     //     A::generate(len, gen)
//     // }
// }

// impl<'a, A: ArrayMut> ArrayMut for &'a mut A {
//     #[inline(always)]
//     fn nth_mut(&mut self, n: usize) -> Option<&mut Self::Element> {
//         (*self).nth_mut(n)
//     }
// }

impl<A> PartialEq for Bound<A>
where
    A: ArrayPartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        <A as ArrayPartialEq>::array_eq(&self.0, &other.0)
    }
}

// impl<'a, A> crate::ops::Add for &'a A
// where
//     A: ArrayAdd,
// {
//     type Output = A::GenOut;

//     fn add(self, rhs: Self) -> Self::Output {
//         self.add(&rhs)
//     }
// }

// impl<'a, A> crate::ops::Sub for &'a A
// where
//     A: ArraySub,
// {
//     type Output = A::GenOut;

//     fn sub(self, rhs: Self) -> Self::Output {
//         self.sub(&rhs)
//     }
// }

// impl<'a, A> crate::ops::Neg for &'a A
// where
//     A: ArrayNeg,
// {
//     type Output = A::GenOut;

//     fn neg(self) -> Self::Output {
//         self.neg()
//     }
// }

// impl<'a, A> crate::ops::Mul for &'a A
// where
//     A: ArrayMul,
// {
//     type Output = A::GenOut;

//     fn mul(self, rhs: Self) -> Self::Output {
//         self.mul(&rhs)
//     }
// }

// impl<'a, A> crate::ops::Div for &'a A
// where
//     A: ArrayDiv,
// {
//     type Output = A::GenOut;

//     fn div(self, rhs: Self) -> Self::Output {
//         self.div(&rhs)
//     }
// }

impl<A> crate::ops::Add for A
where
    A: ArrayAdd,
{
    type Output = A;

    fn add(self, rhs: Self) -> Self::Output {
        self.array_add(&rhs)
    }
}

impl<A> crate::ops::Sub for A
where
    A: ArraySub,
{
    type Output = A;

    fn sub(self, rhs: Self) -> Self::Output {
        self.array_sub(&rhs)
    }
}

impl<A> crate::ops::Neg for A
where
    A: ArrayNeg,
{
    type Output = A;

    fn neg(self) -> Self::Output {
        self.array_neg()
    }
}

impl<A> crate::ops::Mul for A
where
    A: ArrayMul,
{
    type Output = A;

    fn mul(self, rhs: Self) -> Self::Output {
        self.array_mul(&rhs)
    }
}

impl<A> crate::ops::Div for A
where
    A: ArrayDiv,
{
    type Output = A;

    fn div(self, rhs: Self) -> Self::Output {
        self.array_div(&rhs)
    }
}

pub trait RefMath = RefAdd + RefSub + RefNeg + RefMul + RefDiv + RefInv;
pub trait ArrayMath = ArrayAdd + ArraySub + ArrayNeg + ArrayMul + ArrayDiv + ArrayInv;

/// Array of reals
#[auto_gen_impl(RealArrayConstraint)]
pub trait RealArray: Array<Element: Real + RefMath> + OrdArray + ArrayMath + CloneArray {
    fn repr(value: f64) -> Self::Element {
        Self::Element::repr(value)
    }
}
