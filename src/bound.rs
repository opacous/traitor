use {
    crate::{
        collection::{Array, ArrayMut},
        ops::*,
    },
    core::ops::{Deref, DerefMut},
};

// gives a somewhat more ergonomic way to provide blanket implementations
// this way, a user can provide a single trait impl for their type, then
// by calling UserType.b() they get access to all functionality that can be derived
// based upon that trait
pub trait B: Sized {
    #[inline(always)]
    fn b(self) -> Bound<Self> {
        self.into()
    }
}

impl<T> B for T {}

#[repr(transparent)]
#[derive(Debug, Copy, Clone)]
pub struct Bound<T>(T);

impl<T> Bound<T> {
    #[inline(always)]
    pub fn u(self) -> T {
        self.0
    }

    #[inline(always)]
    pub fn ur(&self) -> &T {
        &self.0
    }
}

impl<T> From<T> for Bound<T> {
    #[inline(always)]
    fn from(t: T) -> Self {
        Bound(t)
    }
}

impl<'a, T> From<&'a T> for &'a Bound<T> {
    #[inline(always)]
    fn from(t: &'a T) -> Self {
        unsafe { std::mem::transmute(t) }
    }
}

impl<'a, T> From<&'a mut T> for &'a mut Bound<T> {
    #[inline(always)]
    fn from(t: &'a mut T) -> Self {
        unsafe { std::mem::transmute(t) }
    }
}

impl<T> Deref for Bound<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<T> DerefMut for Bound<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

impl<A: Add<A>> std::ops::Add<Bound<A>> for Bound<A> {
    type Output = Bound<<A as Add<A>>::Output>;

    fn add(self, rhs: Bound<A>) -> Self::Output {
        self.0.add(rhs.0).b()
    }
}

impl<A: Sub<A>> std::ops::Sub<Bound<A>> for Bound<A> {
    type Output = Bound<<A as Sub<A>>::Output>;

    fn sub(self, rhs: Bound<A>) -> Self::Output {
        self.0.sub(rhs.0).b()
    }
}

impl<A: Mul<A>> std::ops::Mul<Bound<A>> for Bound<A> {
    type Output = Bound<<A as Mul<A>>::Output>;

    fn mul(self, rhs: Bound<A>) -> Self::Output {
        self.0.mul(rhs.0).b()
    }
}

impl<A: Div<A>> std::ops::Div<Bound<A>> for Bound<A> {
    type Output = Bound<<A as Div<A>>::Output>;

    fn div(self, rhs: Bound<A>) -> Self::Output {
        self.0.div(rhs.0).b()
    }
}

impl<'a, A> std::ops::Add<&'a Bound<A>> for &'a Bound<A>
where
    for<'b> &'b A: Add<&'b A>,
{
    type Output = Bound<<&'a A as Add<&'a A>>::Output>;

    fn add(self, rhs: &'a Bound<A>) -> Self::Output {
        self.0.add(&rhs.0).b()
    }
}

impl<'a, A> std::ops::Sub<&'a Bound<A>> for &'a Bound<A>
where
    for<'b> &'b A: Sub<&'b A>,
{
    type Output = Bound<<&'a A as Sub<&'a A>>::Output>;

    fn sub(self, rhs: &'a Bound<A>) -> Self::Output {
        self.0.sub(&rhs.0).b()
    }
}

impl<'a, A> std::ops::Mul<&'a Bound<A>> for &'a Bound<A>
where
    for<'b> &'b A: Mul<&'b A>,
{
    type Output = Bound<<&'a A as Mul<&'a A>>::Output>;

    fn mul(self, rhs: &'a Bound<A>) -> Self::Output {
        self.0.mul(&rhs.0).b()
    }
}

impl<'a, A> std::ops::Div<&'a Bound<A>> for &'a Bound<A>
where
    for<'b> &'b A: Div<&'b A>,
{
    type Output = Bound<<&'a A as Div<&'a A>>::Output>;

    fn div(self, rhs: &'a Bound<A>) -> Self::Output {
        self.0.div(&rhs.0).b()
    }
}

impl<A: AddAssign<A>> std::ops::AddAssign<Bound<A>> for Bound<A> {
    fn add_assign(&mut self, rhs: Bound<A>) {
        self.0.add_assign(rhs.0)
    }
}

impl<A: SubAssign<A>> std::ops::SubAssign<Bound<A>> for Bound<A> {
    fn sub_assign(&mut self, rhs: Bound<A>) {
        self.0.sub_assign(rhs.0)
    }
}

impl<A: MulAssign<A>> std::ops::MulAssign<Bound<A>> for Bound<A> {
    fn mul_assign(&mut self, rhs: Bound<A>) {
        self.0.mul_assign(rhs.0)
    }
}

impl<A: DivAssign<A>> std::ops::DivAssign<Bound<A>> for Bound<A> {
    fn div_assign(&mut self, rhs: Bound<A>) {
        self.0.div_assign(rhs.0)
    }
}

impl<A: Array> std::ops::Index<usize> for Bound<A> {
    type Output = Bound<A::Element>;

    fn index(&self, index: usize) -> &Self::Output {
        self.nth(index).unwrap().into()
    }
}

impl<A: ArrayMut> std::ops::IndexMut<usize> for Bound<A> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.nth_mut(index).unwrap().into()
    }
}
