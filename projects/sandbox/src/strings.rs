pub fn run() {
	let hello = "Hello"; 	// Immutable fixed length
	let world = String::from("World");

	println!("{}, {}", hello, world);
	println!("Length: {}", world.len());

	let mut name = String::from("Johnna ");
	println!("{}, {}", hello, name);
	name.push('B');
	println!("{}, {}", hello, name);
	name.push_str("arnaby!");
	println!("{}, {}", hello, name);

	println!("Capacity: {}", world.capacity());
	println!("Is Empty: {}", name.is_empty());
	println!("Contains 'Barnaby': {}", name.contains("Barnaby"));
	println!("Replace last name: {}", name.replace("Barnaby", "Goble"));
	println!("Remove '!': {}", name.replace("!", ""));
	name = name.replace("!", "")
				.replace("Barnaby", "Goble");
	// Loop through string by whitespace
	for word in name.split_whitespace() {
		println!("{}", word);
	}

	let mut s = String::with_capacity(10);
	s.push('a');
	s.push('b');

	assert_eq!(2, s.len());
	assert_eq!(10, s.capacity());

	println!("{}", s);
}