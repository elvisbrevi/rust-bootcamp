trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting object {}", color)
    }
}

struct VehiculeInfo {
    make: String,
    model: String,
    year: u16,
}

struct Car {
    info: VehiculeInfo,
}

impl Park for Car {
    fn park(&self) {
        println!("Parking car ...");
    }
}

impl Paint for Car {}

struct Truck {
    info: VehiculeInfo,
}

impl Park for Truck {
    fn park(&self) {
        println!("Parking truck ...");
    }
}

impl Paint for Truck {}

impl Truck {
    fn unload(&self) {
        println!("Unloading truck ...");
    }
}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("Painting house {}", color)
    }
}

fn main() {
    println!("Hello, world!");
}
