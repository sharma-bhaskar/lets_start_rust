use std::io;
#[allow(dead_code, unused_variables)]

/// Crate comment.
/// Start learning the Rust from the begining
///   /*
// Pacakge manager is cargo
//
// 1. cargo new --- to create new project
// 2. cargo build  ---  to build the project
// 3. cargo check -- to compile the code
// 4. cargo doc -- to create the doc
// 5. cargo run - to run the program
// */
fn main() {

    // user_input();
    // print_ways();
    // variable_read();
    // check_even_odd();
    read_string();
}

fn read_string() {
    //String slice are immutable
    let cat: &str = "Fluffy";
    println!("{}", cat);
    let _dog = String::new();
    let mut dog = String::from("Max");
    println!("{}", dog);
    let owener = format!("Hi my name {} and owner of {} ","Mark",dog);
    println!( "{}",owener);
    println!( "{}",owener.len());
    dog.push(' ');
    dog.push_str("the dog");
    println!("{}",dog)
}

#[allow(dead_code)]
fn check_even_odd() {
    let number = 25;

    if number / 2 == 0 {
        println!("Even number")
    } else {
        println!("odd number")
    }
}

#[allow(dead_code)]
fn variable_read() {
    //Varibale
    // let is immutable variable
    // Rust is strongly typed language
    let _name = "Alex"; // immutable variable
    let _age = 27;
    let _amount: i64 = 672637363746;
    let _color = "Red";
    let color = "Blue"; // this is shadow variables

    println!("color {}", color);


    let (_a, _b, _c) = (23, "njn", 99);

    let million = 1_000_000;
    println!("{}", million);

    let is_day = true;
    let _is_night = false;
    println!("{}", is_day);

    let char1 = 'A';
    println!("{}", char1);
}

#[allow(dead_code)]
fn print_ways() {
    /*
   numbers of way to print
   */
    println!("Hello, world!");
    println!("My name is {} and I am {} old", { "Bhaskar" }, { 29 });
    println!("a + b = {}", 2 + 5);
    println!("{0} has a {2} and {0} has a {1}", "Alex", "cat", "dog");
    println!("{name} and {surname}", name = "Bhaskar", surname = "sharma");

    //printing traits
    println!("binary: {:b}, hex : {:x}, octal : {:o} ", 50, 50, 50);

    //Debug
    println!("Array: {:?}", [1, 2, 3]);
}

#[allow(dead_code)]
fn user_input() {
    let mut input = String::new();

    //Print a message to the user
    println!("Say something!");

    /*
    read line from the user and print it and,
     if the user didn't give anything print error
    */
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("You said {}", input)
        }
        Err(e) => {
            println!("Something is not right{}", e)
        }
    }
}
