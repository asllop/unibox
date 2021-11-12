use core::{
    slice,
    mem,
    ptr,
    ops::Drop
};

/// Generic trait for all static uniboxes.
pub trait StaticUniBox {
    /// Create a new UniBox instance.
    /// 
    /// Returns Err if the struct is bigger than N bytes (N being the size of the unibox).
    fn new<T: Sized>(instance: T) -> Result<Self, ()> where Self: Sized {
        Self::new_with_id(instance, 0)
    }
    /// Create a new UniBox instance.
    /// 
    /// Accepts an *instance* and an *id*: a custom defined identifier used to know what type lies inside.
    /// 
    /// Returns Err if the struct is bigger than N bytes (N being the size of the unibox).
    fn new_with_id<T: Sized>(instance: T, id: usize) -> Result<Self, ()> where Self: Sized;
    /// Get reference to stored data using a type.
    /// 
    /// **WARNING**: If you try to cast a type other than the one actually hosted, you may get a panic or any undefined behavior.
    unsafe fn as_ref<T: Sized>(&self) -> &T;
    /// Stored data length.
    fn len(&self) -> usize;
    /// Type identifier.
    fn id(&self) -> usize;
}

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
            mem::drop(_self.as_owned::<T>());
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
    pub fn as_ref<T: Sized>(&self) -> &T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        unsafe {
            mem::transmute::<&S, &T>(&self.data)
        }
    }

    /// Get owned internal type.
    /// 
    /// **WARNING**: After calling this method, the internal buffer may contain invalid data and must not be used anymore.
    pub fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        let mut buf = S::init();
        buf.copy_from_type(&self.data, len);
        unsafe {
            ptr::read(buf.ptr())
        }
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
        println!("UniBoxN({}) dropped", S::len());
        (self.autodrop)(self);
    }
}

/// Store a struct on stack with a max size of 64 bytes.
pub struct UniBox64 {
    unibox: UniBoxN<[u8; 64]>
}

impl StaticUniBox for UniBox64 {
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

    fn len(&self) -> usize {
        self.unibox.len()
    }

    fn id(&self) -> usize {
        self.unibox.id()
    }
}

/// Store a struct on stack with a max size of 128 bytes.
pub struct UniBox128 {
    unibox: UniBoxN<[u8; 128]>
}

impl StaticUniBox for UniBox128 {
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

    fn len(&self) -> usize {
        self.unibox.len()
    }

    fn id(&self) -> usize {
        self.unibox.id()
    }
}

/// Store a struct on stack with a max size of 256 bytes.
pub struct UniBox256 {
    unibox: UniBoxN<[u8; 256]>
}

impl StaticUniBox for UniBox256 {
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

    fn len(&self) -> usize {
        self.unibox.len()
    }

    fn id(&self) -> usize {
        self.unibox.id()
    }
}
