struct Rectangle {
    size_a: f64,
    size_b: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.size_a * self.size_b
    }

    fn scale(&mut self, scaler: f64) {
        self.size_a *= scaler;
        self.size_b *= scaler;
    }

    fn new(size_a: f64, size_b: f64) -> Rectangle {
        Rectangle { size_a, size_b }
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Test passed!");
}

//#[derive(Debug)]
//struct Shuttle {
//    name: String,
//    crew_size: u8,
//    propellant: f64,
//}

//this is a method
//impl Shuttle {
//    fn get_name(&self) -> &str {
//        &self.name
//    }
//
//    fn add_fuel(&mut self, gallons: f64) {
//        self.propellant += gallons;
//    }
//
//    //This is a constructor
//    fn new(name: &str) -> Shuttle {
//        Shuttle {
//            name: String::from(name),
//            crew_size: 7,
//            propellant: 0.0,
//        }
//    }
//}

fn old_main() {
    //call the constructor to instanziate:
    //let mut vehicle = shuttle::new("endeavour2");

    //instanziate a vehicle
    //let mut vehicle = shuttle {
    //    name: string::from("endeavour"),
    //    crew_size: 7,
    //    propellant: 835958.0,
    //};
    //println!("propellant is {}", vehicle.propellant);
    //vehicle.add_fuel(1000.0);
    //println!("propellant is {}", vehicle.propellant);

    /* first challenge changinge struct values and populating fields */
    //let vehicle2 = shuttle {
    //    name: string::from("discovery"),
    //    ..vehicle //this initializes the other fields with the values from vehicle ( you cant do
    //              //that with strings natively! for that you have to .clone() the values )
    //};
    //
    //vehicle.crew_size = 6;
    ////rename the vehicle
    //vehicle.name = String::from("Atlantis");
    //println!("vehicle is {:?}", vehicle);
    //println!("vehicle2 is {:?}", vehicle2);
}
