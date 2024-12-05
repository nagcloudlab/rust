

#[derive(Hash, Eq, PartialEq, Debug)]
struct Car {
    name: String,
    model: String,
    year: u32,
}


pub fn print_unique_cars(){
    use std::collections::HashSet;
    let mut cars = HashSet::new();

    let car1 = Car {
        name: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2015,
    };

    let car2 = Car {
        name: String::from("Toyota"),
        model: String::from("Corolla"),
        year: 2015,
    };

    cars.insert(car1);
    cars.insert(car2);

    // len
    println!("Number of cars: {}", cars.len());

    for car in cars.iter() {
        println!("Car: {} {} {}", car.name, car.model, car.year);
    }

}