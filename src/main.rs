use crate::aqbanking_wrapper::{hello, list_accounts};

mod aqbanking_wrapper;

fn main() {
    println!("Hello, world!");

    hello();

    list_accounts();
}
