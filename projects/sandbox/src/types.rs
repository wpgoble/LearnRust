// Primitives Types--
// Integers: u/i8, u/i16, u/i32, u/i64, u/i128
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays


// Rust can infer what type our variable is
pub fn run()
{
	let x = 1; 		// Default is i32
	let y = 2.5;		// Default is f64

	let z: i64 = 454545454545;
	println!("Max i32: {}", std::i32::MAX);
	println!("Max i64: {}", std::i64::MAX);

	let is_active: bool = true;
	let is_greater: bool = 10 > 5;

	println!("{:?}", (x, y, z, is_active, is_greater));

	let a1 = 'a';
	let face = '\u{1F600}';		
	println!("a1: {}, face: {}", a1, face);
}