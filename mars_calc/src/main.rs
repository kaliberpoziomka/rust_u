// importing modules and funcitons from them
use std::io;

// main function is allways running first in program (is the entry to rust program)
fn main() {
    println!("Enter your weight (kg):");
    /* 
    & Three OWNERSHIP rules:
     * 1. Each value in Rust is owned by a variable
     * 2. When the owner goes out of scope, the value will be deallocated
     * 3. There can only ba ONE owner at a time
     */
    // "input" variable is the owner of our empty string (string lives on a heap, because its size is not known at compile time). 
    // When owner ("input") goes out of scope, the string will be disallocated
    let mut input = String::new();
    // code below just to show that we can borrow multiple times immutable value, and then borro mutable value
    let s1 = &input;
    let s2 = &input;
    println!("{} {}", s1, s2);

    // we can't just pass a value, because then we would move the owner, but we can BORROW value
    //& "&" means, that we expexct a reference of a value, so we BORROW a value
    //^ "&mut" means that we BORROW a value and we can CHANGE it (its mutable)
    // here we change the input variable by filling it with the text from the console
    io::stdin().read_line(&mut input).unwrap(); // we unwrap Result object here. If the result was error - it would terminate, if success it would yield
    //* We can have ONE MUTABLE borrow, and MULTIPLE IMMUTABLE borrows in the same scope.
    //* But variable which borrowed immutable, cannot be used after mutable borrowing occured.
    // so we cannot have this print with s1, s2 after &mut:
    //* println!("{} {}", s1, s2);  <-- error
    //* println!("Input {}", &input); <-- but this works, because of different scope

    let weight: f32 = input.trim().parse().unwrap();
    // dbg!(weight); // this is debugging

    // & testing with debugger
    // borrow_value(&input);
    // own_value(input);

    // we do need to specify the type of variable, because it infers the type from the function signature
    // ALL VARIABLES in rust are immutable by default, we can change this by "mut" keyword
    let mut mars_weight = calculate_weight_on_mars(weight);
    // in grams (just to show immutability)

    // println! it is a macro, because it has "!"
    // macros - are for metaprogramming (code that writes more code)
    // macros can have variable number of arguments with different types
    println!("Weight on Mars: {}kg", mars_weight);
}

// snake_case convencion in rust is demanded

// the last expression which do not have semicolon will be returned (withhout return word)
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// fn borrow_value(_s: &String) {}

// fn own_value(_s: String) {}