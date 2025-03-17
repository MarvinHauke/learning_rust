fn main() {
    //second();
    //booleans();
    //mean();
    arrays_and_lists();
}

/*functions with strings and numbers.*/
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

/*First challenge*/
fn mean() {
    let a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = (a as f64 + b + c as f64) / 3.0;

    assert_eq!(average, 45.1);
    println!("Test passed! the answer is {}", average);
}

/*Here i try to use functions in Rust*/
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
    let mut stuff: (u8,f64,char)= (10, 3.13, 'x');
    let stuff.1 += 3.0; //takes the second element, adds 3 to it and brings it back into the tuple.
    let first_item = stuff.0;
    println!("first_item from the tuple is {}", first_item);

    let (a, b, c) = stuff; //this assigns values from the tuples to a, b, c
    println!("b is {}", b)
}
