#![feature(trait_alias)]
#![feature(specialization)]
#![feature(extra_log_consts)]
#![feature(associated_type_bounds)]
#![feature(associated_type_defaults)]
#![cfg_attr(not(feature = "std"), no_std)]

use {core::ops::DerefMut, std::ops::Deref, traitor_macros::traitor_ops};

pub mod algebra;
pub mod analysis;
pub mod bound;
pub mod collection;
pub mod linear;
pub mod ops;

pub trait IntoClass<Destination> {
    fn into(self) -> Destination;
}

pub trait FromClass<Origin> {
    fn from(origin: Origin) -> Self;
}

extern crate self as traitor;

pub mod prefix {
    pub use {
        super::{algebra::*, analysis::*, bound::*, collection::*, *},
        traitor_macros::traitor_ops,
    };
}
