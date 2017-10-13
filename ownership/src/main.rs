fn main() {
	// let mut s = String::from("Hello");
	// s.push_str(", world!");

	// println!("{}",s);
	// let x = 5;
	// let y = x;

	// let s1 = String::from("Hello");
	// let s2 = s1;
	// println!("{}, world",s1);

	// let s1 = String::from("Hello");
	// let s2 = s1.clone();
	// println!("s1 = {}, s2 = {}",s1, s2);

	// let x = 5;
	// let y = x.clone();
	// println!("x = {}, y ={}",x, y);

	// let hello = "hello";
	// let hello2 = hello;
	// println!("hello1 {}, hello2{}",hello, hello2);

	// let s = String::from("hello");
	// takes_ownership(s);
	// println!("{}",s);

	// let x = 5;
	// makes_copy(x);

	// let s1 = gives_ownership();

	// let s2 = String::from("hello");

	// let s3 = takes_and_gives_back(s2);

	let s1 = String::from("hello");
	let (s2, len) = calculate_length(s1);
	println!("The length of '{}' is {}",s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
	let length = s.len();
	(s, length)
}

fn gives_ownership() -> String {
	let some_string = String::from("hello");
	some_string
}

fn takes_and_gives_back(arg: String) -> String {
	arg
}


// fn takes_ownership(arg: String) {
//     println!("{}",arg);
// }

// fn makes_copy(arg: i32) {
//     println!("{}",arg);
// }

