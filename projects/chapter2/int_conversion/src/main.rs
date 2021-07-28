use std::convert::TryInto;

fn main() {
	let a: i32 = 10;
	let b: u16 = 100;

	// try_into returns a Result type that provides access to the 
	// conversion attempt
	let b_ = b.try_into()
				.unwrap();

	if a < b_ {
		println!("Ten is still less than one hundred!");
	}
}
