fn main() {
    println!("Hello, world!");
}

// Euclid's algorithm
fn gcd(mut n: u64, mut m: u64) -> u64 {
	// the assert macro verifies that neither argument is zero.
	// the ! character marks this as a marco invocation, not a 
	// function call. If this returns false it will terminate the program
	assert!(n != 0 && m != 0);
	while m != 0 {
		if m < n {
			let t = m;
			m = n;
			n = t;
		}
		m = m % n;
	}
	// Rust does have a return statement, but if a function 
	// body ends with an expression that is NOT followed by 
	// a semicolon, that is the function's return value. 
	// In Rust we use the return statement for early function 
	// exit, and for the final exiting of the function we just 
	// ket the statement fall off.
	n
}

#[test]
fn test_gcd() {
	assert_eq!(gcd(14, 15), 1);
	
	assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

