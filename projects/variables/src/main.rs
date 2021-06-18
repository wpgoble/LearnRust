
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

    // variable types:
    // Scalar types"
    //      Integer Types: a number without a fractional component.
    //          signed:   i8, i16, i32, i64, i128, isize
    //          unsigned: u8, u16, u32, u64, u128, usize
    //      Floating Point: 
    let x = 2.0;        // default is f64
    let y: f32 = 3.0;   // f32

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    //      Booleans: 
    let t = true;
    let f: bool = false;
    //      Characters

    // Compound types
    // Tuple: A general way of grouping together
    // a number of values with a variety of types 
    // into one type

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x: {}, y: {}, z: {}", x, y, z);
}
