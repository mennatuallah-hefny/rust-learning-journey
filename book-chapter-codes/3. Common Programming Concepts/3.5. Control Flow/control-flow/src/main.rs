

/*
fn main() {
    // let number = 3;
    
    let number = 3;

    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }
}
*/


/* 
/**
 * Rust will not automatically try to convert non-Boolean types to a Boolean.
 */

fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}

*/

/*
fn main() {
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }
}
*/


/*
// Handling Multiple Conditions with else if
fn main() {
    let number = 8;

    if number % 4 == 0 {
        println!("number is divisible by 4")
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("number is not divisible by 4, 3, or 2")
    }
}
*/

/*
// Using if in a let Statement
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
*/

/*
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
*/


/*
// Repetition with Loops
fn main() {
    loop {
        println!("again!");
    }
}
*/


/*
// Returning Values from Loops
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
*/


/*
// Loop Labels to Disambiguate Between Multiple Loops
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
*/


/*
// Conditional Loops with while
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
*/

/*
// Looping Through a Collection with for
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    fn main() {
        let a = [10, 20, 30, 40, 50];
    
        for element in a {
            println!("the value is: {element}");
        }
    }
    fn main() {
        for number in (1..4).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
    } 
}
*/
