//! # UniBox
//! 
//! *Universal Box that can store any type.*
//! 
//! Usually, when we want to store different types in a collection we use either one of the following techniques:
//! 
//! 1. An enum to wrap all possible types.
//! 2. A trait that all types must implement.
//! 
//! Sometimes none of the above techniques are possible. The set of types may be unknown beforehand, for example in a library. And when we implement a trait, we can only access the methods defined in the trait, not all the methods and properties of the original struct. Furthermore, we are required to use a Box, that is, to allocate memory, making `no_std` applications more complicated.
//! 
//! If you encountered any of these limitations, UniBox may be a useful solution for your use case.
//! 
//! - UniBox can store a generic struct in heap or stack, you decide.
//! - Can get a reference to any type.
//! - Store mixed data in collections.
//! 

pub mod heap;
pub mod stack;