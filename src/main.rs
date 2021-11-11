use unibox::stack::{
    StaticUniBox, UniBox64, UniBox128
};
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

fn drop_user<T: StaticUniBox>(ubox: &T) {
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
fn drop_addr<T: StaticUniBox>(ubox: &T) {
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

    println!("---- Vector of UniBoxes ----");

    let v = vec!(ub1, ub2);

    for b in v.iter() {
        println!("UniBox len = {}", b.len());
    }

    println!("---- Create 64 bytes unibox ----");

    // Create a smaller piece
    let ub3 = UniBox64::new(
        Address {
            street: "Carrer Escoles Pies".to_owned(),
            number: 42,
            city: "Calella".to_owned(),
            zip: 08370,
            country_code: ['C' as u8, 'T' as u8]
        },
        drop_addr
    ).expect("Couldn't create UniBox64 for Address");

    println!("{:#?}", ub3.as_ref::<Address>());

    println!("---- Finish and drop all ----");
}