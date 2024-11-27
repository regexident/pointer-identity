use triomphe::Arc;

use super::Pointer;

impl<T: ?Sized> Pointer for Arc<T> {
    type Target = T;

    fn get(&self) -> *const Self::Target {
        Self::as_ptr(self)
    }
}
