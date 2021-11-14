use core::{
    slice,
    mem,
    ptr,
    ops::Drop
};
use super::Uniboxed;

/// Interface for supported buffer types.
/// 
/// The internal buffer of all uniboxes must implement this trait.
pub trait Buffer {
    /// Init the type.
    fn init() -> Self;
    /// Raw pointer to type.
    fn ptr<T>(&self) -> *const T;
    /// Copy from byte array to type *len* bytes.
    fn copy_from_byte(&mut self, src: &[u8], len: usize);
    /// Copy from type to type *len* bytes.
    fn copy_from_type(&mut self, src: &Self, len: usize);
}

impl Buffer for [u8; 32] {
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

impl Buffer for [u8; 64] {
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

impl Buffer for [u8; 128] {
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

impl Buffer for [u8; 256] {
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

/// Generic static unibox that can implement any [`Buffer`].
/// 
/// This is the base of other static types, and should not be used directly. Use it only to implement your custom static unibox type.
pub struct UniBoxN<B: Buffer> {
    data: B,
    len: usize,
    alig: usize,
    autodrop: fn(&Self),
    id: usize
}

impl<B: Buffer> UniBoxN<B> {
    /// Create a new UniBox instance.
    /// 
    /// Accepts an *instance* and an *id*: a custom defined identifier used to know what type lies inside.
    /// 
    /// Returns Err if the struct is bigger than N bytes (N being the size of the unibox).
    pub fn new<T: Sized>(instance: T, id: usize) -> Result<Self, ()> {
        let bytes = unsafe {
            slice::from_raw_parts(
                (&instance as *const T) as *const u8,
                mem::size_of::<T>()
            )
        };
        let autodrop = |_self: &Self| {
            mem::drop(unsafe { _self.as_owned::<T>() });
        };
        let len = bytes.len();
        if len > mem::size_of::<B>() {
            Err(())
        }
        else {
            let mut data = B::init();
            data.copy_from_byte(bytes, len);
            mem::forget(instance);
            Ok(
                Self {
                    data,
                    len,
                    alig: mem::align_of::<T>(),
                    autodrop,
                    id
                }
            )
        }
    }

    /// Get reference to stored data using a type.
    /// 
    /// **WARNING**: If you try to cast a type other than the one actually hosted, you may get a panic or any undefined behavior.
    pub unsafe fn as_ref<T: Sized>(&self) -> &T {
        let len = mem::size_of::<T>();
        let alig = mem::align_of::<T>();
        // Integrity checks
        if len != self.len || alig != self.alig {
            panic!("Size or align of hosted and requiered types are different");
        }
        mem::transmute::<&B, &T>(&self.data)
    }

    /// Get mutable reference to stored data using a type.
    /// 
    /// **WARNING**: If you try to cast a type other than the one actually hosted, you may get a panic or any undefined behavior.
    pub unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T {
        let len = mem::size_of::<T>();
        let alig = mem::align_of::<T>();
        // Integrity checks
        if len != self.len || alig != self.alig {
            panic!("Size or align of hosted and requiered types are different");
        }
        mem::transmute::<&mut B, &mut T>(&mut self.data)
    }

    /// Stored data length.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Type identifier.
    pub fn id(&self) -> usize {
        self.id
    }

    unsafe fn as_owned<T: Sized>(&self) -> T {
        ptr::read(self.data.ptr() as *const T)
    }
}

impl<S: Buffer> Drop for UniBoxN<S> {
    fn drop(&mut self) {
        (self.autodrop)(self);
    }
}

/// Store a type on stack with a max size of 32 bytes.
pub struct UniBox32 {
    unibox: UniBoxN<[u8; 32]>
}

impl Uniboxed for UniBox32 {
    fn new_with_id<T: Sized>(instance: T, id: usize) -> Result<Self, ()> where Self: Sized {
        Ok(
            Self {
                unibox: UniBoxN::new(instance, id)?
            }
        )
    }

    unsafe fn as_ref<T: Sized>(&self) -> &T {
        self.unibox.as_ref()
    }

    unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T {
        self.unibox.as_mut_ref()
    }

    fn len(&self) -> usize {
        self.unibox.len()
    }

    fn id(&self) -> usize {
        self.unibox.id()
    }
}

/// Store a type on stack with a max size of 64 bytes.
pub struct UniBox64 {
    unibox: UniBoxN<[u8; 64]>
}

impl Uniboxed for UniBox64 {
    fn new_with_id<T: Sized>(instance: T, id: usize) -> Result<Self, ()> where Self: Sized {
        Ok(
            Self {
                unibox: UniBoxN::new(instance, id)?
            }
        )
    }

    unsafe fn as_ref<T: Sized>(&self) -> &T {
        self.unibox.as_ref()
    }

    unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T {
        self.unibox.as_mut_ref()
    }

    fn len(&self) -> usize {
        self.unibox.len()
    }

    fn id(&self) -> usize {
        self.unibox.id()
    }
}

/// Store a type on stack with a max size of 128 bytes.
pub struct UniBox128 {
    unibox: UniBoxN<[u8; 128]>
}

impl Uniboxed for UniBox128 {
    fn new_with_id<T: Sized>(instance: T, id: usize) -> Result<Self, ()> where Self: Sized {
        Ok(
            Self {
                unibox: UniBoxN::new(instance, id)?
            }
        )
    }

    unsafe fn as_ref<T: Sized>(&self) -> &T {
        self.unibox.as_ref()
    }

    unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T {
        self.unibox.as_mut_ref()
    }

    fn len(&self) -> usize {
        self.unibox.len()
    }

    fn id(&self) -> usize {
        self.unibox.id()
    }
}

/// Store a type on stack with a max size of 256 bytes.
pub struct UniBox256 {
    unibox: UniBoxN<[u8; 256]>
}

impl Uniboxed for UniBox256 {
    fn new_with_id<T: Sized>(instance: T, id: usize) -> Result<Self, ()> where Self: Sized {
        Ok(
            Self {
                unibox: UniBoxN::new(instance, id)?
            }
        )
    }

    unsafe fn as_ref<T: Sized>(&self) -> &T {
        self.unibox.as_ref()
    }

    unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T {
        self.unibox.as_mut_ref()
    }

    fn len(&self) -> usize {
        self.unibox.len()
    }

    fn id(&self) -> usize {
        self.unibox.id()
    }
}
