# UniBox

*Universal Box that can store any type.*

Usually, when we want to store different types in a collection we use either one of the following techniques:

1. An enum to wrap all possible types.
2. A boxed trait that all types must implement.

Sometimes none of the above techniques are possible. The set of types might be unknown beforehand, for example in a library. And, when we implement a trait, we can only access the methods defined in the trait, not all the methods and properties of the original struct. Furthermore, we are required to use a Box, that means allocating dynamic memory, making `no_std` applications more complicated.

If you encountered any of these limitations, UniBox is probably a viable solution for your use case.

UniBox can:

- Store a generic struct in static or dynamic memory, you decide.
- Return a reference to any type.
- Be used to store mixed data in collections.

The crate offers two kinds of types:

- Static: uniboxes that store data without using heap memory. They have a fixed size and the type they host can't be bigger than that. Currently there are four types: `UniBox32`, `UniBox64`, `UniBox128` and `UniBox256`, to store types up to 32, 64, 128 and 256 bytes.
- Dynamic: store data by allocating memory, like a regular Box. There is only one type, `UniBox`.

## Usage

Suppose we have 2 different structs with little or nothing in common, like the following User and Server. And we want to store instances of these types in the same collection.

We can use static uniboxes like this:

```
use unibox::{ Uniboxed, UniBox64 };

#[derive(Debug)]
struct BornDate {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug)]
struct User {
    pub name: String,
    pub lastname: String,
    pub born: BornDate,
}

#[derive(Debug)]
struct Server {
    pub domain: String,
    pub port: u16
}

// We use ID to identify the different types ones uniboxed
const USER_ID : usize = 1111;
const SERVER_ID : usize = 2222;

// If we don't care about identifying the internal type, we can use UniBox64::new() instead
let ubox_usr = UniBox64::new_with_id(
    User {
        name: "John".to_owned(),
        lastname: "Dow".to_owned(),
        born: BornDate {
            year: 1984,
            month: 12,
            day: 25
        }
    },
    USER_ID
).expect("Couldn't create UniBox64 for User");

let ubox_server = UniBox64::new_with_id(
    Server {
        domain: "example.com".to_owned(),
        port: 8080
    },
    SERVER_ID
).expect("Couldn't create UniBox64 for User");

// Create a vector with the uniboxes
let v = vec!(ubox_usr, ubox_server);

for ubox in v.iter() {
    match ubox.id() {
        USER_ID => {
            // It's a User struct
            println!("{:#?}", unsafe { ubox.as_ref::<User>() });
        },
        SERVER_ID => {
            // It's a Server struct
            println!("{:#?}", unsafe { ubox.as_ref::<Server>() });
        },
        _ => {}
    }
}
```

The dynamic version, `UniBox` works exactly in the same way, the only difference is that it allocates memory to store the type and thus, you don't have to worry about it's size.

## Features and `no_std`

This crate is `no_std`, but it uses the [`alloc`](https://doc.rust-lang.org/alloc/) crate to allocate dynamic memory inside `UniBox`. This is controlled via a feature, enabled by default, named `alloc`.

If your environment doesn't provide the alloc crate, just disable the default features. If you do so, you won't be able to use `UniBox`.
