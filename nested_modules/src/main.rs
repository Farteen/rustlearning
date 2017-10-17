
pub mod a {
	pub mod series {
		pub mod of {
			pub fn nested_modules() {

			}
		}
	}
}

// use a::series::of;
use a::series::of::nested_modules;

// use TrafficLight::{ Red, Yellow};

use TrafficLight::*;

fn main() {
    // println!("Hello, world!");
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = Green;
}

enum TrafficLight {
	Red,
	Yellow,
	Green,
}
