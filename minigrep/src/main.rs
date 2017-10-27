extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
	let args: Vec<String> = env::args().collect();

	let config = Config::new(&args).unwrap_or_else(|err| {
		println!("problem parsing arguments {}", err);
		process::exit(1);
	});


	// println!("{:?}",args);
	// let query = &args[1];
	// let filename = &args[2];
	// println!("Searching for {}",config.query);
	// println!("In file {}",config.filename);

	// let mut f = File::open(config.filename)
	// 	.expect("file not found");

	// let mut contents = String::new();

	// f.read_to_string(&mut contents)
	// 	.expect("something went wrong reading file");

	// println!("with text: \n{}",contents);

	if let Err(e) = minigrep::run(config) {
		println!("Application error {}",e);
		process::exit(1);
	}

}

// fn run(config: Config) -> Result<(), Box<Error>> {
// 	let mut f = File::open(config.filename).expect("file not found");
// 	let mut contents = String::new();
// 	f.read_to_string(&mut contents)?;
// 	println!("With text:\n{}", contents);
// 	Ok(())
// }

// #[derive(Debug)]
// struct Config {
//     query: String,
//     filename: String,
// }

// impl Config {
// 	fn new(args: &[String]) -> Result<Config, &'static str> {
// 		if args.len() < 3 {
// 		    // panic!("not enough arguments");
// 		    return Err("not enough arguments");
// 		}
// 	    let query = args[1].clone();
// 	    let filename = args[2].clone();

// 	    Ok(Config{ query, filename })
// 	}
// }

// fn parse_config(args: &[String]) -> Config {	
//     let query = args[1].clone();
//     let filename = args[2].clone();
//     Config { query, filename }
// }
