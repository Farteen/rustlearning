#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
  		println!("Dropping...");
    }
}

use std::mem::drop;

fn main() {
    // println!("Hello, world!");
    let c = CustomSmartPointer {data: String::from("data")};
    println!("CustomSmartPointer created");
    drop(c);
    println!("Wait for it ...");
}
