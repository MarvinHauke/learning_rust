use rand::prelude::*; // the "prelude imports the most important functions from rand"
use std::io;

fn main() {
    challenge5();
    /* all old challenges
        random_number();
        modules();
        second();
        booleans();
        mean();
        arrays_and_lists();
        let celsius = 23.0;
        let recived_temp = challenge2(celsius);
        assert_eq!(recived_temp, 73.4);
        println!("your output temperature was {}", recived_temp);
        forloop_test();
        challenge4();
    */
}

/* Challenge 5 */
// 1. The Program generates a random number between 1 and 100
// 2. the user tries to guess the secret number.
// 3. The Program tells the user if their guess was too high, too low, or correct.
// 4. Repeat steps 2 and 3 until the user guesses correctly

fn challenge5() {
    /*this was my solution:
        let number = rand::random_range(1..101);
        let mut input = String::new();
        loop {
            println!("please enter a number: ");
            io::stdin().read_line(&mut input); // <-- "let _" is for throwing away the result.
            let guessed_number = input.trim().parse::<u32>().unwrap();
            if number == guessed_number {
                println!("Bravo you guessed corret! 🥳");
                break;
            } else if guessed_number > number {
                println!("Your guess was too big");
            } else if guessed_number < number {
                println!("Your guess was too small");
            }
            input.clear();
        }
        println!("the number of the computer is {number} the input was {input}");
    */
    //Course solution:

    let secret_number = rand::random_range(1..101);
    println!("I'm thinking of a number between 1 an 100...");
    println!("Guess the number:");
    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input line."); // expect() is simular too unwrap()
        let guess: u32 = guess.trim().parse().expect("Failed to parse the guess.");

        if guess > secret_number {
            println!("\n{} is too high! Guess lower:", guess);
        } else if guess < secret_number {
            println!("\n{} is too low! Guess higher:", guess);
        } else {
            println!("\nYou got it! The secret number was {}.", guess);
            break;
        }
    }
}

/* Modules */
fn random_number() {
    let number = rand::random::<f64>(); // <-- this uses the turbofish operator "::<f64>"
    println!("number is {}", number);
}

fn modules() {
    let mut buffer = String::new();
    println!("Ener a message:");
    io::stdin().read_line(&mut buffer);
    println!("buffer is {}", buffer);

    let number_turbofish = buffer.trim().parse::<i32>(); // .trim() is needed because of the \n after the input and before .parse()
    // the parse::<i32>(); has two parts the parse() and the "turbofish operator" ::<i32>
    let number: i32 = buffer.trim().parse().unwrap(); // a different way would be to define number as i32
    // both result in a "result enum" which can contain the questined value from input or an error type which has to be unwraped
    println!("this is the result from parse() {:?}", number);
    println!("number_turbofish + 1 is {}", number_turbofish.unwrap() + 1);
    println!("number + 1 is {}", number + 1);
}

/*functions with strings and numbers */
fn second() {
    let mut x: u8 = 245;
    println!("x is {}", x);
    // x += 20; // this produces a runtime error because of overfolw of x u8 is restricted to 255 = 8bit
    println!("x is {} now, x has to be 'mut' for this", x);

    let mut y: f32 = 255.5;
    println!("y is {}", y);
    y = 234.324_52;
    println!("y is {} now, y is a 32bit float number", y);

    // lets cast some values!
    let c = x as f32 / y;
    println!("c is {} and its x/y. For this x has to be casted to f32", c);
    // this is how a print can be formatted
    // check out rust docs for std::format module
    println!(
        "c is {:.3} this shows float formatting for 3 decimal places",
        c
    );

    let sign = "\u{261D}";
    println!("this is a unicode sign {}", sign);
}

/*functions for logic operations */
fn booleans() {
    //let letter = 'a';
    //let letter = '1';
}

/*First challenge */
fn mean() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed! the answer is {}", average);
}

/*Here i try to use functions in Rust */
fn arrays_and_lists() {
    let parking_lot = [[1, 2, 3], [4, 5, 6]];
    let number = parking_lot[0][1];
    println!(
        "a Parking lot on the place {} aka multidimensional Array",
        number
    );

    //tree dimensional array
    let garage = [[[0; 10]; 10]; 5];
    println!(
        "a 10x10x5 Array filled with 0s. This is position[0][0][3] {}",
        garage[0][0][3]
    );

    //tuples multiple items of mixed datatypes
    //elements are ordered
    //stored in a fixed-length, contiguous section memory
    let mut stuff: (u8, f64, char) = (10, 3.13, 'x');
    let first_item = stuff.0;
    println!(
        "first_item from the tuple is: {}, the second item is: {} and the third is: {}",
        first_item, stuff.1, stuff.2
    );
    stuff.1 += 3.0; //takes the second element, adds 3 to it and brings it back into the tuple.

    let (a, b, c) = stuff; //this assigns values from the tuples to a, b, c
    println!(
        " a = {}, b is now the element from stuff.1 + 3 = {} and c = {}",
        a, b, c
    )
}

/* convert from Celsius to Fahrenheit */
fn challenge2(celsius: f64) -> f64 {
    let fahrenheit = (celsius * 1.8) + 32.0;
    println!(
        "recived Temperature in C° is {}. The returned temperature in F° is {}",
        celsius, fahrenheit
    );
    fahrenheit
}

/* loops */
fn loops() {
    /* loops can return a value*/
    //fn loop_test() {
    //    let value = loop{}
    //}

    /* while loops don´t return a value!!! */
    //fn whileloop_test() {}
}

/* For loops are not that simple in rust. However in patch 1.53 there was a change which made it easier*/
fn forloop_test() {
    let message = ['h', 'e', 'l', 'l', 'o'];

    //e has is only a `&char which means in a for loop its only a reference to the array. (aka
    //pointer to the value) You cant simply check for 'e' it has to be dereferenced!
    for (index, &item) in message.iter().enumerate() {
        println!("item {} is {}", index, item);
        if item == 'e' {
            break;
        }
    }

    //this is a range:
    for number in 0..5 {
        println!("number is {}", number);
    }
}

/* Challenge 3 */
fn challenge3() {
    let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
    let mut max: i32;
    let mut min: i32;
    let mut mean: f64;

    min = numbers[0];
    max = numbers[0];
    mean = 0.0;
    for number in numbers {
        if number < min {
            min = number;
        } else if number > max {
            max = number;
        }
        mean += number as f64;
    }
    mean /= numbers.len() as f64;

    assert_eq!(max, 56);
    assert_eq!(min, -18);
    assert_eq!(mean, 12.5);
    println!("Test passed!");
}

/* Ownership*/
fn ownership() {
    /* Ownership:
        Ownership is a conecept which builds around the Stack and the Heap.
        While the Stack needs fixed sizes of variables, the heap can handle modifications of varaalble sizes.
        It is also important to know that pointers are always saved on the Stack.
        In Rust a chunk of memory at the Heap can only have one "onwner" (pointer). The compiler checks if ownership was transfered and if so he complains.
        To go around this problem you have to make sure, that only one "owner" is capable of reaching out to the heap allocated memory at anytime.
        You can do so by cloning the memory chunk at the heap, or by returning values from functions and "shadowing" its initial Variable.

        This is what Chat-GPT says about my explanation:
        1. Stack vs. Heap:
            You correctly mention that the Stack requires fixed-size variables, while the Heap can handle dynamic allocations.
            However, it's important to clarify that not all heap-allocated data has a single owner—some abstractions like Rc<T> allow shared ownership.
            Pointers themselves (e.g., references) are stored on the Stack, but they can point to data on the Heap.

        2. Ownership in Rust:
            A heap-allocated chunk of memory (e.g., a String or Vec<T>) has exactly one owner at a time.
            If ownership is transferred (moved), the original variable can no longer be used, but the compiler does not "complain"—it enforces this rule by preventing access to the old variable.
            To allow multiple access patterns, Rust provides borrowing (&T for immutable, &mut T for mutable references), rather than cloning.

        3. Avoiding Move Semantics:
            Cloning (.clone()) creates a new independent copy of the data, which avoids moves but has a performance cost.
            Returning values from functions moves ownership but does not necessarily involve shadowing. Shadowing is when you reassign a variable name to a new value, possibly consuming the old one.

        Suggested Correction:
            "Ownership in Rust is a concept that ensures memory safety without a garbage collector.
        Values stored on the Heap have a single owner at any given time, and ownership can be transferred (moved) to another variable. 
        The compiler enforces these rules to prevent data races and dangling references. 
        To allow multiple accesses, Rust provides borrowing (&T for immutable, &mut T for mutable references). Cloning (.clone()) 
        creates a new independent copy, while shadowing or function returns can also be used to manage ownership."


        Takeaways:
            There are several different versions to refere to a variable.
            - handing over ownership. -> Then you have to take care when the variable goes "out of scope"
            - borrowing or "call by reference", this leaves the ownership at the initial variable or reference -> create as many immutable references as you want
            - borrowing as mutable, but this prefents other references (also immutables!) to that variable. -> prevents data races
    */
}

/* Borrowing:*/
fn challenge4() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There is space in front.");
    assert_eq!(trim_spaces(&test2), "There is space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = " We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " 🚀";
    assert_eq!(trim_spaces(test7), "🚀");

    println!("All test passed! 🥳")
}

fn trim_spaces(s: &str) -> &str {
    //remove all leading and trailing spaces from a string but only space characters NOT \n or \t
    //take a &String or &str[] as input from challenge4()
    //return &str[]

    //locate the first non-space character
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    //search in reverse to locate the last non-space character
    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }
    &s[start..end]
}
