use {
    super::{Array, ArrayMut, GenArray, StaticLenArray},
    core::mem::MaybeUninit,
    num_traits::ToPrimitive,
};

impl<T, const L: usize> Array for [T; L]
where
    [T; L]: Sized,
{
    type Element = T;

    #[inline(always)]
    fn nth<'a>(&'a self, n: usize) -> Option<&'a T> {
        self.get(n)
    }

    #[inline(always)]
    fn len(&self) -> usize {
        L
    }
}

impl<T: 'static, const L: usize> GenArray for [T; L]
where
    [T; L]: Sized,
{
    #[inline(always)]
    fn generate(gen: impl Iterator<Item = T>) -> Self {
        <Self as StaticLenArray>::generate(gen)
    }
}

// impl<'a, T: 'static, const L: usize> Array for [&'a T; L]
// where
//     [&'a T; L]: Sized,
// {
//     type Element = &'a T;
//     type GenOut = [T; L];

//     #[inline(always)]
//     fn nth(&self, n: usize) -> Option<&T> {
//         self.get(n)
//     }

//     #[inline(always)]
//     fn len(&self) -> usize {
//         L
//     }

//     // #[inline(always)]
//     // fn generate(
//     //     _len: usize,
//     //     gen: impl Iterator<Item = <Self::GenOut as Array>::Element>,
//     // ) -> Self::GenOut {
//     //     <Self as StaticLenArray<T>>::generate(gen)
//     // }
// }

impl<T: 'static, const L: usize> ArrayMut for [T; L]
where
    [T; L]: Sized,
{
    #[inline(always)]
    fn nth_mut(&mut self, n: usize) -> Option<&mut T> {
        self.get_mut(n.to_usize()?)
    }
}

impl<T, const L: usize> StaticLenArray for [T; L]
where
    [T; L]: Sized,
{
    type Element = T;

    #[inline(always)]
    fn len() -> usize {
        L
    }

    #[inline(always)]
    fn generate(gen: impl Iterator<Item = T>) -> Self {
        let mut array: [T; L] = unsafe { MaybeUninit::uninit().assume_init() };

        for (i, element) in gen.take(L).enumerate() {
            array[i] = element;
        }

        array
    }
}
