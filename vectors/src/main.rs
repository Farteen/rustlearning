fn main() {
    // println!("Hello, world!");
    // let v: Vec<i32> = Vec::new();


    // let v = vec![1, 2, 3];

    // let mut v = Vec::new();
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // let v = vec![1, 2, 3, 4, 5];
    // let third: &i32 = &v[2];
    // let third: Option<&i32> = v.get(2);

    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push[6];

	let row = vec![
	    SpreadsheetCell::Int(3),
	    SpreadsheetCell::Text(String::from("blue")),
	    SpreadsheetCell::Float(10.12),
	];
    // let row = vec![
	   //  SpreadsheetCell::Int(3),
	   //  SpreadsheetCell::Text(String::from("blue")).
	   //  SpreadsheetCell::Float(10.12),
    // ];

}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
