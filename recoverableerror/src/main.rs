use std::io;
use std::io::Read;
use std::fs::File;
// use std::io::ErrorKind;

fn main() {
	// let f: u32 = File::open("hello.txt");
	// let f = File::open("hello.txt");
	// let f = match f {
	//     Ok(file) => file,
	//     Err(ref error) if error.kind() == ErrorKind::NotFound => {
	//     	match File::create("hello.txt") {	
	//     	    Ok(fc) => fc,
	//     	    Err(e) => {
	//     	    	panic!(
	//     	    		"tried to create file but there was a problem {:?}",
	//     	    		e
	// 				)
	//     	 	},
	//     	}
	//     },
	//     Err(error) => {
	//     	panic!("there was a problem in reading file {:?}",error);
	//     }
	// };
	// let f = File::open("hello.txt").unwrap();



}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
	let mut f = match f {
	    Ok(file) => file,
	    Err(e) => return Err(e),
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
	    Ok(_) => Ok(s),
	    Err(e) => Err(e),
	}
}
