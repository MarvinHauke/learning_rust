//this is about traits in rust.
//they are simular to interfaces in c++ but they have some differences
use std::fmt;

struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "a Sattellite the name is: '{}',\nand it's velocity is: '{}'\n",
            self.name, self.velocity
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    println!("hubble is {}", hubble);
}
