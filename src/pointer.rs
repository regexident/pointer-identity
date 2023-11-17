mod builtin;
#[cfg(feature = "bytes")]
mod bytes;

// TODO: impl for anything that has a as_ptr()?

/// Type that stores it's value in an allocation and can retrieve a pointer to the value.
pub trait Pointer {
    type Target;

    /// Get a read-only pointer to the value.
    fn get(&self) -> *const Self::Target;
}
