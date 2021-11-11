use std::slice;
use std::mem;

struct UniBox128 {
    data: [u8; 128],
    len: usize,
    autodrop: fn(&Self)
}

impl UniBox128 {
    unsafe fn as_buf_ptr<T: Sized>(p: &T) -> &[u8] {
        slice::from_raw_parts(
            (p as *const T) as *const u8,
            mem::size_of::<T>()
        )
    }
    
    pub fn new<T: Sized>(instance: T, autodrop: fn(&Self)) -> Result<Self, ()> {
        let bytes = unsafe { Self::as_buf_ptr(&instance) };
        let len = bytes.len();
        
        dbg!(len);
        
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

    pub fn as_ref<T: Sized>(&self) -> &T {
        let len = mem::size_of::<T>();
        if len != self.len {
            panic!("Size of hosted data and requiered type are different");
        }
        unsafe {
            mem::transmute::<&[u8; 128], &T>(&self.data)
        }
    }

    pub fn as_owned<T: Sized>(&self) -> T {
        let len = mem::size_of::<T>();
        let mut buf = [0u8; 128];
        buf[0..len].clone_from_slice(&self.data[0..len]);
        unsafe {
            std::ptr::read(buf.as_ptr() as *const T)
        }
    }
}

impl Drop for UniBox128 {
    fn drop(&mut self) {
        println!("UniBox128 dropped");
        (self.autodrop)(self);
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    pub name: String,
    pub surname: String,
    pub age: u8,
    pub address: Address
}

impl Drop for User {
    fn drop(&mut self) {
        println!("User dropped");
    }
}

fn drop_user(ubox: &UniBox128) {
    mem::drop(ubox.as_owned::<User>());
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

fn drop_addr(ubox: &UniBox128) {
    mem::drop(ubox.as_owned::<Address>());
}

fn main() {
    let ub1 = UniBox128::new(
        User {
            name: "Andreu".to_owned(),
            surname: "Santarén".to_owned(),
            age: 37,
            address: Address {
                street: "Plaça piruleta".to_owned(),
                number: 101,
                city: "Vila del Pingüí".to_owned(),
                zip: 888888,
                country_code: ['J' as u8, 'P' as u8]
            }
        },
        drop_user
    ).unwrap();
    
    let ub2 = UniBox128::new(
        Address {
            street: "Carrer Escoles Pies".to_owned(),
            number: 42,
            city: "Calella".to_owned(),
            zip: 08370,
            country_code: ['C' as u8, 'T' as u8]
        },
        drop_addr
    ).unwrap();

    let user_ref = ub1.as_ref::<User>();
    let addr_ref = ub2.as_ref::<Address>();

    println!("---- Reference to structs ----");

    println!("{:#?}", user_ref);
    println!("{:#?}", addr_ref);

    // println!("---- Owned structs ----");

    // let user = ub1.as_owned::<User>();
    // let addr = ub2.as_owned::<Address>();

    // println!("{:#?}", user);
    // println!("{:#?}", addr);

    // mem::drop(addr);
    // mem::drop(user);

    // Pointers user_ref and addr_ref are no longer valid anymore, because structs have been droped
}