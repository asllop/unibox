use core::{
    slice,
    mem,
    ptr,
    ops::Drop,
};

/// Generic trait for all static uniboxes.
pub trait StaticUniBox {
    fn new<T: Sized>(instance: T, autodrop: fn(&Self)) -> Result<Self, ()> where Self: Sized;
    fn as_ref<T: Sized>(&self) -> &T;
    fn as_owned<T: Sized>(&self) -> T;
    fn len(&self) -> usize;
}

/// Store a struct on stack with a max size of 64 bytes.
pub struct UniBox64 {
    data: [u8; 64],
    len: usize,
    autodrop: fn(&Self)
}

impl StaticUniBox for UniBox64 {
    /// Create a new UniBox instance.
    /// 
    /// Returns Err if the struct is bigger than 64 bytes.
    fn new<T: Sized>(instance: T, autodrop: fn(&Self)) -> Result<Self, ()> {
        let bytes = unsafe {
            slice::from_raw_parts(
                (&instance as *const T) as *const u8,
                mem::size_of::<T>()
            )
        };
        let len = bytes.len();
        if len > 64 {
            Err(())
        }
        else {
            let mut data = [0; 64];
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
    fn as_ref<T: Sized>(&self) -> &T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        unsafe {
            mem::transmute::<&[u8; 64], &T>(&self.data)
        }
    }

    /// Gets ownership of the stored value.
    /// 
    /// After this call, the data hosted inside the UniBox won't be valid anymore.
    fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        let mut buf = [0u8; 64];
        buf[0..len].clone_from_slice(&self.data[0..len]);
        unsafe {
            ptr::read(buf.as_ptr() as *const T)
        }
    }

    /// Stored data length.
    fn len(&self) -> usize {
        self.len
    }
}

impl Drop for UniBox64 {
    fn drop(&mut self) {
        println!("UniBox64 dropped");
        (self.autodrop)(self);
    }
}

/// Store a struct on stack with a max size of 128 bytes.
pub struct UniBox128 {
    data: [u8; 128],
    len: usize,
    autodrop: fn(&Self)
}

impl StaticUniBox for UniBox128 {
    /// Create a new UniBox instance.
    /// 
    /// Returns Err if the struct is bigger than 128 bytes.
    fn new<T: Sized>(instance: T, autodrop: fn(&Self)) -> Result<Self, ()> {
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
    fn as_ref<T: Sized>(&self) -> &T {
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
    fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        let mut buf = [0u8; 128];
        buf[0..len].clone_from_slice(&self.data[0..len]);
        unsafe {
            ptr::read(buf.as_ptr() as *const T)
        }
    }

    /// Stored data length.
    fn len(&self) -> usize {
        self.len
    }
}

impl Drop for UniBox128 {
    fn drop(&mut self) {
        println!("UniBox128 dropped");
        (self.autodrop)(self);
    }
}


/// Store a struct on stack with a max size of 256 bytes.
pub struct UniBox256 {
    data: [u8; 256],
    len: usize,
    autodrop: fn(&Self)
}

impl StaticUniBox for UniBox256 {
    /// Create a new UniBox instance.
    /// 
    /// Returns Err if the struct is bigger than 256 bytes.
    fn new<T: Sized>(instance: T, autodrop: fn(&Self)) -> Result<Self, ()> {
        let bytes = unsafe {
            slice::from_raw_parts(
                (&instance as *const T) as *const u8,
                mem::size_of::<T>()
            )
        };
        let len = bytes.len();
        if len > 256 {
            Err(())
        }
        else {
            let mut data = [0; 256];
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
    fn as_ref<T: Sized>(&self) -> &T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        unsafe {
            mem::transmute::<&[u8; 256], &T>(&self.data)
        }
    }

    /// Gets ownership of the stored value.
    /// 
    /// After this call, the data hosted inside the UniBox won't be valid anymore.
    fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        let mut buf = [0u8; 256];
        buf[0..len].clone_from_slice(&self.data[0..len]);
        unsafe {
            ptr::read(buf.as_ptr() as *const T)
        }
    }

    /// Stored data length.
    fn len(&self) -> usize {
        self.len
    }
}

impl Drop for UniBox256 {
    fn drop(&mut self) {
        println!("UniBox256 dropped");
        (self.autodrop)(self);
    }
}