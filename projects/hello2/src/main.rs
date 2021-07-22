fn main() {
    greet_world()
}

fn greet_world() {
	println!("Hello, world!");
	let southern_germany = "Grüß Gott!";
	let japan = "こんにちは世界";
	let regions = [southern_germany, japan];

	for region in regions.iter() { 
		println!("{}", &region);
	}
}