// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
	let name = "William";
	let mut age = 27;

	println!("My name is {} and I'm {}", name, age);

	age = 28;

	println!("Now I'm {}", age);

	// Define constant
	const ID: i32 = 001;
	println!("ID: {}", ID);

	// assign multiple vars at once
	let (her_name, her_age) = ("Johnna", 29);
	println!("{} is {} years old", her_name, her_age);

}
