use {
    super::{Array, ArrayMut, GenArray, StaticLenArray},
    num_traits::ToPrimitive,
    smallvec::SmallVec,
};

impl<T, const LEN: usize> Array for SmallVec<[T; LEN]> {
    type Element = T;

    #[inline(always)]
    fn nth(&self, n: usize) -> Option<&T> {
        self.get(n.to_usize()?)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }
}

impl<T, const LEN: usize> GenArray for SmallVec<[T; LEN]> {
    fn generate(gen: impl Iterator<Item = Self::Element>) -> Self {
        gen.collect()
    }
}

impl<T, const LEN: usize> ArrayMut for SmallVec<[T; LEN]> {
    #[inline(always)]
    fn nth_mut(&mut self, n: usize) -> Option<&mut T> {
        self.get_mut(n)
    }
}
