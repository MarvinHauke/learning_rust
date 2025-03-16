fn main() {
    second();
}

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
}
