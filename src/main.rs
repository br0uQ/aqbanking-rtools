
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


use crate::aqbanking_wrapper::{hello, list_accounts};
mod aqbanking_wrapper;
mod list_accs;

fn main() {
    println!("Hello, world!");

    hello();

    list_accounts();
}
