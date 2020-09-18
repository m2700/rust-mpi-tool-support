#[repr(transparent)]
pub struct UnsafeBox<T>(T);
impl<T> UnsafeBox<T> {
    #[inline]
    pub fn new(t: T) -> Self {
        Self(t)
    }
    #[inline]
    pub unsafe fn unwrap(self) -> T {
        self.0
    }
}
