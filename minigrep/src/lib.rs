use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub fn run(config: Config) -> Result<(), Box<Error>> {
	let mut f = File::open(config.filename).expect("file not found");
	let mut contents = String::new();
	f.read_to_string(&mut contents)?;

	for line in search(&config.query, &contents) {
		println!("{}",line);
	}
	
	// println!("With text:\n{}", contents);
	Ok(())
}

#[derive(Debug)]
pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

// impl Config {
// 	pub fn new(args: &[String]) -> Result<Config, &'static str> {
// 		if args.len() < 3 {
// 		    // panic!("not enough arguments");
// 		    return Err("not enough arguments");
// 		}
// 	    let query = args[1].clone();
// 	    let filename = args[2].clone();

// 	    Ok(Config{ query, filename })
// 	}
// }

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
  		args.next();

  		let query = match args.next() {
  		    Some(arg) => arg,
  		    None => return Err("Didn't get a query string"),
  		};

  		let filename = match args.next() {
  		    Some(arg) => arg,
  		    None => return Err("Didn't get a filename"),
  		};

  		let case_sensitive = env::var("CASE_SENSITIVE").is_err();

  		Ok(Config{
  			query, filename, case_sensitive,
  		})
    }
}

mod test {
	use super::*;

	#[test]
	fn one_result() {
		let query = "duct";
		let contents = "\
		Rust:
		safe, fast, productive.
		Pick three.";

		assert_eq!(vec!["safe, fast, productive"],search(query, contents));
	}
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	contents.lines()
		.filter(|line| line.contains(query))
		.collect()
}

// pub fn search<'a>(query: &'a str, contents:&'a str) -> Vec<&'a str> {
	// vec![]
	// let mut results = Vec::new();
	// for line in contents.lines() {
	//     if line.contains(query) {
	//         results.push(line);
	//     }
	// }
	// results
// }