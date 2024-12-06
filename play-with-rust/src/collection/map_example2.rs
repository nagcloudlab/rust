// #[allow(unused_variables)]

// create map  with key as Owner and value as Car
// input key ( i.e Owner ) and get value ( i.e Car )

struct Owner {
    name: String,
    age: u32,
}

// implement Hash for Owner only based on name
impl std::hash::Hash for Owner {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

// implement Eq for Owner only based on name
impl std::cmp::Eq for Owner {}

// implement PartialEq for Owner only based on name
impl std::cmp::PartialEq for Owner {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
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
    car_map.insert(owner2, car2);

    // get car details for owner with name John

    let owner_key = Owner {
        name: String::from("John"),
        age: 0
    };

    match car_map.get(&owner_key) {
        Some(car) => println!("Car details for owner1: model: {}, year: {}", car.model, car.year),
        None => println!("Car details not found for owner1"),
    }

    

}