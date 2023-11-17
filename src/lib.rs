//! # Pointer Identity
//!
//! Rust has some traits that operate on values:
//!
//! - [`Ord`] and [`PartialOrd`] check the ordering of values,
//! - [`Eq`] and [`PartialEq`] check equality of values,
//! - [`Hash`] computes a hash sum of values.
//!
//! When using smart pointers such as [`Arc`], [`Rc`] and [`Box`] in Rust, these will forward the
//! implementation to the underlying types. However, in some cases this might not be what you want.
//! In some cases, it is good enough to check the identity of the pointer rather than that of the
//! value.

mod identity;
mod pointer;

pub use crate::{identity::PointerIdentity, pointer::Pointer};
