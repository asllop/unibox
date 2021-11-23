# UniBox

*Universal Box.*

Usually, when we want to store different types in a collection we use either one of the following techniques:

1. An enum to wrap all possible types.
2. A boxed trait that all types must implement.

Sometimes none of the above techniques are possible. The set of types might be unknown beforehand, for example in a library. And, when we implement a trait, we can only access the methods defined in the trait, not all the methods and properties of the original struct. Furthermore, we are required to use a Box, that means allocating dynamic memory, making `no_std` applications more complicated.

If you encountered any of these limitations, UniBox is probably a viable solution for your use case.

UniBox can:

- Store a generic type without using generics in the struct signature.
- Use either static or dynamic memory, you decide.
- Return a reference to any type.
- Be used to store mixed data in collections or arrays.

UniBox offers two kinds of types:

- *Static*: uniboxes that store data without using the heap. They have a fixed size, and the type they host can't be bigger than that. Currently there are four types: `UniBox32`, `UniBox64`, `UniBox128` and `UniBox256`, to store types up to 32, 64, 128 and 256 bytes. All these types are based on the generic static type, `UniBoxN`, that can also be used to implement custom static uniboxes.
- *Dynamic*: store data by allocating memory, like a regular Box. There is only one type, `UniBox`.

## Usage

Suppose we have 2 different structs with little or nothing in common, like the following User and Server. And we want to store instances of these types in the same collection.

We can use static uniboxes like so:

```rust
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

let ubox_usr = UniBox64::new(
    User {
        name: "John".to_owned(),
        lastname: "Dow".to_owned(),
        born: BornDate {
            year: 1984,
            month: 12,
            day: 25
        }
    }
).expect("Couldn't create UniBox64 for User");

let ubox_server = UniBox64::new(
    Server {
        domain: "example.com".to_owned(),
        port: 8080
    }
).expect("Couldn't create UniBox64 for Server");

// Create a vector with the uniboxes
let v = vec!(ubox_usr, ubox_server);

for ubox in v.iter() {
    match ubox.id() {
        "my_crate::User" => {
            // It's a User struct
            println!("{:#?}", unsafe { ubox.as_ref::<User>() });
        },
        "my_crate::Server" => {
            // It's a Server struct
            println!("{:#?}", unsafe { ubox.as_ref::<Server>() });
        },
        _ => {}
    }
}
```

The dynamic version, `UniBox`, works exactly in the same way, the only difference is that it allocates memory to store the type and thus, you don't have to worry about the size.

## Uniboxing types with references

Is possible to unibox a type that contains a reference with non-static lifetime, like so:

```rust
struct MyStruct<'a> {
    my_ref: &'a [i32]
}

let arr = [1, 2, 3, 4, 5];

let ubox = UniBox32::new(
    MyStruct {
        my_ref: &arr
    }
).expect("Failed uniboxing MyStruct");

println!("{:#?}", unsafe { ubox.as_ref::<MyStruct>() }.my_ref);
```

But once the type is embedded inside a UniBox, the rust compiler looses track of it, and it won't be able to ensure that lifetime constraints are observed. For this reason, is the programmer who must make sure that no references are used after being droped the original value. That's the main reason why `Uniboxed::as_ref` and `Uniboxed::as_mut_ref` are unsafe.

## Why not `Any`?

The [`Any`](https://doc.rust-lang.org/std/any/trait.Any.html) trait exposes a similar functionality, it allows a generic type to be casted, but it has some limitations compared to uniboxes:

1. Only types with static references can be used, so something like the following can't be allocated inside a `Box<dyn Any>`:

```rust
struct MyStruct<'a> {
    my_ref: &'a [i32]
}
```

2. The size of a `dyn Any` can't be known at compile time, and thus, we can't use it to store generic types inside arrays or other non-heap memory artifacts.

## Features and `no_std`

This crate is `no_std`, but it uses the [`alloc`](https://doc.rust-lang.org/alloc/) crate to allocate dynamic memory inside `UniBox`. This is controlled via a feature, enabled by default, named `alloc`.

If your environment doesn't provide the alloc crate, just disable the default features. If you do so, you won't be able to use `UniBox` type.
