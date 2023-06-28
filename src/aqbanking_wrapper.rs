#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn hello() {
    println!("Hello from the aqbanking wrapper");
}

pub fn list_accounts() {
    println!("Try to list accounts");
}
