use std::{
    ops::{Deref, DerefMut},
    cmp::Ordering,
    hash::{Hash, Hasher},
};
use crate::Pointer;

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PointerIdentity<P: Pointer> {
    value: P,
}

impl<P: Pointer> PointerIdentity<P> {
    pub fn new(value: P) -> Self {
        Self { value }
    }

    pub fn inner(&self) -> &P {
        &self.value
    }

    pub fn into_inner(self) -> P {
        self.value
    }
}

impl<P: Pointer> PartialEq for PointerIdentity<P> {
    fn eq(&self, other: &Self) -> bool {
        self.value.get() == other.value.get()
    }
}

impl<P: Pointer> Eq for PointerIdentity<P> {}

impl<P: Pointer> PartialOrd for PointerIdentity<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.value.get().cmp(&other.value.get()))
    }
}

impl<P: Pointer> Ord for PointerIdentity<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.get().cmp(&other.value.get())
    }
}

impl<P: Pointer> Hash for PointerIdentity<P> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.get().hash(state);
    }
}

impl<P: Pointer> Deref for PointerIdentity<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<P: Pointer> DerefMut for PointerIdentity<P> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
