use std::io::{ self, Write };

fn main() {
	print!("Input number: ");
	io::stdout().flush().unwrap();
	let mut degrees = String::new();

	io::stdin()
		.read_line(&mut degrees)
		.expect("Failed to read line!");

	let degrees: f64 = degrees.trim().parse().expect("Input a number!");
	
	let converted_ff = (degrees - 32.0) * 0.5556;
	let converted_fc = (degrees * 1.8) + 32.0;

	println!("");

	println!("{}F converted to Celsius is: {}C", degrees, converted_ff);
	println!("{}C converted to Fahrenheit is: {}F", degrees, converted_fc);
}
