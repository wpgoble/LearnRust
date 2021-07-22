use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main() {
	let a = 10;							// normal int on the stack
	let b = Box::new(20);				// int on the heap, aka a boxed int
	let c = Rc::new(Box::new(30));		// Box int wrapped within a reference counter
	let d = Arc::new(Mutex::new(40));	// int wrapped in an atomic ref ctr and 
										// protected by a mutual exlusion lock
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
