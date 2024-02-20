use core::{mem::MaybeUninit, ptr};

use crate::algebra::Natural;

/// An array is a thing that permits random access at integer offsets.
pub trait Array<T> {
    fn nth(&self, n: impl Natural) -> Option<&T>;
    fn nth_mut(&mut self, n: impl Natural) -> Option<&mut T>;
    fn len(&self) -> impl Natural;
    fn generate<N: Natural, F: Fn(N) -> T>(len: N, gen: F) -> Self;
}

pub trait StaticLenArray<T> {
    fn len() -> impl Natural;
    fn generate<N: Natural, F: Fn(N) -> T>(gen: F) -> Self;
}

impl<T> Array<T> for Vec<T> {
    fn nth(&self, n: impl Natural) -> Option<&T> {
        self.get(n.to_usize()?)
    }

    fn nth_mut(&mut self, n: impl Natural) -> Option<&mut T> {
        self.get_mut(n.to_usize()?)
    }

    fn len(&self) -> impl Natural {
        self.len()
    }

    fn generate<N: Natural, F: Fn(N) -> T>(len: N, gen: F) -> Self {
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
    fn nth(&self, n: impl Natural) -> Option<&T> {
        self.get(n.to_usize()?)
    }

    fn nth_mut(&mut self, n: impl Natural) -> Option<&mut T> {
        self.get_mut(n.to_usize()?)
    }

    fn len(&self) -> impl Natural {
        L
    }

    fn generate<N: Natural, F: Fn(N) -> T>(_: N, gen: F) -> Self {
        <Self as StaticLenArray<T>>::generate(gen)
    }
}

impl<T, const L: usize> StaticLenArray<T> for [T; L]
where
    [T; L]: Sized,
{
    fn len() -> impl Natural {
        L
    }

    fn generate<N: Natural, F: Fn(N) -> T>(gen: F) -> Self {
        let mut array: [T; L] = unsafe { MaybeUninit::uninit().assume_init() };

        for (i, element) in array.iter_mut().enumerate() {
            *element = gen(N::from_usize(i).unwrap());
        }

        array
    }
}
