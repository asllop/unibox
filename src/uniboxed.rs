/// Generic trait for all uniboxes.
pub trait Uniboxed {
    /// Create a new UniBox instance.
    fn new<T: Sized>(instance: T) -> Result<Self, ()> where Self: Sized;
    /// Get reference to stored data using a type.
    /// 
    /// **WARNING**: If you try to cast a type other than the one actually hosted, it will panic.
    unsafe fn as_ref<T: Sized>(&self) -> &T;
    /// Get mutable reference to stored data using a type.
    /// 
    /// **WARNING**: If you try to cast a type other than the one actually hosted, it will panic.
    unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T;
    /// Stored data length.
    fn len(&self) -> usize;
    /// Type identifier. Automatically assigned with [`core::any::type_name`].
    fn id(&self) -> &'static str;
    /// Check if the provided and hosted types are the same.
    fn check_type<T>(&self) -> bool {
        self.len() == core::mem::size_of::<T>() && self.id() == core::any::type_name::<T>()
    }
}
