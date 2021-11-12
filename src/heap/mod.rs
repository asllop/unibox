use core::{
    mem,
    alloc::{
        Layout
    },
    ops::Drop,
    ptr
};
use super::Uniboxed;

/// Store a struct on heap.
pub struct UniBox {
    buffer: *mut u8,
    layout: Layout,
    id: usize,
    len: usize,
    autodrop: fn(&Self)
}

impl UniBox {
    fn new_with_alloc<T: Sized>(instance: T, id: usize, alloc_func: fn(Layout) -> *mut u8) -> Self where Self: Sized {
        let autodrop = |_self: &Self| {
            mem::drop(unsafe { _self.as_owned::<T>() });
        };
        let align = mem::align_of::<T>();
        let len = mem::size_of::<T>();
        let layout = Layout::from_size_align(len, align).unwrap();
        let buffer = alloc_func(layout);
        if buffer.is_null() {
            panic!("Null pointer exception");
        }
        let src = &instance as *const T;
        unsafe {
            core::ptr::copy(src, buffer as *mut T, 1)
        };
        mem::forget(instance);
        Self {
            buffer,
            layout,
            id,
            len,
            autodrop
        }
    }
    
    unsafe fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        ptr::read(self.buffer as *const T)
    }

    //TODO: implement for no_std and no_alloc
    // With std
    fn free(&self) {
        unsafe {
            std::alloc::dealloc(self.buffer, self.layout);
        }
    }
}

//TODO: use cfg to select features (std/no_std, alloc/no_alloc)

impl Uniboxed for UniBox {
    // With std
    fn new_with_id<T: Sized>(instance: T, id: usize) -> Result<Self, ()> where Self: Sized {
        Ok(
            Self::new_with_alloc(instance, id, |layout| {
                unsafe { std::alloc::alloc_zeroed(layout) }
            })
        )
    }

    /*
    // Without std, with alloc
    fn new_with_id<T: Sized>(instance: T, id: usize) -> Result<Self, ()> where Self: Sized {
        extern crate alloc;
        use alloc::alloc::alloc_zeroed;
        Ok(
            Self::new_with_alloc(instance, id, |layout| {
                unsafe { alloc_zeroed(layout) }
            })
        )
    }
    */

    /*
    // Without std and alloc
    fn new_with_id<T: Sized>(_: T, _: usize) -> Result<Self, ()> where Self: Sized {
        Err(())
    }
    */

    unsafe fn as_ref<T: Sized>(&self) -> &T {
        let len = mem::size_of::<T>();
        if len != self.len() {
            panic!("Size of hosted data and requiered type are different");
        }
        mem::transmute::<*mut u8, &T>(self.buffer)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl Drop for UniBox {
    fn drop(&mut self) {
        println!("UniBox(heap) dropped");
        (self.autodrop)(self);
        self.free();
    }
}