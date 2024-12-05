
// create map  with key as Owner and value as Car
// input key ( i.e Owner ) and get value ( i.e Car )


#[derive(Eq, PartialEq,Hash)]
struct Owner {
    name: String,
    age: u32,
}
struct Car {
    model: String,
    year: u32,
}

pub fn get_car_details(){

    let mut car_map = std::collections::HashMap::new();

    let owner1 = Owner {
        name: String::from("John"),
        age: 30,
    };
    let car1 = Car {
        model: String::from("Toyota"),
        year: 2010,
    };
    let owner2 = Owner {
        name: String::from("Jane"),
        age: 25,
    };
    let car2 = Car {
        model: String::from("Honda"),
        year: 2015,
    };

    car_map.insert(owner1, car1);

    // get car details for owner1

    let owner_key = Owner {
        name: String::from("John"),
        age: 30,
    };

    match car_map.get(&owner_key) {
        Some(car) => println!("Car details for owner1: model: {}, year: {}", car.model, car.year),
        None => println!("Car details not found for owner1"),
    }

}