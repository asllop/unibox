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
    /// Type length.
    fn len() -> usize;
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

    fn len() -> usize {
        32
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

    fn len() -> usize {
        64
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

    fn len() -> usize {
        128
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

    fn len() -> usize {
        256
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
pub struct UniBoxN<S: Buffer> {
    data: S,
    len: usize,
    autodrop: fn(&Self),
    id: usize
}

impl<S: Buffer> UniBoxN<S> {
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
        if len > S::len() {
            Err(())
        }
        else {
            let mut data = S::init();
            data.copy_from_byte(bytes, len);
            mem::forget(instance);
            Ok(
                Self {
                    data,
                    len,
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
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        mem::transmute::<&S, &T>(&self.data)
    }

    /// Get mutable reference to stored data using a type.
    /// 
    /// **WARNING**: If you try to cast a type other than the one actually hosted, you may get a panic or any undefined behavior.
    pub unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        mem::transmute::<&mut S, &mut T>(&mut self.data)
    }

    /// Get owned internal type.
    /// 
    /// **WARNING**: After calling this method, the internal buffer may contain invalid data and must not be used anymore.
    pub unsafe fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        ptr::read(self.data.ptr() as *const T)
    }

    /// Stored data length.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Type identifier.
    pub fn id(&self) -> usize {
        self.id
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
