use std::cmp::PartialOrd;

fn main() {
	let number_list = vec![34, 50, 25, 100, 65];
	let mut largest = number_list[0];
	for number in number_list {
	    if number > largest {
	        largest = number;
	    }
	}

}


fn largest_i32(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
	let mut largest = list[0];

	for &item in list.iter() {
	    if item > largest {
	        largest = item;
	    }
	}

	largest
}


fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
