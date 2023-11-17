use super::Pointer;
use std::{rc::Rc, sync::Arc};

impl<T> Pointer for Arc<T> {
    type Target = T;

    fn get(&self) -> *const Self::Target {
        Arc::as_ptr(self)
    }
}

impl<T> Pointer for Rc<T> {
    type Target = T;

    fn get(&self) -> *const Self::Target {
        Rc::as_ptr(self)
    }
}

impl<T> Pointer for Box<T> {
    type Target = T;

    fn get(&self) -> *const Self::Target {
        &**self as *const Self::Target
    }
}

impl<T> Pointer for Vec<T> {
    type Target = T;

    fn get(&self) -> *const Self::Target {
        self.as_ptr()
    }
}

impl<T> Pointer for &[T] {
    type Target = T;

    fn get(&self) -> *const Self::Target {
        self.as_ptr()
    }
}

impl<const LEN: usize, T> Pointer for &[T; LEN] {
    type Target = T;

    fn get(&self) -> *const Self::Target {
        self.as_ptr()
    }
}
