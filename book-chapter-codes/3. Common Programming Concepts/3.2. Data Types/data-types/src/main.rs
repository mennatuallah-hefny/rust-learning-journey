/*
fn main() {
    let x = 2.0;

    let y: f32 = 2.0;
}
*/


/*
// Numeric Operations
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
*/


/*
// The Boolean Type
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
*/


/*
// The Character Type
fn main() {
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    println!("heart_eyed_cat: {}", heart_eyed_cat)
}
*/


// Compound Types
/* 
// 1. The Tuple Type
fn main() {
    let tpl: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tpl;

    println!("The value of y is: {}", y);

    let five_hundred = tpl.0;

    let six_point_four = tpl.1;

    let one = tpl.2;

    println!("five_hundred -> {}", five_hundred);

    println!("six_point_four -> {}", six_point_four);

    println!("one -> {}", one)
}
*/

/*
// 2. The Array Type
fn main() {
    let array = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // let a = [3; 5];

    let first = a[0];

    println!("First element in a: {}", first);
    
    let third = a[2];

    println!("Third element in a: {}", third);

    
}
*/


// Invalid Array Element Access
use std::io;
use rand::Rng;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}