
#[allow(dead_code)]

trait Park {
    fn park(&self);
}

trait Paint {
    fn paint(&self, color: String) {
        println!("Painting {color}!");
    }
}

struct VehicleInfo {
    make: String,
    modle: String,
    year: u16,
}

struct Car {
    info: VehicleInfo,
}

impl Park for Car {
    fn park(&self) {
        println!("parking car")
    }
}

impl Paint for Car {}



struct Truck {
    info: VehicleInfo,
}

impl Truck {
    fn unlock(&self) {
        println!("Truck unlock");
    }
}

impl Park for Truck {
    fn park(&self) {
        println!("parking truck");
    }
}

impl Paint for Truck {}

struct House {}

impl Paint for House {
    fn paint(&self, color: String) {
        println!("paint house with {color}")
    }
}

fn main() {
    let house = House {};
    house.paint("blue".to_owned());

    let car = Car {
        info: VehicleInfo {
            make: "Ford".to_owned(),
            modle: "Mustang".to_owned(),
            year: 2020,
        },
    };
    car.park();
    car.paint("blue".to_owned());

    let truck = Truck {
        info: VehicleInfo {
            make: "Ford".to_owned(),
            modle: "Mustang".to_owned(),
            year: 2020,
        },
    };
    truck.unlock();
    truck.park();
    truck.paint("green".to_owned());

    let object = create_paintable_obkect();

    paint_red1(&car);
    paint_red1(&house);
    paint_red1(&object);

    paint_vehicle_red(&car);
}

fn paint_red1<T: Paint>(object: &T) {
    object.paint("red".to_owned());
}

fn paint_red2(object: &impl Paint) {
    object.paint("red".to_owned());
}

fn paint_red3<T>(object: &T) where T: Paint {
    object.paint("red".to_owned());
}

fn paint_vehicle_red<T>(object: &T) where T: Paint + Park{
    object.paint("red".to_owned());
}

fn create_paintable_obkect() -> impl Paint {
    House {}
}