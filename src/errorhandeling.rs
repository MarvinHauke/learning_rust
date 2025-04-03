//This is about lifetimes in rust.
//Lifetime Elision Rules:
// 1. Each input prameter that is a reference is assigned its own lifetime
//      fn foo <'a>(x:&'a str, y:i32) -> & str{}
// 2. If there is exactly one input lifetime, assign it to all output lifetimes
//      fn foo <'a>(x:&'a str) -> &'a str{}
// 3. if there is a &self or &mut self input parameter, its lifetime will be assigned to all output
//    lifetimes.

use std::io;

fn main() {
    let number = rand::random_range(1..101);
    let mut input = String::new();

    loop {
        println!("Please enter a number: ");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(guessed_number) => match guessed_number.cmp(&number) {
                std::cmp::Ordering::Equal => {
                    println!("Bravo, you guessed correct! 🥳");
                    break;
                }
                std::cmp::Ordering::Greater => println!("Your guess was too big"),
                std::cmp::Ordering::Less => println!("Your guess was too small"),
            },
            Err(_) => println!("Invalid input! Please enter a valid number."),
        }

        input.clear();
    }

    println!("The number of the computer was {number}");
}
