fn main() {
	let a: i32 = 10;
	let b: u16 = 100;

	if a < b.into() {
		println!("Ten is less than one hundred!");
	}

	if a < (b as i32) {
		println!("Okay, now ten is less than one hundred!");
	}
}
