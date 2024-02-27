use {
    crate::{algebra::Natural, analysis::Real},
    core::{marker::PhantomData, mem::MaybeUninit, ptr},
    num_traits::ToPrimitive,
};

pub trait ArrayIndex: Natural + Copy {}

impl ArrayIndex for usize {}

/// An array is a thing that permits random access at integer offsets.
pub trait Array<T>: Sized {
    fn nth(&self, n: impl ArrayIndex) -> Option<&T>;
    fn nth_mut(&mut self, n: impl ArrayIndex) -> Option<&mut T>;
    fn len(&self) -> impl ArrayIndex;
    fn generate<N: ArrayIndex, F: Fn(N) -> T>(len: N, gen: F) -> Self;

    /// Auto Generated functions
    /// Applies `f` to each pair of components of `self` and `other`.
    fn component_wise(&self, other: &Self, mut f: impl Fn(&T, &T) -> T) -> Self {
        Self::generate(self.len(), |i| {
            f(self.nth(i).unwrap(), other.nth(i).unwrap())
        })
    }

    fn fold<A>(&self, start_value: A, mut f: impl FnMut(A, &T) -> A) -> A {
        let mut accumulated = start_value;
        for i in 0..self.len().to_usize().unwrap() {
            accumulated = f(accumulated, self.nth(i).unwrap());
        }
        accumulated
    }

    fn for_each<A>(&mut self, mut f: impl FnMut(&mut T)) {
        for i in 0..self.len().to_usize().unwrap() {
            f(self.nth_mut(i).unwrap());
        }
    }

    /// Short-hand for a zip and a fold
    fn zip_fold<A>(&self, other: &Self, start_value: A, mut f: impl FnMut(A, (&T, &T)) -> A) -> A {
        let mut accumulated = start_value;
        for i in 0..self.len().to_usize().unwrap() {
            accumulated = f(accumulated, (self.nth(i).unwrap(), other.nth(i).unwrap()));
        }
        accumulated
    }

    fn iter<'a>(&'a self) -> ArrayIter<'a, T, Self> {
        ArrayIter {
            offset: 0,
            array: self,
            _phantom: Default::default(),
        }
    }

    fn iter_mut<'a>(&'a mut self) -> ArrayMutIter<'a, T, Self> {
        ArrayMutIter {
            offset: 0,
            array: self,
            _phantom: Default::default(),
        }
    }
}

pub struct ArrayIter<'a, T, A: Array<T>> {
    offset: usize,
    array: &'a A,
    _phantom: PhantomData<T>,
}

impl<'a, T: 'a, A: Array<T>> Iterator for ArrayIter<'a, T, A> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let ind = self.offset;
        self.offset += 1;
        self.array.nth(ind)
    }
}

pub struct ArrayMutIter<'a, T, A: Array<T>> {
    offset: usize,
    array: &'a mut A,
    _phantom: PhantomData<T>,
}

impl<'a, T: 'a, A: 'a + Array<T>> Iterator for ArrayMutIter<'a, T, A> {
    type Item = &'a mut T;

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
    fn len() -> impl ArrayIndex;
    fn generate<N: ArrayIndex, F: Fn(N) -> T>(gen: F) -> Self;
}

impl<T> Array<T> for Vec<T> {
    fn nth(&self, n: impl ArrayIndex) -> Option<&T> {
        self.get(n.to_usize()?)
    }

    fn nth_mut(&mut self, n: impl ArrayIndex) -> Option<&mut T> {
        self.get_mut(n.to_usize()?)
    }

    fn len(&self) -> impl ArrayIndex {
        self.len()
    }

    fn generate<N: ArrayIndex, F: Fn(N) -> T>(len: N, gen: F) -> Self {
        let len = len.to_usize().unwrap();
        let mut retv = Vec::with_capacity(len);
        for ind in 0..len {
            retv.push(gen(N::from_usize(ind).unwrap()))
        }
        retv
    }
}

impl<T, const L: usize> Array<T> for [T; L]
where
    [T; L]: Sized,
{
    fn nth(&self, n: impl ArrayIndex) -> Option<&T> {
        self.get(n.to_usize()?)
    }

    fn nth_mut(&mut self, n: impl ArrayIndex) -> Option<&mut T> {
        self.get_mut(n.to_usize()?)
    }

    fn len(&self) -> impl ArrayIndex {
        L
    }

    fn generate<N: ArrayIndex, F: Fn(N) -> T>(_: N, gen: F) -> Self {
        <Self as StaticLenArray<T>>::generate(gen)
    }
}

impl<T, const L: usize> StaticLenArray<T> for [T; L]
where
    [T; L]: Sized,
{
    fn len() -> impl ArrayIndex {
        L
    }

    fn generate<N: ArrayIndex, F: Fn(N) -> T>(gen: F) -> Self {
        let mut array: [T; L] = unsafe { MaybeUninit::uninit().assume_init() };

        for (i, element) in array.iter_mut().enumerate() {
            *element = gen(N::from_usize(i).unwrap());
        }

        array
    }
}

pub trait OrdArray<T: PartialOrd + Copy>: Array<T> {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .zip(other.iter())
            .fold(true, |acc, (a, b)| acc && a == b)
    }

    fn min(&self, other: &Self) -> Self {
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

    fn max(&self, other: &Self) -> Self {
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

/// Array of reals
pub trait RealArray<T: Real>: Array<T> {}

impl<T: Real + PartialOrd + Copy, R: RealArray<T>> OrdArray<T> for R {}

#[derive(Default)]
pub struct EuclideanMetric {}

impl<T: Real, X: RealArray<T>> crate::analysis::Metric<X, T> for EuclideanMetric {
    fn distance(&self, x1: X, x2: X) -> T {
        x1.zip_fold(&x2, T::repr(0.0), |a, (x, y)| a + (x - y).pow(T::repr(2.0)))
            .sqrt()
    }
}
