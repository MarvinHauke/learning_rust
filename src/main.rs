fn main() {
    let mut astronauts: Vec<String> = Vec::new();
    astronauts.push(String::from("Shepard"));
    astronauts.push(String::from("Grissom"));
    astronauts.push(String::from("Glenn"));
    println!("astronaut is {:?}", astronauts);

    //removing the last astronaut
    let last = astronauts.pop();
    println!("last is {:?}", last);

    //print third astronaut this is not reachable anymore because we "popped" it
    //let third = &astronauts[2];
    //this is a safer variant to call it. ther will be a "None" value returned
    let third = astronauts.get(2);
    println!("third is {:?}", third);

    //this is the vecmacro for prepoulating a new vector with values
    //datatypes are also not needed to specify
    let countdown = vec![5, 4, 3, 2, 1];
}
