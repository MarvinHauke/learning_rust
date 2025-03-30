#[derive(Debug)]
//Generics are a zero cost abstraction this means generics dont creapte overhead. its translated
//into different types at complie time.
struct Rectangle<T, U> {
    width: T,
    height: U, //U is just the next letter in the alphabet after T. Add other Variables for different
               //Data types you want to use in your generic Object
}

impl<T, U> Rectangle<T, U> {
    fn get_width(&self) -> &T {
        &self.width
    }
}

//Implement methods for specific types (in this case width and height as u8)
impl Rectangle<u8, u8> {
    fn get_perimeter(&self) -> u8 {
        2 * self.width + 2 * self.height
    }
}

fn get_biggest<T: PartialOrd>(a: T, b: T) -> T {
    // PartialOrd limits the datatypes, T can become. in this cas only numeric values
    if a > b { a } else { b }
}

//Store Data on the heap instead of the stack with Box<T>
//Boxes have ownership about theier data, they point to. They are smart pointers

use std::{f64::consts::PI, mem};
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}

fn sum_boxes<T: std::ops::Add<Output = T>>(a: Box<T>, b: Box<T>) -> Box<T> {
    Box::new(*a + *b)
}
fn main() {
    let vehicle = Shuttle {
        name: String::from("Atlantis"),
        crew_size: 7,
        propellant: 835958.0,
    };
    println!(
        "vehicle size on stack: {} bytes",
        mem::size_of_val(&vehicle)
    );

    let boxed_vehicle: Box<Shuttle> = Box::new(vehicle); // this is not a copy operation. The box
    // will become the new owner of the Strct.

    println!(
        "boxed_vehicle size on stack: {} bytes",
        mem::size_of_val(&boxed_vehicle)
    );
    println!(
        "boxed_vehicle size on heap: {} bytes",
        mem::size_of_val(&*boxed_vehicle)
    );

    let unboxed_vehicle: Shuttle = *boxed_vehicle; //unboxing vehicle by de referencing it and bringing it back to the stack

    // Old function calls
    let rect = Rectangle {
        width: 1u8,
        height: 3u8,
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());
    println!("biggest is {}", get_biggest(1.2, 2.3));

    //new tests for challenge
    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(one, two), 3);

    let pi = Box::new(std::f64::consts::PI);
    let e = Box::new(std::f64::consts::E);
    assert_eq!(*sum_boxes(pi, e), 5.859874482048838);

    println!("all test passed!! 🥳")
}
