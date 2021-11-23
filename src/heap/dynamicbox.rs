use core::{
    mem,
    alloc::{
        Layout
    },
    ops::Drop,
    ptr
};
use super::super::Uniboxed;
extern crate alloc;

/// Store a type on heap.
pub struct UniBox {
    buffer: *mut u8,
    layout: Layout,
    id: &'static str,
    len: usize,
    autodrop: fn(&Self)
}

impl UniBox {
    unsafe fn as_owned<T: Sized>(&self) -> T {
        ptr::read(self.buffer as *const T)
    }

    fn integrity_checks<T>(&self) {
        if !self.check_type::<T>() {
            panic!("Hosted and requiered types are different");
        }
    }
}

impl Uniboxed for UniBox {
    fn new<T: Sized>(instance: T) -> Result<Self, ()> where Self: Sized {
        let autodrop = |_self: &Self| {
            mem::drop(unsafe { _self.as_owned::<T>() });
        };
        let layout = Layout::new::<T>();
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
                id: core::any::type_name::<T>(),
                len: mem::size_of::<T>(),
                autodrop
            }
        )
    }

    unsafe fn as_ref<T: Sized>(&self) -> &T {
        self.integrity_checks::<T>();
        mem::transmute::<*mut u8, &T>(self.buffer)
    }

    unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T {
        self.integrity_checks::<T>();
        mem::transmute::<*mut u8, &mut T>(self.buffer)
    }

    fn len(&self) -> usize {
        self.len
    }

    fn id(&self) -> &'static str {
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
