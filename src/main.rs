use std::slice;
use std::mem;

// Create a smart pointer like Box that hold a fixed amount of memory and doesn't use heap, and allocates structs and then cast to any type.
#[derive(Debug)]
struct UniBox64 {
    data: [u8; 64],
    len: usize
}

impl UniBox64 {
    unsafe fn as_buf_ptr<T: Sized>(p: &T) -> &[u8] {
        slice::from_raw_parts(
            (p as *const T) as *const u8,
            mem::size_of::<T>()
        )
    }
    
    pub fn from<T: Sized>(instance: T) -> Result<Self, ()> {
        let bytes = unsafe { Self::as_buf_ptr(&instance) };
        let len = bytes.len();
        
        dbg!(len);
        
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
                    len
                }
            )
        }
    }

    pub fn as_ref<T: Sized>(&self) -> &T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        unsafe {
            mem::transmute::<&[u8; 64], &T>(&self.data)
        }
    }

    pub fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        let mut buf = [0u8; 64];
        buf[0..len].clone_from_slice(&self.data[0..len]);
        unsafe {
            std::ptr::read(buf.as_ptr() as *const T)
        }
    }
}

impl Drop for UniBox64 {
    fn drop(&mut self) {
        println!("UniBox64 dropped");
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    pub name: String,
    pub surname: String,
    pub age: u8
}

impl Drop for User {
    fn drop(&mut self) {
        println!("User dropped");
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Address {
    pub street: String,
    pub number: u32,
    pub city: String,
    pub zip: u32,
    pub country_code: [u8; 2]
}

impl Drop for Address {
    fn drop(&mut self) {
        println!("Address dropped");
    }
}

fn main() {
    let ub1 = UniBox64::from(
        User {
            name: "Andreu".to_owned(),
            surname: "Santarén".to_owned(),
            age: 37
        }
    ).unwrap();
    
    let ub2 = UniBox64::from(
        Address {
            street: "Carrer Escoles Pies".to_owned(),
            number: 42,
            city: "Calella".to_owned(),
            zip: 08370,
            country_code: ['E' as u8, 'S' as u8]
        }
    ).unwrap();

    let user_ref = ub1.as_ref::<User>();
    let addr_ref = ub2.as_ref::<Address>();

    println!("---- Reference to structs ----");

    println!("{:#?}", user_ref);
    println!("{:#?}", addr_ref);

    println!("---- Owned structs ----");

    let user = ub1.as_owned::<User>();
    let addr = ub2.as_owned::<Address>();

    println!("{:#?}", user);
    println!("{:#?}", addr);

    mem::drop(addr);
    mem::drop(user);

    println!("---- Values of references after dropping structs ----");

    // Pointers are no longer valid, because structs have been droped

    println!("{:#?}", user_ref);
    println!("{:#?}", addr_ref);
}