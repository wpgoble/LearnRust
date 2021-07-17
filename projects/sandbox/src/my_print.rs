// This is a public function that main will use
pub fn run() {
	println!("Hello from my_print!");
	
	// To print ints, floats, etc.
	println!("Number: {}", 69);

	// To have multiple place holders
	println!("{} is from {}", "William", "FL");

	// positional arguments
	println!("{0} is from {1} and {0} likes to {2}", "William", "FL", "code");

	// Named Arguments
	println!("{name} likes to read {genre} books", 
			name = "Johnna", 
			genre = "Christian Romance"
	);

	// placeholder traits
	println!("Binary: {:b} Hex: {:x}, Octal: {:o}", 10, 10, 10);

	// Placeholder for debug trait
	println!("{:?}", (12, true, "Hello"));

	// basic math
	println!("10 + 10 = {}", 10 + 10);
}
