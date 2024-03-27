use {
    super::{Array, ArrayMut, OwnedArray},
    crate::Bound,
};

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
    fn generate(
        len: usize,
        gen: impl Iterator<Item = <Self::GenOut as Array>::Element>,
    ) -> Self::GenOut {
        A::generate(len, gen).b()
    }
}

impl<A: ArrayMut> ArrayMut for Bound<A> {
    #[inline(always)]
    fn nth_mut(&mut self, n: usize) -> Option<&mut A::Element> {
        self.0.nth_mut(n)
    }
}
