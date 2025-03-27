use std::io::prelude::*;
use std::{env, fs};

fn main() {
    //radding command line arguments:
    if env::args().len() <= 2 {
        println!("The Program needs at least 2 arguments");
        return;
    }

    //reading from files:
    let filepath = env::args().nth(2).unwrap();
    let file_content = fs::read_to_string(&filepath).unwrap();

    for line in file_content.lines() {
        println!("{}", line);
    }

    //if you want to read bytes for example for bin files or image use read()
    let file_content = fs::read(&filepath).unwrap(); //this is "shadowind" file_content 
    println!("the file conntent as vector is {:?}", file_content);

    //this is how you append to a file
    let mut file = fs::OpenOptions::new().append(true).open(&filepath).unwrap();
    file.write_all(b"\nappended text")
        .expect("Failed to write to file");

    for (index, argument) in env::args().enumerate() {
        println!("argument {} is {}", index, argument);
    }

    let arg2 = env::args().nth(2).unwrap();
    println!("arg2 is {}", arg2);
}
