fn main() {
    // let mut s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // change(&mut s1);
    // println!("The length of '{}' is {}",s1, len);

    // let mut s = String::from("hello");
    // {
    // 	let r1 = &mut s;
    // }
    // let r2 = &mut s;

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;

    // let r3 = &mut s;
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("Hello");
    &s
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}