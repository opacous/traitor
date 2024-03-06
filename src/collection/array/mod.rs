use {
    crate::{
        algebra::{
            Inv, Natural, RefAddable, RefDivable, RefInvable, RefMulable, RefNegable, RefSubable,
        },
        analysis::{Metric, Real, RealExponential},
        Bound, B,
    },
    core::{
        mem::MaybeUninit,
        ops::{Add, Div, Mul, Sub},
    },
    num_traits::ToPrimitive,
    traitor_macros::auto_gen_impl,
};

mod metric;
pub use metric::*;

/// An array is a thing that permits random access at integer offsets.
pub trait Array: Sized {
    type Element;
    type GenOut: OwnedArray;

    fn nth(&self, n: usize) -> Option<&Self::Element>;
    fn len(&self) -> usize;
    fn generate<F: Fn(usize) -> Self::Element>(len: usize, gen: F) -> Self::GenOut;

    /// Auto Generated functions
    /// Applies `f` to each pair of components of `self` and `other`.
    #[inline(always)]
    fn component_wise(
        &self,
        other: &Self,
        mut f: impl Fn(&Self::Element, &Self::Element) -> Self::Element,
    ) -> Self::GenOut {
        Self::generate(self.len(), |i| {
            f(self.nth(i).unwrap(), other.nth(i).unwrap())
        })
    }

    #[inline(always)]
    fn map(&self, mut f: impl Fn(&Self::Element) -> Self::Element) -> Self::GenOut {
        Self::generate(self.len(), |i| f(self.nth(i).unwrap()))
    }

    #[inline(always)]
    fn fold<A>(&self, start_value: A, mut f: impl FnMut(A, &Self::Element) -> A) -> A {
        let mut accumulated = start_value;
        for i in 0..self.len().to_usize().unwrap() {
            accumulated = f(accumulated, self.nth(i).unwrap());
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
        for i in 0..self.len().to_usize().unwrap() {
            accumulated = f(accumulated, (self.nth(i).unwrap(), other.nth(i).unwrap()));
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
pub trait OwnedArray = Array<GenOut = Self> + ArrayMut;

#[auto_gen_impl(CloneArrayConstraint)]
pub trait CloneArray: Array<Element: Clone> + Sized {
    fn from_value(value: Self::Element, len: usize) -> Self::GenOut {
        Self::generate(len, |_| value.clone())
    }

    fn dup(&self) -> Self::GenOut {
        Self::generate(self.len(), |ind| self.nth(ind).unwrap().clone())
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
}

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

pub trait StaticLenArray<T> {
    fn len() -> usize;
    fn generate<F: Fn(usize) -> T>(gen: F) -> Self;
}

impl<T> Array for Vec<T> {
    type Element = T;
    type GenOut = Vec<T>;

    #[inline(always)]
    fn nth(&self, n: usize) -> Option<&T> {
        self.get(n.to_usize()?)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    #[inline(always)]
    fn generate<F: Fn(usize) -> T>(len: usize, gen: F) -> Self {
        let len = len.to_usize().unwrap();
        let mut retv = Vec::with_capacity(len);
        for ind in 0..len {
            retv.push(gen(ind))
        }
        retv
    }
}

impl<T> ArrayMut for Vec<T> {
    #[inline(always)]
    fn nth_mut(&mut self, n: usize) -> Option<&mut T> {
        self.get_mut(n)
    }
}

impl<T, const L: usize> Array for [T; L]
where
    [T; L]: Sized,
{
    type Element = T;
    type GenOut = [T; L];

    #[inline(always)]
    fn nth(&self, n: usize) -> Option<&T> {
        self.get(n)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        L
    }

    #[inline(always)]
    fn generate<F: Fn(usize) -> T>(_: usize, gen: F) -> Self {
        <Self as StaticLenArray<T>>::generate(gen)
    }
}

impl<T, const L: usize> ArrayMut for [T; L]
where
    [T; L]: Sized,
{
    #[inline(always)]
    fn nth_mut(&mut self, n: usize) -> Option<&mut T> {
        self.get_mut(n.to_usize()?)
    }
}

impl<T, const L: usize> StaticLenArray<T> for [T; L]
where
    [T; L]: Sized,
{
    #[inline(always)]
    fn len() -> usize {
        L
    }

    #[inline(always)]
    fn generate<F: Fn(usize) -> T>(gen: F) -> Self {
        let mut array: [T; L] = unsafe { MaybeUninit::uninit().assume_init() };

        for (i, element) in array.iter_mut().enumerate() {
            *element = gen(i);
        }

        array
    }
}

#[auto_gen_impl(OrdArrayConstraint)]
pub trait OrdArray: Array<Element: PartialOrd + Copy> {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .zip(other.iter())
            .fold(true, |acc, (a, b)| acc && a == b)
    }

    fn min(&self, other: &Self) -> Self::GenOut {
        Self::generate(self.len(), |ind| {
            let lhs = self.nth(ind).unwrap();
            let rhs = other.nth(ind).unwrap();

            if lhs.lt(rhs) {
                *lhs
            } else {
                *rhs
            }
        })
    }

    fn max(&self, other: &Self) -> Self::GenOut {
        Self::generate(self.len(), |ind| {
            let lhs = self.nth(ind).unwrap();
            let rhs = other.nth(ind).unwrap();

            if lhs.gt(rhs) {
                *lhs
            } else {
                *rhs
            }
        })
    }
}

#[auto_gen_impl(ArraySubConstraint)]
pub trait ArraySub: Array<Element: RefSubable> {
    #[inline(always)]
    fn sub(self, other: &Self) -> Self::GenOut {
        self.component_wise(other, |x, y| x - y)
    }
}

#[auto_gen_impl(ArrayAddConstraint)]
pub trait ArrayAdd: Array<Element: RefAddable> {
    #[inline(always)]
    fn add(self, other: &Self) -> Self::GenOut {
        self.component_wise(other, |x, y| x + y)
    }
}

#[auto_gen_impl(ArrayMulConstraint)]
pub trait ArrayMul: Array<Element: RefMulable> {
    #[inline(always)]
    fn mul(self, other: &Self) -> Self::GenOut {
        self.component_wise(other, |x, y| x * y)
    }
}

#[auto_gen_impl(ArrayDivConstraint)]
pub trait ArrayDiv: Array<Element: RefDivable> {
    #[inline(always)]
    fn div(self, other: &Self) -> Self::GenOut {
        self.component_wise(other, |x, y| x / y)
    }
}

#[auto_gen_impl(ArrayNegConstraint)]
pub trait ArrayNeg: Array<Element: RefNegable> {
    #[inline(always)]
    fn neg(self) -> Self::GenOut {
        self.map(|x| -x)
    }
}

// pub trait ArrayInvConstraint = Array<Element: RefInvable>;
// pub trait ArrayInv: ArrayInvConstraint {
//     #[inline(always)]
//     fn inv(self) -> Self::GenOut {
//         self.map(|x| x.inv())
//     }
// }
// impl<A: ArrayInvConstraint> ArrayInv for A {}

// A#[macro_export]
// macro_rules! auto_trait {
//     ($trait_name:ident : $constraint_name:ident = $constaints:stmt) => {
//         pub trait $constraint_name = $constraints;
//         impl<A: $constaint_name> $trait_name for A {}
//     };
// }
//
// auto_trait!(ArrayInv : ArrayInvConstraint = Array<Element: RefInvable>);
#[auto_gen_impl(ArrayInvConstraint)]
pub trait ArrayInv: Array<Element: RefInvable> {
    #[inline(always)]
    fn inv(self) -> Self::GenOut {
        self.map(|x| x.inv())
    }
}
impl<A: Array> Array for Bound<A> {
    type Element = A::Element;
    type GenOut = Bound<A::GenOut>;
    #[inline(always)]
    fn nth(&self, n: usize) -> Option<&A::Element> {
        self.0.nth(n)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn generate<F: Fn(usize) -> A::Element>(len: usize, gen: F) -> Self::GenOut {
        A::generate(len, gen).b()
    }
}

impl<A: ArrayMut> ArrayMut for Bound<A> {
    #[inline(always)]
    fn nth_mut(&mut self, n: usize) -> Option<&mut A::Element> {
        self.0.nth_mut(n)
    }
}

impl<'a, A: Array> Array for &'a A {
    type GenOut = A::GenOut;
    type Element = A::Element;

    #[inline(always)]
    fn nth(&self, n: usize) -> Option<&Self::Element> {
        (**self).nth(n)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline(always)]
    fn generate<F: Fn(usize) -> Self::Element>(len: usize, gen: F) -> Self::GenOut {
        A::generate(len, gen)
    }
}

impl<'a, A: Array> Array for &'a mut A {
    type GenOut = A::GenOut;
    type Element = A::Element;

    #[inline(always)]
    fn nth(&self, n: usize) -> Option<&Self::Element> {
        (**self).nth(n)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        (**self).len()
    }

    #[inline(always)]
    fn generate<F: Fn(usize) -> Self::Element>(len: usize, gen: F) -> Self::GenOut {
        A::generate(len, gen)
    }
}

impl<'a, A: ArrayMut> ArrayMut for &'a mut A {
    #[inline(always)]
    fn nth_mut(&mut self, n: usize) -> Option<&mut Self::Element> {
        (*self).nth_mut(n)
    }
}

impl<A> std::ops::Add for Bound<A>
where
    A: ArrayAdd,
{
    type Output = A::GenOut;

    fn add(self, rhs: Self) -> Self::Output {
        self.0.add(&rhs.0)
    }
}

impl<A> std::ops::Sub for Bound<A>
where
    A: ArraySub,
{
    type Output = A::GenOut;

    fn sub(self, rhs: Self) -> Self::Output {
        self.0.sub(&rhs.0)
    }
}

impl<A> std::ops::Neg for Bound<A>
where
    A: ArrayNeg,
{
    type Output = A::GenOut;

    fn neg(self) -> Self::Output {
        self.0.neg()
    }
}

impl<A> std::ops::Mul for Bound<A>
where
    A: ArrayMul,
{
    type Output = A::GenOut;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0.mul(&rhs.0)
    }
}

impl<A> std::ops::Div for Bound<A>
where
    A: ArrayDiv,
{
    type Output = A::GenOut;

    fn div(self, rhs: Self) -> Self::Output {
        self.0.div(&rhs.0)
    }
}

impl<A> Inv for Bound<A>
where
    A: ArrayInv,
{
    type Output = A::GenOut;

    fn inv(self) -> Self::Output {
        self.0.inv()
    }
}

pub trait RefMath = RefAddable + RefSubable + RefNegable + RefMulable + RefDivable + RefInvable;
pub trait ArrayMath = ArrayAdd + ArraySub + ArrayNeg + ArrayMul + ArrayDiv + ArrayInv;

/// Array of reals
#[auto_gen_impl(RealArrayConstraint)]
pub trait RealArray:
    Array<Element: Real, GenOut: Array<Element: Real> + CloneArray> + OrdArray + ArrayMath + CloneArray
{
    fn repr(value: f64) -> <Self::GenOut as Array>::Element {
        <Self::GenOut as Array>::Element::repr(value)
    }
}
