use std::thread;
use std::time::Duration;


fn main() {
	// let x = 4;
	// let equal_to_x = |z| z == x;
	// let y = 4;
	// assert!(equal_to_x(y));

	let x = vec![1, 2, 3];
	let equal_to_x = move |z| z == x;
	println!("can't use x here: {}",x);
	let y = vec![1, 2, 3];
	assert!(equal_to_x(y));
}


fn simulated_expensive_calculation(intensity: i32) -> i32 {
    println!("calculating slowly...");
	thread::sleep(Duration::from_secs(2));
	intensity
}

fn generate_workout(intensity: i32, random_number: i32) {

	let expensive_result = simulated_expensive_calculation(intensity);

	if intensity < 23 {
	    println!(
	    	"Today, do{}",
	    	simulated_expensive_calculation(intensity)
		);
		println!(
			"Next, do {} situps",
			simulated_expensive_calculation(intensity)
		);
	} else {
	     if random_number == 3 {
	         println!("Take a break today! Remember to stay hybrated!");
	     } else {
	         println!(
	         	"Today, run for {} minutes",
	         	simulated_expensive_calculation(intensity)
	         );
	     }
	}
}