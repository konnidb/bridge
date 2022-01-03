use std::{option, ops::Deref};

mod storage;
mod schemas;
fn main() {
    println!("Hello, world!");
    let option_vector = Some(vec!["asd", "b"]);

    let mut get = option_vector.unwrap();

    let item = get.get(0);
    let item = item.unwrap();
    let item = *item;

    println!("{}", &item);

    get.insert(0, "New");
    println!("{}", &item);
    let item = get.get(0).unwrap().deref();
    println!("{}", &item);

    
}
