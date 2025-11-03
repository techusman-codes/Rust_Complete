// Data types in Rust
// Rust is a static typed languge need to know what to compile at a run time.

fn main() {
    // TYPES OF DATA TYPES
    // INTEGERS
    let small_number: u8 = 255;
    let big_number: u128 = 123456789098;
    let small_number2: i8 = -127;
    let big_number2: i128 = -123456789098765;

    println!("small_number: {}", small_number);
    println!("big_number:{}", big_number);
    println!("Small_number2: {}", small_number2);
    println!("big_number2: {}", big_number2);

    // Floating Point

    let x = 2.0; // f64 default
    let y: f32 = 3.0; //32 

    println!("x:{}, y = {}", x, y);

    // Numerical Operators
    let sum = x + y;
    let difference = x - y;
    let product = x * y;
    let quotient = x / y;
    let remainder = x % y;

    println!(
        "sum: {}, difference: {}, product: {}, quotient:{}, remainder: {}",
        sum, difference, product, quotient, remainder
    );

    // BOOLEANS TRUE/FALSE

    let t = true; // implicit decleration
    let f: bool = false; // explicit decleration

    println!("t:{}, f:{}", t, f);

    // CHARACTER A 4BYTES its used to represent a single chracter value

    // TUPLES this is used to group different number or values
}
