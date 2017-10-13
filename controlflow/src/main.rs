extern crate rand;
use rand::Rng;
// fn main() {
// 	let number = 3;
// 	if number < 5 {
// 		println!("condition was true");
// 	} else {
// 	    println!("condition");
// 	}
// }


// fn main() {
//     let number = 3;
//     if number != 0 {
//         println!("number was something other than zero");
//     }
// }

fn main() {

	// let a = [10, 20, 30, 40, 50];
	// for element in a.iter() {
	//     println!("the value is: {}", element);
	// }

	// for number in (1..4).rev() {
	//     println!("{}", number);
	// }
	// let k_temp = rand::thread_rng().gen_range(1.0, 1000.0);

	// let k_to_c_result = k_to_c(k_temp);
	// println!("k {} to c result is {}", k_temp, k_to_c_result);

	let fabonacci_index = 2;
	println!("The index {} fabonacci is {}",fabonacci_index,fabonacci(fabonacci_index));
	
	// let a = [10, 20, 30, 40, 50];
	// let mut index = 0;
	// while index < 5 {
	//     println!("The value is: {}",a[index]);
	//     index = index + 1;
	// }

	// let condition = true;

	// let number = if condition {
	//     5
	// } else {
	//     // "six"
	//     6
	// };
	// // let number = if condition {

	// println!("The value of number is: {}",number);
	//     5
	// } else {
	//     6
	// };

	// println!("The value of number is: {}",number);
    // let number = 6;

   //  if number % 4 == 0 {
   //      println!("number is divisible by 4");
   //  } else if number % 3 == 0 {
 		// println!("number is divisible by 3");
   //  } else if number % 2 == 0 {
   //  	println!("number is divisible by 2");
   //  } else {
   //      println!("number is not divisible by 4, 3, or 2");
   //  }
}

fn fabonacci(n: i64) -> i64 {
    if n == 1 {
        return 1;
    } else {
        return n + fabonacci(n - 1);
    }
}

fn k_to_c(k: f64) -> f64 {
	return (k - 273.15) * 9.0 / 5.0 + 32.0;
}

fn f_to_c(f: f64) -> f64 {
    return (f - 32.0) * 5.0 / 9.0;
}

fn c_to_k(c: f64) -> f64 {
    return (c + 273.15);
}