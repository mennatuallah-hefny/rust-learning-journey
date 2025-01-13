/**
 * Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
 * 
 * Parameters => are special variables that are part of a function’s signature.
 * 
 * Arguments => the concrete values passed to a function when it is called.
 *
 * Expression-based language => a programming language where nearly everything is treated as an expression that evaluates to a value, as opposed to being a statement. This approach makes the language more consistent and often leads to a more functional or declarative programming style.
 * 
 * Statements => are instructions that perform some action and do not return a value.
 * 
 * Expressions => evaluate to a resultant value. do not include ending semicolons.
 * 
 * 
*/


/*
fn main() {
    println!("Hello, world!");

    another_function()
}

fn another_function() {
    println!("Another function.")
}
*/


/*
// Parameters
fn main() {
    another_function(5);
}

fn another_function(x:i32) {
    println!("The value of X is: {x}");
}
*/


/*
// Multiple parameters
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value:i32, unit_label:char) {
    println!("The measure is: {value}{unit_label}")
}
*/


/*
// Statements and Expressions
/*
fn main() {
    let y = 6;
}
*/

/*
fn main() {
    let x = (let y = 6);
} 
*/

/*
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value y is: {y}");
}
*/


*/


/*
// Functions with Return Values
/*
fn five() -> i32 {
    5
}


fn main() {
    let x = five();

    println!("The value of x is: {x}");
}
*/

/*
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}



fn plus_one(x: i32) -> i32 {
    x + 1
}
*/

/*
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
*/    
*/