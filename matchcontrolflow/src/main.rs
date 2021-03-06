fn main() {
	let five = Some(5);
	let six = plus_one(five);
	let none = plus_one(None);
}


#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState),
}
#[derive(Debug)]
enum USState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
        	println!("Lucky Penny");
        	1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
        	println!("State quarter from {:?}",state);
        	25
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}