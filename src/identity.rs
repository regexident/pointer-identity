use crate::Pointer;
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

/// Wrapper type that uses the pointer address rather than the value as the identity for the
/// [`PartialEq`], [`Eq`], [`PartialOrd`], [`Ord`] and [`Hash`] traits.
///
/// This wrapper is transparent thanks to implementing both [`Deref`] and [`DerefMut`], so you can
/// use an instance of this just like you would use an instance of the type you are wrapping.
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerIdentity<P: Pointer>(pub P);

impl<P: Pointer> PointerIdentity<P> {
    /// Create new from a pointer.
    pub fn new(value: P) -> Self {
        Self(value)
    }

    /// Get reference to inner value.
    pub fn inner(&self) -> &P {
        &self.0
    }

    /// Convert into inner value.
    pub fn into_inner(self) -> P {
        self.0
    }
}

impl<P: Pointer> From<P> for PointerIdentity<P> {
    fn from(value: P) -> Self {
        Self::new(value)
    }
}

impl<P: Pointer> PartialEq for PointerIdentity<P> {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<P: Pointer> Eq for PointerIdentity<P> {}

impl<P: Pointer> PartialOrd for PointerIdentity<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.get().cmp(&other.0.get()))
    }
}

impl<P: Pointer> Ord for PointerIdentity<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.get().cmp(&other.0.get())
    }
}

impl<P: Pointer> Hash for PointerIdentity<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.get().hash(state);
    }
}

impl<P: Pointer> Deref for PointerIdentity<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Pointer> DerefMut for PointerIdentity<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
