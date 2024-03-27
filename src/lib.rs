#![feature(trait_alias)]
#![feature(specialization)]
#![feature(extra_log_consts)]
#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]
#![cfg_attr(not(feature = "std"), no_std)]

use {core::ops::DerefMut, std::ops::Deref, traitor_macros::traitor_ops};

pub mod algebra;
pub mod analysis;
pub mod collection;
pub mod linear;
pub mod ops;

pub trait IntoClass<Destination> {
    fn into(self) -> Destination;
}

pub trait FromClass<Origin> {
    fn from(origin: Origin) -> Self;
}

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

extern crate self as traitor;

pub mod prefix {
    pub use {
        super::{algebra::*, analysis::*, collection::*, *},
        traitor_macros::traitor_ops,
    };
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
