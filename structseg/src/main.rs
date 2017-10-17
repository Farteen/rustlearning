fn main() {
	let length = 50;
	let width = 50;
	let rect = Rectangle {
		length,
		width
	};

	let square = Rectangle::square(10);
	println!("{}",square.area());
	println!("The rect is {:?}",rect);
	// println!("The area of the rectangle is{} square pixel",area(length1, width1));
	// println!("The area of the rectangle is {} square pixel",area(&rect));
	// println!("the rect length is {}",rect.length);
}

// fn area(length: u32, width: u32) -> u32 {
// 	length * width
// }

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.length
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}


impl Rectangle {
	fn square(size: u32) -> Rectangle {
	    Rectangle { length: size, width: size }
	}

	fn area(&self) -> u32 {
		self.width * self.length
	}
}