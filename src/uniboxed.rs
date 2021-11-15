/// Generic trait for all uniboxes.
pub trait Uniboxed {
    /// Create a new UniBox instance.
    fn new<T: Sized>(instance: T) -> Result<Self, ()> where Self: Sized {
        Self::new_with_id(instance, 0)
    }
    /// Create a new UniBox instance.
    /// 
    /// Accepts an *instance* and an *id*: a custom defined identifier used to know what type lies inside.
    fn new_with_id<T: Sized>(instance: T, id: usize) -> Result<Self, ()> where Self: Sized;
    /// Get reference to stored data using a type.
    /// 
    /// **WARNING**: If you try to cast a type other than the one actually hosted, you may get a panic or any undefined behavior.
    unsafe fn as_ref<T: Sized>(&self) -> &T;
    /// Get mutable reference to stored data using a type.
    /// 
    /// **WARNING**: If you try to cast a type other than the one actually hosted, you may get a panic or any undefined behavior.
    unsafe fn as_mut_ref<T: Sized>(&mut self) -> &mut T;
    /// Stored data length.
    fn len(&self) -> usize;
    /// Type identifier assigned with [`Uniboxed::new_with_id`], or 0 if unibox was created with [`Uniboxed::new`].
    fn id(&self) -> usize;
}
