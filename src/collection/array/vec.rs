use {
    super::{Array, ArrayMut, GenArray, StaticLenArray},
    num_traits::ToPrimitive,
};
impl<T> Array for Vec<T> {
    type Element = T;

    #[inline(always)]
    fn nth(&self, n: usize) -> Option<&T> {
        self.get(n.to_usize()?)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.len()
    }

    // #[inline(always)]
    // fn generate(len: usize, mut gen: impl Iterator<Item = Self::Element>) -> Self {
    //     let len = len.to_usize().unwrap();
    //     let mut retv = Vec::with_capacity(len);
    //     for value in gen.take(len) {
    //         retv.push(value)
    //     }
    //     retv
    // }
}

impl<T> GenArray for Vec<T> {
    fn generate(gen: impl Iterator<Item = Self::Element>) -> Self {
        gen.collect()
    }
}

impl<T> ArrayMut for Vec<T> {
    #[inline(always)]
    fn nth_mut(&mut self, n: usize) -> Option<&mut T> {
        self.get_mut(n)
    }
}
