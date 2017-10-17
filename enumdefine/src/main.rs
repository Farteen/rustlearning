fn main() {
	// let home = IpAddr {
	// 	kind: IpAddrKind::V4,
	// 	address: String::from("127.0.0.1"),
	// };
	// let loopback = IpAddr {
	// 	kind: IpAddrKind::V6,
	// 	address: String::from("::1"),
	// };
	// let home = IpAddrKind::V4(String::from("127.0.0.1"));
	// let loopback = IpAddrKind::V6(String::from("::1"));

	let some_number = Some(123);
	let some_string = Some("love love love");

	let absent_number: Option<i32> = None;
	

}


#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}

enum IpAddress {
	V4(u8, u8, u8, u8),
	V6(String),
}

enum Option<T> {
	Some(T),
	None,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}