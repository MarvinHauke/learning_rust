fn main() {
    /*
        second();
        booleans();
        mean();
        arrays_and_lists();
        let celsius = 23.0;
        let recived_temp = challenge2(celsius);
        assert_eq!(recived_temp, 73.4);
        println!("your output temperature was {}", recived_temp);
        forloop_test();
    */
    challenge3();
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

/* loops can return a value*/
//fn loop_test() {
//    let value = loop{}
//}

/* while loops don´t return a value!!! */
//fn whileloop_test() {}

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
