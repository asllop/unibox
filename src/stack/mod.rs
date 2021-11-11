use core::{
    slice,
    mem,
    ptr,
    ops::Drop,
};

//TODO: create a UniBox64 and a UniBox256

/// Store a struct on stack with a max size of 128 bytes.
pub struct UniBox128 {
    data: [u8; 128],
    len: usize,
    autodrop: fn(&Self)
}

impl UniBox128 {
    /// Create a new UniBox instance.
    /// 
    /// Returns Err if the struct is bigger than 128 bytes.
    pub fn new<T: Sized>(instance: T, autodrop: fn(&Self)) -> Result<Self, ()> {
        let bytes = unsafe {
            slice::from_raw_parts(
                (&instance as *const T) as *const u8,
                mem::size_of::<T>()
            )
        };
        let len = bytes.len();
        if len > 128 {
            Err(())
        }
        else {
            let mut data = [0; 128];
            data[0..len].clone_from_slice(bytes);
            mem::forget(instance);
            Ok(
                Self {
                    data,
                    len,
                    autodrop
                }
            )
        }
    }

    /// Get reference to stored data using a type.
    pub fn as_ref<T: Sized>(&self) -> &T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        unsafe {
            mem::transmute::<&[u8; 128], &T>(&self.data)
        }
    }

    /// Gets ownership of the stored value.
    /// 
    /// After this call, the data hosted inside the UniBox won't be valid anymore.
    pub fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        let mut buf = [0u8; 128];
        buf[0..len].clone_from_slice(&self.data[0..len]);
        unsafe {
            ptr::read(buf.as_ptr() as *const T)
        }
    }
}

impl Drop for UniBox128 {
    fn drop(&mut self) {
        println!("UniBox128 dropped");
        (self.autodrop)(self);
    }
}