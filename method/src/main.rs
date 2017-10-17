fn main() {
    // println!("Hello, world!");

    let rect1 = Rectangle{length: 20, width: 30};
    println!("{:#?}",rect1.area());
}


#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
    	self.length * self.width
    }
}