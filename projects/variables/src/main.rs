
const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    println!("The max number of points: {}", MAX_POINTS);

    let y = 5;
    println!("First value of y: {}", y);

    // Here we shadow the first value of y
    let y = y + 1;
    let y = y * 2;
    println!("The shadowed value of y: {}", y);

    // shadowing allows us to change the type 
    // of variable in Rust
    let spaces = "        ";
    let spaces = spaces.len();
    println!("New value of spaces: {}", spaces);

    // in the above example we transform spaces 
    // from a string to a number
}
