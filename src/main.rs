use unibox::{UniBox, UniBox128, UniBox64, UniBox32, Uniboxed};

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

// To check memory leaks
fn _main() {
    let mut line = String::new();
    println!("Press enter to start...");
    std::io::stdin().read_line(&mut line).unwrap_or_default();
 
    loop {
        let x = UniBox128::new(
            User {
                name: "Andreu".to_owned(),
                surname: "Llop".to_owned(),
                age: 37,
                address: Address {
                    street: "Carrer del Julivert".to_owned(),
                    number: 101,
                    city: "Vila del Pingüí".to_owned(),
                    zip: 888888,
                    country_code: ['A' as u8, 'D' as u8]
                }
            }
        ).expect("Couldn't create UniBox128 for User");
        
        let y = UniBox64::new(
            Address {
                street: "Sense Nom".to_owned(),
                number: 666,
                city: "Infern".to_owned(),
                zip: 55555,
                country_code: ['C' as u8, 'T' as u8]
            }
        ).expect("Couldn't create UniBox64 for Address");

        let mut z = UniBox::new(
    User {
                name: "Andreu".to_owned(),
                surname: "Llop".to_owned(),
                age: 37,
                address: Address {
                    street: "Carrer del Julivert".to_owned(),
                    number: 101,
                    city: "Vila del Pingüí".to_owned(),
                    zip: 888888,
                    country_code: ['A' as u8, 'D' as u8]
                }
            }
        ).expect("Couldn't create dynamic UniBox for User");

        unsafe { z.as_mut_ref::<User>() }.address.street = "Changed street".to_owned();

        core::mem::drop(x);
        core::mem::drop(y);
        core::mem::drop(z);
    }
}

fn main() {
    const ADDR_ID : usize = 1;
    const USER_ID : usize = 2;

    let ub1 = UniBox128::new_with_id(
        User {
            name: "Andreu".to_owned(),
            surname: "Llop".to_owned(),
            age: 37,
            address: Address {
                street: "Carrer del Julivert".to_owned(),
                number: 101,
                city: "Vila del Pingüí".to_owned(),
                zip: 888888,
                country_code: ['A' as u8, 'D' as u8]
            }
        },
        USER_ID
    ).expect("Couldn't create UniBox128 for User");
    
    let ub2 = UniBox128::new_with_id(
        Address {
            street: "Sense Nom".to_owned(),
            number: 666,
            city: "Infern".to_owned(),
            zip: 55555,
            country_code: ['C' as u8, 'T' as u8]
        },
        ADDR_ID
    ).expect("Couldn't create UniBox128 for Address");

    let user_ref = unsafe { ub1.as_ref::<User>() };
    let addr_ref = unsafe { ub2.as_ref::<Address>() };

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
            street: "Carrer de l'Església".to_owned(),
            number: 203,
            city: "Calella".to_owned(),
            zip: 08370,
            country_code: ['C' as u8, 'T' as u8]
        }
    ).expect("Couldn't create UniBox64 for Address");

    println!("{:#?}", unsafe { ub3.as_ref::<Address>() });

    println!("---- Create dynamic uniboxes ----");

    let ub4 = UniBox::new(
Address {
            street: "Carrer de l'Església".to_owned(),
            number: 203,
            city: "Calella".to_owned(),
            zip: 08370,
            country_code: ['C' as u8, 'T' as u8]
        }
    ).expect("Couldn't create dynamic UniBox for Address");

    let mut ub5 = UniBox::new(
User {
            name: "Andreu".to_owned(),
            surname: "Llop".to_owned(),
            age: 37,
            address: Address {
                street: "Carrer del Julivert".to_owned(),
                number: 101,
                city: "Vila del Pingüí".to_owned(),
                zip: 888888,
                country_code: ['A' as u8, 'D' as u8]
            }
        }
    ).expect("Couldn't create dynamic UniBox for User");

    println!("{:#?}", unsafe { ub4.as_ref::<Address>() });
    println!("{:#?}", unsafe { ub5.as_ref::<User>() });

    println!("---- After mutating address street ----");

    unsafe { ub5.as_mut_ref::<User>() }.address.street = "Changed street".to_owned();
    println!("{:#?}", unsafe { ub5.as_ref::<User>() });

    println!("---- Create uniboxed enum ----");

    #[derive(Debug)]
    #[allow(dead_code)]
    enum Color {
        Red,
        Gree,
        Blue,
        Custom(u8, u8, u8)
    }

    let ub6 = UniBox32::new(Color::Blue).expect("Failed uniboxing Color");
    println!("{:#?}", unsafe { ub6.as_ref::<Color>() });
    let ub6 = UniBox32::new(Color::Custom(0,100,200)).expect("Failed uniboxing Color");
    println!("{:#?}", unsafe { ub6.as_ref::<Color>() });

    println!("---- Create uniboxed primitive types ----");

    let ub7 = UniBox32::new([10, 100, 1000, -10000]).expect("Failed uniboxing [i32; 4]");
    println!("{:#?}", unsafe { ub7.as_ref::<[i32; 4]>() });

    let ub7 = UniBox32::new("hello").expect("Failed uniboxing &str");
    println!("{:#?}", unsafe { ub7.as_ref::<&str>() });

    let ub7 = UniBox32::new(99.99).expect("Failed uniboxing f64");
    println!("{:#?}", unsafe { ub7.as_ref::<f64>() });

    println!("---- Finish and drop all ----");
}