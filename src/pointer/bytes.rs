use super::Pointer;
use ::bytes::Bytes;

#[cfg(feature = "bytes")]
impl Pointer for Bytes {
    type Target = u8;

    fn get(&self) -> *const Self::Target {
        self.as_ptr()
    }
}
