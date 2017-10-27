use std::mem;

fn main() {
	//You can create a String from a literal string with String::from:
    // let hello = String::from("Hello, world");
    //
    // let mut hello = String::from("hello, ");
    // let world = String::from("world");
    // // hello.push('w');
    // hello.push_str(world);
    // println!("{}",hello);
    // let mut sparkle_heart = vec![240, 159, 146, 150];
    // let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
    // println!("{}",sparkle_heart);

    // let s = String::from("Hello");
    // takes_str(&s);

    // let example_string = String::from("example_string");
    // // example_func(example_string.as_str());
    // example_func(&*example_string);
    let story = String::from("Once upon a time...");
    let ptr = story.as_ptr();
    let len = story.len();
    let capacity= story.capacity();

    assert_eq!(19, len);

    mem::forget(story);

    let s = unsafe { String::from_raw_parts(ptr as *mut _, len, capacity) };
    assert_eq!(String::from("Once upon a time..."), s);
}

fn takes_str(s: &str) {
    
}


trait TraitExample {
    
}

impl <'a> TraitExample for &'a str {

}

fn example_func<A: TraitExample>(example_arg: A) {

}

