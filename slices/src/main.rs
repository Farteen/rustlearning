fn main() {
	// let mut s = String::from("hello world");
	// let word = first_word(&s);
	// s.clear();

	let my_string = String::from("hello world");
	let world = first_word(&my_string[..]);
	let my_string_literal = "hello world";
	let word = first_word(&my_string_literal);
	let word = first_word(my_string_literal);
}


fn first_word(s: &str) -> &str {
    // let bytes = s.as_bytes();
    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }
    // s.len()

    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}