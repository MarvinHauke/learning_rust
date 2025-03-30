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

fn main() {
    let rect = Rectangle {
        width: 1u8,
        height: 3u8,
    };
    println!("rect is {:?}", rect);
    println!("width is {}", rect.get_width());
    println!("perimeter is {}", rect.get_perimeter());
    println!("biggest is {}", get_biggest(1.2, 2.3))
}
