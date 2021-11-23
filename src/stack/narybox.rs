use core::{
    slice,
    mem,
    ptr,
    ops::Drop
};
use super::Buffer;

/// Generic static unibox that can implement any [`Buffer`].
/// 
/// This is the base of other static types, and should not be used directly. Use it only to implement your custom static unibox type.
pub struct UniBoxN<B: Buffer> {
    data: B,
    len: usize,
    autodrop: fn(&Self),
    id: &'static str
}

impl<B: Buffer> UniBoxN<B> {
    /// Create a new UniBox instance.
    /// 
    /// Returns Err if the struct is bigger than N bytes (N being the size of the unibox).
    pub fn new<T: Sized>(instance: T) -> Result<Self, ()> {
        Self::new_with_id(instance, core::any::type_name::<T>())
    }

    /// Create a new UniBox instance.
    /// 
    /// Accepts an *instance* and an *id*: a custom defined identifier used to know what type lies inside.
    /// 
    /// Returns Err if the struct is bigger than N bytes (N being the size of the unibox).
    pub fn new_with_id<T: Sized>(instance: T, id: &'static str) -> Result<Self, ()> {
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
        self.integrity_checks::<T>();
        mem::transmute::<&B, &T>(&self.data)
    }

    /// Get mutable reference to stored data using a type.
    /// 
    /// **WARNING**: If you try to cast a type other than the one actually hosted, you may get a panic or any undefined behavior.
    pub unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T {
        self.integrity_checks::<T>();
        mem::transmute::<&mut B, &mut T>(&mut self.data)
    }

    /// Stored data length.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Type identifier.
    pub fn id(&self) -> &'static str {
        self.id
    }

    unsafe fn as_owned<T: Sized>(&self) -> T {
        ptr::read(self.data.ptr() as *const T)
    }

    pub fn check_type<T>(&self) -> bool {
        let len = mem::size_of::<T>();
        // Integrity checks
        len == self.len && self.id == core::any::type_name::<T>()
    }

    fn integrity_checks<T>(&self) {
        if !self.check_type::<T>() {
            panic!("Hosted and requiered types are different");
        }
    }
}

impl<S: Buffer> Drop for UniBoxN<S> {
    fn drop(&mut self) {
        (self.autodrop)(self);
    }
}
