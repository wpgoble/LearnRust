fn main() {
	let three = 0b11;			// binary    
	let thirty = 0o36;			// octal
	let three_hundred = 0x12C;	// hexadecimal

	println!("base 10:\t{}\t{}\t{}", three, thirty, three_hundred);
	println!("base 2:\t\t{:b}\t{:b}\t{:b}", three, thirty, three_hundred);
	println!("base 8:\t\t{:o}\t{:o}\t{:o}", three, thirty, three_hundred);
	println!("base 16:\t{:x}\t{:x}\t{:x}", three, thirty, three_hundred);
}
