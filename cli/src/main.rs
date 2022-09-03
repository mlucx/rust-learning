// rust uses 'snake_case' as the default typing convension

// a '!' means a macro, not function.

use std::io;

fn main() {
    println!("Enter your weight (kg): ");

    let mut input = String::new(); // string is owned by 'input' variable 
    io::stdin().read_line(&mut input).unwrap();

    // unwrap terminates program if func returns an error, and passes correct value.

    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight = calculate_weight_on_mars(weight);
    println!("Weight on Mars: {}kg", mars_weight);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// 1. Each value in Rust is owned by a variable.

// 2 . When the owner goes out of scope, the value will be deallocated.

// 3. There can only be ONE owner at a time.



// references allow us to refer to a value without altering it. & before type.
// &mut = mutable reference:a