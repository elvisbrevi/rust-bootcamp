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
    let car = Car {
        info: VehiculeInfo {
            make: "Toyota".to_string(),
            model: "Camry".to_string(),
            year: 2020,
        },
    };
    let house = House {};
    let object = create_paintable_object();

    paint_red1(&car);
    paint_red1(&house);
    paint_red1(&object);

    paint_vehicule(&car);
    //paint_vehicule(&house); // error because house dont implement Park
    //paint_vehicule(&object); // error because object dont implement Park
}

// Traits bounds

fn paint_red1<T: Paint>(obj: &T) {
    obj.paint("red".to_string())
}

fn paint_red2(obj: &impl Paint) {
    obj.paint("red".to_string())
}

fn paint_red3<T>(obj: &T)
where
    T: Paint,
{
    obj.paint("red".to_string())
}

fn paint_vehicule<T>(obj: &T)
where
    T: Paint + Park,
{
    obj.paint("red".to_string())
}

fn create_paintable_object() -> impl Paint {
    House {}
}
