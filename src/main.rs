use unibox::stack::UniBox128;
use core::mem;

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

//TODO: generate autodrop functions with a macro. Someting like:
//#[autodrop(func_name)]
//struct MyStruct { ... }
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
    ).expect("Couldn't create UniBox128 for User");
    
    let ub2 = UniBox128::new(
        Address {
            street: "Carrer Escoles Pies".to_owned(),
            number: 42,
            city: "Calella".to_owned(),
            zip: 08370,
            country_code: ['C' as u8, 'T' as u8]
        },
        drop_addr
    ).expect("Couldn't create UniBox128 for Address");

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