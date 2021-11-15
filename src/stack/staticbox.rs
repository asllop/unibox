use super::super::Uniboxed;
use super::UniBoxN;

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
