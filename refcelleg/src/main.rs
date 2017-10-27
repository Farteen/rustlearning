use std::cell::RefCell;

fn a_fn_that_immutablely_borrow(a: &i32) {
    println!("a is {}",a);
}

fn a_fn_that_mutably_borrow(b: &mut i32) {
	*b += 1;
}

fn demo(r: &RefCell<i32>) {
	a_fn_that_immutablely_borrow(&r.borrow());
	a_fn_that_mutably_borrow(&mut r.borrow_mut());
	a_fn_that_immutablely_borrow(&r.borrow());
}

fn main() {
	// let data = RefCell::new(5);
	// demo(&data);

	let mut s = String::from("hello");
	let r1 = &mut s;
	let r2 = &mut s;
}
