use std::collections::HashMap;

fn main() {
	// let mut scores = HashMap::new();
	// scores.insert(String::from("Blue"), 10);
	// scores.insert(String::from("Yellow"), 50);

	// let teams = vec![String::from("Blue"), String::from("Red")];
	// let initial_scores = vec![10, 50];
	// let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

	// let field_name = String::from("Favorite color");
	// let field_value = String::from("Blue");

	// let mut map = HashMap::new();
	// map.insert(field_name, field_value);
	// println!("{}",field_name);


	// let mut scores = HashMap::new();
	// scores.insert(String::from("Blue"), 10);
	// scores.insert(String::from("Yellow"), 50);
	// let team_name = String::from("Blue");
	// let score = scores.get(&team_name);

	// for (key, value) in &scores {
	//     println!("{}, {}",key,value);
	// }

	let mut scores = HashMap::new();
	scores.insert(String::from("Blue"), 10);

	scores.entry(String::from("Yellow")).or_insert(50);
	scores.entry(String::from("Blue")).or_insert(50);

	println!("{:?}",scores);

}
