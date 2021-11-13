use core::{
    mem,
    alloc::{
        Layout
    },
    ops::Drop,
    ptr
};
use super::Uniboxed;
extern crate alloc;

/// Store a type on heap.
pub struct UniBox {
    buffer: *mut u8,
    layout: Layout,
    id: usize,
    len: usize,
    autodrop: fn(&Self)
}

impl UniBox {
    unsafe fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        ptr::read(self.buffer as *const T)
    }
}

impl Uniboxed for UniBox {
    fn new_with_id<T: Sized>(instance: T, id: usize) -> Result<Self, ()> where Self: Sized {
        let autodrop = |_self: &Self| {
            mem::drop(unsafe { _self.as_owned::<T>() });
        };
        let align = mem::align_of::<T>();
        let len = mem::size_of::<T>();
        let layout = Layout::from_size_align(len, align).unwrap();
        let buffer = unsafe { alloc::alloc::alloc(layout) };
        if buffer.is_null() {
            return Err(());
        }
        let src = &instance as *const T;
        unsafe {
            core::ptr::copy(src, buffer as *mut T, 1)
        };
        mem::forget(instance);
        Ok(
            Self {
                buffer,
                layout,
                id,
                len,
                autodrop
            }
        )
    }

    unsafe fn as_ref<T: Sized>(&self) -> &T {
        let len = mem::size_of::<T>();
        if len != self.len() {
            panic!("Size of hosted data and requiered type are different");
        }
        mem::transmute::<*mut u8, &T>(self.buffer)
    }

    unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T {
        let len = mem::size_of::<T>();
        if len != self.len() {
            panic!("Size of hosted data and requiered type are different");
        }
        mem::transmute::<*mut u8, &mut T>(self.buffer)
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
        (self.autodrop)(self);
        unsafe {
            alloc::alloc::dealloc(self.buffer, self.layout);
        }
    }
}