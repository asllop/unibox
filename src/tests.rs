use crate::{
    Uniboxed, UniBox32, UniBox64, UniBox128, UniBox256, UniBox
};

fn check_sucession(arr: &[u8]) -> bool {
    for (i, v) in arr.iter().enumerate() {
        if *v != i as u8 {
            return false;
        }
    }
    true
}

fn init_array(arr: &mut [u8]) {
    for (i, v) in arr.iter_mut().enumerate() {
        *v = i as u8;
    }
}

trait TestArrayStruct {
    fn new() -> Self where Self: Sized;
    fn check(&self) -> bool;
}

struct Test32([u8; 32]);
impl TestArrayStruct for Test32 {
    fn new() -> Self {
        let mut arr = [0; 32];
        init_array(&mut arr);
        Self(arr)
    }

    fn check(&self) -> bool {
        check_sucession(&self.0)
    }
}

struct Test64([u8; 64]);
impl TestArrayStruct for Test64 {
    fn new() -> Self {
        let mut arr = [0; 64];
        init_array(&mut arr);
        Self(arr)
    }

    fn check(&self) -> bool {
        check_sucession(&self.0)
    }
}

struct Test128([u8; 128]);
impl TestArrayStruct for Test128 {
    fn new() -> Self {
        let mut arr = [0; 128];
        init_array(&mut arr);
        Self(arr)
    }

    fn check(&self) -> bool {
        check_sucession(&self.0)
    }
}

struct Test256([u8; 256]);
impl TestArrayStruct for Test256 {
    fn new() -> Self {
        let mut arr = [0; 256];
        init_array(&mut arr);
        Self(arr)
    }

    fn check(&self) -> bool {
        check_sucession(&self.0)
    }
}

fn test_type<T: TestArrayStruct, U: Uniboxed>() {
    let ubox = U::new(T::new()).expect("Couldn't create a uniboxed type");
    let inner = unsafe { ubox.as_ref::<T>() };
    assert!(inner.check(), "Content is incorrect");
}

#[test]
fn static_32() {
    test_type::<Test32, UniBox32>();
}

#[test]
fn static_64() {
    test_type::<Test64, UniBox64>();
}

#[test]
fn static_128() {
    test_type::<Test128, UniBox128>();
}

#[test]
fn static_256() {
    test_type::<Test256, UniBox256>();
}

#[test]
fn dynamic() {
    test_type::<Test32, UniBox>();
    test_type::<Test64, UniBox>();
    test_type::<Test128, UniBox>();
    test_type::<Test256, UniBox>();
}
