/// Interface for supported buffer types.
/// 
/// The internal buffer of all uniboxes must implement this trait.
pub unsafe trait Buffer {
    /// Init the type.
    fn init() -> Self;
    /// Raw pointer to type.
    fn ptr<T>(&self) -> *const T;
    /// Copy from byte array to type *len* bytes.
    fn copy_from_byte(&mut self, src: &[u8], len: usize);
    /// Copy from type to type *len* bytes.
    fn copy_from_type(&mut self, src: &Self, len: usize);
}

unsafe impl Buffer for [u8; 32] {
    fn init() -> Self {
        [0; 32]
    }

    fn ptr<T>(&self) -> *const T {
        self.as_ptr() as *const T
    }

    fn copy_from_byte(&mut self, src: &[u8], len: usize) {
        self[0..len].clone_from_slice(src);
    }

    fn copy_from_type(&mut self, src: &Self, len: usize) {
        self[0..len].clone_from_slice(&src[0..len]);
    }
}

unsafe impl Buffer for [u8; 64] {
    fn init() -> Self {
        [0; 64]
    }

    fn ptr<T>(&self) -> *const T {
        self.as_ptr() as *const T
    }

    fn copy_from_byte(&mut self, src: &[u8], len: usize) {
        self[0..len].clone_from_slice(src);
    }

    fn copy_from_type(&mut self, src: &Self, len: usize) {
        self[0..len].clone_from_slice(&src[0..len]);
    }
}

unsafe impl Buffer for [u8; 128] {
    fn init() -> Self {
        [0; 128]
    }

    fn ptr<T>(&self) -> *const T {
        self.as_ptr() as *const T
    }

    fn copy_from_byte(&mut self, src: &[u8], len: usize) {
        self[0..len].clone_from_slice(src);
    }

    fn copy_from_type(&mut self, src: &Self, len: usize) {
        self[0..len].clone_from_slice(&src[0..len]);
    }
}

unsafe impl Buffer for [u8; 256] {
    fn init() -> Self {
        [0; 256]
    }

    fn ptr<T>(&self) -> *const T {
        self.as_ptr() as *const T
    }

    fn copy_from_byte(&mut self, src: &[u8], len: usize) {
        self[0..len].clone_from_slice(src);
    }

    fn copy_from_type(&mut self, src: &Self, len: usize) {
        self[0..len].clone_from_slice(&src[0..len]);
    }
}
